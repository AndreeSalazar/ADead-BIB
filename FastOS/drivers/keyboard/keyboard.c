/*
 * FastOS v2.0 — PS/2 Keyboard Driver
 * Generic keyboard driver for x86/x86-64
 * Works on all PCs with PS/2 or USB legacy support
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* PS/2 Controller Ports */
#define KB_DATA_PORT    0x60
#define KB_STATUS_PORT  0x64
#define KB_CMD_PORT     0x64

/* Status Register Bits */
#define KB_STATUS_OUTPUT_FULL  0x01
#define KB_STATUS_INPUT_FULL   0x02

/* Keyboard Commands */
#define KB_CMD_SET_LEDS        0xED
#define KB_CMD_ECHO            0xEE
#define KB_CMD_SCANCODE_SET    0xF0
#define KB_CMD_IDENTIFY        0xF2
#define KB_CMD_SET_RATE        0xF3
#define KB_CMD_ENABLE          0xF4
#define KB_CMD_DISABLE         0xF5
#define KB_CMD_RESET           0xFF

/* Special Keys */
#define KEY_ESCAPE      0x01
#define KEY_BACKSPACE   0x0E
#define KEY_TAB         0x0F
#define KEY_ENTER       0x1C
#define KEY_LCTRL       0x1D
#define KEY_LSHIFT      0x2A
#define KEY_RSHIFT      0x36
#define KEY_LALT        0x38
#define KEY_CAPSLOCK    0x3A
#define KEY_F1          0x3B
#define KEY_F2          0x3C
#define KEY_F3          0x3D
#define KEY_F4          0x3E
#define KEY_F5          0x3F
#define KEY_F6          0x40
#define KEY_F7          0x41
#define KEY_F8          0x42
#define KEY_F9          0x43
#define KEY_F10         0x44
#define KEY_F11         0x57
#define KEY_F12         0x58
#define KEY_NUMLOCK     0x45
#define KEY_SCROLLLOCK  0x46

/* Keyboard State */
typedef struct {
    uint8_t shift_pressed;
    uint8_t ctrl_pressed;
    uint8_t alt_pressed;
    uint8_t caps_lock;
    uint8_t num_lock;
    uint8_t scroll_lock;
    uint8_t extended;
} keyboard_state_t;

static keyboard_state_t kb_state = {0};

/* Circular buffer for key events */
#define KB_BUFFER_SIZE 256
static uint8_t kb_buffer[KB_BUFFER_SIZE];
static volatile uint32_t kb_buffer_head = 0;
static volatile uint32_t kb_buffer_tail = 0;

/* US QWERTY Scancode to ASCII (Set 1) */
static const char scancode_to_ascii[128] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0, '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

/* Shifted characters */
static const char scancode_to_ascii_shift[128] = {
    0,  27, '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\b',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    0, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    0, '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

/* Wait for keyboard controller */
static void kb_wait_input(void) {
    int timeout = 100000;
    while ((inb(KB_STATUS_PORT) & KB_STATUS_INPUT_FULL) && timeout--) {
        io_wait();
    }
}

static void kb_wait_output(void) {
    int timeout = 100000;
    while (!(inb(KB_STATUS_PORT) & KB_STATUS_OUTPUT_FULL) && timeout--) {
        io_wait();
    }
}

/* Send command to keyboard */
static void kb_send_cmd(uint8_t cmd) {
    kb_wait_input();
    outb(KB_DATA_PORT, cmd);
}

/* Read from keyboard */
static uint8_t kb_read(void) {
    kb_wait_output();
    return inb(KB_DATA_PORT);
}

/* Update LEDs */
static void kb_update_leds(void) {
    uint8_t leds = 0;
    if (kb_state.scroll_lock) leds |= 0x01;
    if (kb_state.num_lock)    leds |= 0x02;
    if (kb_state.caps_lock)   leds |= 0x04;
    
    kb_send_cmd(KB_CMD_SET_LEDS);
    kb_send_cmd(leds);
}

/* Convert scancode to ASCII */
char kb_scancode_to_char(uint8_t scancode) {
    if (scancode >= 128) return 0;  /* Key release */
    
    char c;
    if (kb_state.shift_pressed ^ kb_state.caps_lock) {
        c = scancode_to_ascii_shift[scancode];
    } else {
        c = scancode_to_ascii[scancode];
    }
    
    /* Handle caps lock for letters only */
    if (kb_state.caps_lock && !kb_state.shift_pressed) {
        if (c >= 'a' && c <= 'z') {
            c = c - 'a' + 'A';
        }
    }
    
    return c;
}

/* Add key to buffer */
static void kb_buffer_push(uint8_t scancode) {
    uint32_t next = (kb_buffer_head + 1) % KB_BUFFER_SIZE;
    if (next != kb_buffer_tail) {
        kb_buffer[kb_buffer_head] = scancode;
        kb_buffer_head = next;
    }
}

/* Get key from buffer */
int kb_buffer_pop(uint8_t *scancode) {
    if (kb_buffer_head == kb_buffer_tail) {
        return 0;  /* Buffer empty */
    }
    *scancode = kb_buffer[kb_buffer_tail];
    kb_buffer_tail = (kb_buffer_tail + 1) % KB_BUFFER_SIZE;
    return 1;
}

/* Check if key available */
int kb_has_key(void) {
    return kb_buffer_head != kb_buffer_tail;
}

/* Keyboard interrupt handler (IRQ1) */
void keyboard_handler(void) {
    uint8_t scancode = inb(KB_DATA_PORT);
    
    /* Handle extended scancodes (0xE0 prefix) */
    if (scancode == 0xE0) {
        kb_state.extended = 1;
        return;
    }
    
    int released = scancode & 0x80;
    uint8_t key = scancode & 0x7F;
    
    /* Handle modifier keys */
    switch (key) {
        case KEY_LSHIFT:
        case KEY_RSHIFT:
            kb_state.shift_pressed = !released;
            break;
        case KEY_LCTRL:
            kb_state.ctrl_pressed = !released;
            break;
        case KEY_LALT:
            kb_state.alt_pressed = !released;
            break;
        case KEY_CAPSLOCK:
            if (!released) {
                kb_state.caps_lock = !kb_state.caps_lock;
                kb_update_leds();
            }
            break;
        case KEY_NUMLOCK:
            if (!released) {
                kb_state.num_lock = !kb_state.num_lock;
                kb_update_leds();
            }
            break;
        case KEY_SCROLLLOCK:
            if (!released) {
                kb_state.scroll_lock = !kb_state.scroll_lock;
                kb_update_leds();
            }
            break;
        default:
            if (!released) {
                kb_buffer_push(scancode);
            }
            break;
    }
    
    kb_state.extended = 0;
}

/* Get character (blocking) */
char kb_getchar(void) {
    uint8_t scancode;
    while (!kb_buffer_pop(&scancode)) {
        hlt();  /* Wait for interrupt */
    }
    return kb_scancode_to_char(scancode);
}

/* Get character (non-blocking) */
int kb_getchar_nonblock(char *c) {
    uint8_t scancode;
    if (kb_buffer_pop(&scancode)) {
        *c = kb_scancode_to_char(scancode);
        return 1;
    }
    return 0;
}

/* Get raw scancode */
int kb_get_scancode(uint8_t *scancode) {
    return kb_buffer_pop(scancode);
}

/* Check modifier state */
int kb_is_shift_pressed(void) { return kb_state.shift_pressed; }
int kb_is_ctrl_pressed(void)  { return kb_state.ctrl_pressed; }
int kb_is_alt_pressed(void)   { return kb_state.alt_pressed; }
int kb_is_caps_lock(void)     { return kb_state.caps_lock; }

/* Initialize keyboard */
void keyboard_init(void) {
    kprintf("[KB] Initializing PS/2 keyboard driver...\n");
    
    /* Flush buffer */
    while (inb(KB_STATUS_PORT) & KB_STATUS_OUTPUT_FULL) {
        inb(KB_DATA_PORT);
    }
    
    /* Enable keyboard */
    kb_send_cmd(KB_CMD_ENABLE);
    
    /* Set default state */
    kb_state.num_lock = 1;  /* NumLock on by default */
    kb_update_leds();
    
    kprintf("[KB] Keyboard initialized (US QWERTY)\n");
}
