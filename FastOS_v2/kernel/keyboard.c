/*
 * FastOS v2.0 — PS/2 Keyboard Driver (i8042)
 * IRQ1 (vector 33), scan code set 1 (US QWERTY)
 * Ports: 0x60 (data), 0x64 (status/command)
 */

#include "include/kernel.h"

/* Circular key buffer */
#define KEY_BUF_SIZE 64
static char key_buffer[KEY_BUF_SIZE];
static volatile int key_head = 0;
static volatile int key_tail = 0;

static int shift_held = 0;
static int caps_lock  = 0;

/* Scan code set 1 → ASCII (unshifted) */
static const char scancode_ascii[128] = {
    0,  27, '1','2','3','4','5','6','7','8','9','0','-','=','\b',
    '\t','q','w','e','r','t','y','u','i','o','p','[',']','\n',
    0,  'a','s','d','f','g','h','j','k','l',';','\'','`',
    0,  '\\','z','x','c','v','b','n','m',',','.','/',0,
    '*', 0, ' ', 0,   /* F1-F10 follow */
    0,0,0,0,0,0,0,0,0,0,  /* F1-F10 */
    0, 0,  /* NumLock, ScrollLock */
    0,0,0,'-',0,0,0,'+',0,0,0,0,0,  /* Keypad */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
};

/* Shifted variants */
static const char scancode_shift[128] = {
    0,  27, '!','@','#','$','%','^','&','*','(',')','_','+','\b',
    '\t','Q','W','E','R','T','Y','U','I','O','P','{','}','\n',
    0,  'A','S','D','F','G','H','J','K','L',':','"','~',
    0,  '|','Z','X','C','V','B','N','M','<','>','?',0,
    '*', 0, ' ', 0,
    0,0,0,0,0,0,0,0,0,0,
    0, 0,
    0,0,0,'-',0,0,0,'+',0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
};

/* Forward declarations */
typedef struct {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
    uint64_t int_no, err_code;
    uint64_t rip, cs, rflags, rsp, ss;
} __packed isr_frame_t;

typedef void (*irq_handler_t)(isr_frame_t *frame);
extern void irq_register_handler(uint8_t irq, irq_handler_t handler);

static void key_buffer_push(char c) {
    int next = (key_head + 1) % KEY_BUF_SIZE;
    if (next != key_tail) {
        key_buffer[key_head] = c;
        key_head = next;
    }
}

/* IRQ1 handler */
static void keyboard_irq_handler(isr_frame_t *frame) {
    (void)frame;
    uint8_t sc = inb(0x60);

    /* Key release (bit 7 set) */
    if (sc & 0x80) {
        uint8_t released = sc & 0x7F;
        if (released == 0x2A || released == 0x36) shift_held = 0;
        return;
    }

    /* Shift press */
    if (sc == 0x2A || sc == 0x36) { shift_held = 1; return; }

    /* Caps Lock toggle */
    if (sc == 0x3A) { caps_lock = !caps_lock; return; }

    /* Convert to ASCII */
    char c;
    if (shift_held) {
        c = scancode_shift[sc];
    } else {
        c = scancode_ascii[sc];
    }

    /* Caps lock affects letters */
    if (caps_lock && c >= 'a' && c <= 'z') c -= 32;
    else if (caps_lock && c >= 'A' && c <= 'Z') c += 32;

    if (c) key_buffer_push(c);
}

void keyboard_init(void) {
    /* Flush any pending scancodes */
    while (inb(0x64) & 1) inb(0x60);

    irq_register_handler(1, keyboard_irq_handler);
    pic_clear_mask(1);
}

int keyboard_has_key(void) {
    return key_head != key_tail;
}

char keyboard_getchar(void) {
    /* Blocking wait for a key */
    while (key_head == key_tail) {
        hlt();   /* Sleep until next interrupt */
    }
    char c = key_buffer[key_tail];
    key_tail = (key_tail + 1) % KEY_BUF_SIZE;
    return c;
}
