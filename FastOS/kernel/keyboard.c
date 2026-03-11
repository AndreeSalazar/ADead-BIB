/*
 * kernel/keyboard.c — FastOS PS/2 Keyboard Driver
 * FastOS v2.0
 *
 * Driver PS/2 completo para QEMU y hardware real.
 * Scancode Set 1 (AT-compatible) — el default de QEMU.
 *
 * Arquitectura:
 *   IRQ1 (INT 0x21) → keyboard_irq_handler() → buffer circular
 *   shell_readline() → keyboard_getchar() → lee del buffer
 *
 * Sin libc. Sin scanf. Solo ports I/O x86-64.
 *
 * Compilar: adb step kernel/keyboard.c
 */

#include "../include/kernel.h"

/* ── Puertos PS/2 ───────────────────────────────────────── */
#define KBD_DATA_PORT   0x60
#define KBD_STATUS_PORT 0x64
#define KBD_CMD_PORT    0x64

/* Status bits */
#define KBD_STATUS_OUTPUT_FULL  0x01  /* Hay un byte listo para leer */
#define KBD_STATUS_INPUT_FULL   0x02  /* Buffer de entrada lleno (no escribir) */

/* PIC (Programmable Interrupt Controller) */
#define PIC1_CMD    0x20
#define PIC1_DATA   0x21
#define PIC_EOI     0x20   /* End-Of-Interrupt */

/* ── Buffer Circular de Teclado ─────────────────────────── */
#define KBD_BUF_SIZE 256

static char  kbd_buf[KBD_BUF_SIZE];
static int   kbd_head = 0;   /* posición de lectura */
static int   kbd_tail = 0;   /* posición de escritura */
static int   kbd_ready = 0;  /* inicializado? */

/* ── Scancode Set 1 → ASCII ─────────────────────────────── */
/* Teclas normales (sin shift) */
static const char kbd_scancode_normal[128] = {
    0,    27,  '1', '2', '3', '4', '5', '6',   /* 0x00–0x07 */
    '7',  '8', '9', '0', '-', '=',  8,    9,   /* 0x08–0x0F  (8=BS, 9=TAB) */
    'q',  'w', 'e', 'r', 't', 'y', 'u', 'i',   /* 0x10–0x17 */
    'o',  'p', '[', ']', '\n',  0,  'a', 's',   /* 0x18–0x1F  (0=LCTRL) */
    'd',  'f', 'g', 'h', 'j', 'k', 'l', ';',   /* 0x20–0x27 */
    '\'', '`',  0,  '\\','z', 'x', 'c', 'v',   /* 0x28–0x2F  (0=LSHIFT) */
    'b',  'n', 'm', ',', '.', '/',  0,   '*',   /* 0x30–0x37  (0=RSHIFT) */
    0,    ' ',  0,   0,   0,   0,   0,   0,    /* 0x38–0x3F  (SPC, F1-F6) */
    0,    0,    0,   0,   0,   0,   0,  '7',   /* 0x40–0x47  (F7-F12, KP7) */
    '8',  '9', '-', '4', '5', '6', '+', '1',   /* 0x48–0x4F  (numpad) */
    '2',  '3', '0', '.',  0,   0,   0,   0,    /* 0x50–0x57 */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x58–0x5F */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x60–0x67 */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x68–0x6F */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x70–0x77 */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x78–0x7F */
};

/* Teclas con Shift activo */
static const char kbd_scancode_shift[128] = {
    0,    27,  '!', '@', '#', '$', '%', '^',   /* 0x00–0x07 */
    '&',  '*', '(', ')', '_', '+',  8,    9,   /* 0x08–0x0F */
    'Q',  'W', 'E', 'R', 'T', 'Y', 'U', 'I',   /* 0x10–0x17 */
    'O',  'P', '{', '}', '\n',  0,  'A', 'S',   /* 0x18–0x1F */
    'D',  'F', 'G', 'H', 'J', 'K', 'L', ':',   /* 0x20–0x27 */
    '"',  '~',  0,  '|', 'Z', 'X', 'C', 'V',   /* 0x28–0x2F */
    'B',  'N', 'M', '<', '>', '?',  0,   '*',   /* 0x30–0x37 */
    0,    ' ',  0,   0,   0,   0,   0,   0,    /* 0x38–0x3F */
    0,    0,    0,   0,   0,   0,   0,  '7',   /* 0x40–0x47 */
    '8',  '9', '-', '4', '5', '6', '+', '1',   /* 0x48–0x4F */
    '2',  '3', '0', '.',  0,   0,   0,   0,    /* 0x50–0x57 */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x58–0x5F */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x60–0x67 */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x68–0x6F */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x70–0x77 */
    0,    0,   0,   0,   0,   0,   0,   0,    /* 0x78–0x7F */
};

/* Especiales */
#define KBD_SC_LSHIFT 0x2A
#define KBD_SC_RSHIFT 0x36
#define KBD_SC_LSHIFT_REL 0xAA
#define KBD_SC_RSHIFT_REL 0xB6

static int kbd_shift_active = 0;

/* ── Inline port I/O (sin includes extra) ─────────────────── */
static inline unsigned char _inb(unsigned short port) {
    unsigned char val;
    asm volatile("inb %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}

static inline void _outb(unsigned short port, unsigned char val) {
    asm volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

/* ── Buffer circular — operaciones ──────────────────────── */
static void kbd_buf_push(char c) {
    int next = (kbd_tail + 1) & (KBD_BUF_SIZE - 1);
    if (next != kbd_head) {          /* no sobrescribir si lleno */
        kbd_buf[kbd_tail] = c;
        kbd_tail = next;
    }
}

static int kbd_buf_pop(char *out) {
    if (kbd_head == kbd_tail) return 0;   /* vacío */
    *out = kbd_buf[kbd_head];
    kbd_head = (kbd_head + 1) & (KBD_BUF_SIZE - 1);
    return 1;
}

static int kbd_buf_empty(void) {
    return kbd_head == kbd_tail;
}

/* ── IRQ Handler — llamado por el interrupt handler de IRQ1 ─ */
void keyboard_irq_handler(void) {
    /* Verificar que hay un byte listo */
    unsigned char status = _inb(KBD_STATUS_PORT);
    if (!(status & KBD_STATUS_OUTPUT_FULL)) {
        _outb(PIC1_CMD, PIC_EOI);
        return;
    }

    unsigned char scancode = _inb(KBD_DATA_PORT);

    /* Shift press/release */
    if (scancode == KBD_SC_LSHIFT || scancode == KBD_SC_RSHIFT) {
        kbd_shift_active = 1;
        _outb(PIC1_CMD, PIC_EOI);
        return;
    }
    if (scancode == KBD_SC_LSHIFT_REL || scancode == KBD_SC_RSHIFT_REL) {
        kbd_shift_active = 0;
        _outb(PIC1_CMD, PIC_EOI);
        return;
    }

    /* Solo procesar key-press (scancode < 0x80) */
    if (scancode & 0x80) {
        /* Key release — ignorar (excepto shift, ya manejado) */
        _outb(PIC1_CMD, PIC_EOI);
        return;
    }

    /* Convertir scancode → ASCII */
    char ascii;
    if (kbd_shift_active) {
        ascii = kbd_scancode_shift[scancode & 0x7F];
    } else {
        ascii = kbd_scancode_normal[scancode & 0x7F];
    }

    if (ascii != 0) {
        kbd_buf_push(ascii);
    }

    /* Señal End-Of-Interrupt al PIC maestro */
    _outb(PIC1_CMD, PIC_EOI);
}

/* ── API Pública ─────────────────────────────────────────── */

/**
 * keyboard_init() — Habilita IRQ1 en el PIC maestro.
 * Llamado desde kernel_main() DESPUÉS de idt_init().
 */
void keyboard_init(void) {
    /* Leer máscara actual del PIC maestro */
    unsigned char mask = _inb(PIC1_DATA);
    /* Limpiar bit 1 (IRQ1 = teclado) para habilitarlo */
    mask &= ~(1 << 1);
    _outb(PIC1_DATA, mask);

    kbd_head  = 0;
    kbd_tail  = 0;
    kbd_shift_active = 0;
    kbd_ready = 1;

    kprintf("[KBD] PS/2 keyboard initialized (IRQ1, Scancode Set 1)\n");
}

/**
 * keyboard_getchar() — Lee un carácter del buffer.
 * Bloquea (spinning) hasta que hay una tecla disponible.
 * El scheduler tick sigue corriendo durante la espera.
 */
char keyboard_getchar(void) {
    char c;
    /* Busy-wait: el PIC/IDT maneja la interrupción en segundo plano */
    while (kbd_buf_empty()) {
        asm volatile("hlt");  /* esperar interrupt (PIT tick o keypress) */
    }
    kbd_buf_pop(&c);
    return c;
}

/**
 * keyboard_getchar_nowait() — Intenta leer, retorna 0 si no hay nada.
 */
char keyboard_getchar_nowait(void) {
    char c = 0;
    kbd_buf_pop(&c);
    return c;
}

/**
 * keyboard_ready() — ¿Está el driver inicializado?
 */
int keyboard_ready(void) {
    return kbd_ready;
}
