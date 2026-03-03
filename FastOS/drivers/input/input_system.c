/*
 * FastOS v2.0 — Unified Input System
 * PS/2 Mouse + Keyboard with Window Manager integration
 * 
 * Compile: adB cc input_system.c -o input.po --driver
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* ============================================================
 * Input System Constants
 * ============================================================ */

/* PS/2 Controller Ports */
#define PS2_DATA_PORT       0x60
#define PS2_STATUS_PORT     0x64
#define PS2_COMMAND_PORT    0x64

/* PS/2 Controller Commands */
#define PS2_CMD_READ_CONFIG     0x20
#define PS2_CMD_WRITE_CONFIG    0x60
#define PS2_CMD_DISABLE_PORT2   0xA7
#define PS2_CMD_ENABLE_PORT2    0xA8
#define PS2_CMD_TEST_PORT2      0xA9
#define PS2_CMD_TEST_CONTROLLER 0xAA
#define PS2_CMD_TEST_PORT1      0xAB
#define PS2_CMD_DISABLE_PORT1   0xAD
#define PS2_CMD_ENABLE_PORT1    0xAE
#define PS2_CMD_WRITE_PORT2     0xD4

/* PS/2 Device Commands */
#define PS2_DEV_RESET           0xFF
#define PS2_DEV_ENABLE          0xF4
#define PS2_DEV_DISABLE         0xF5
#define PS2_DEV_SET_DEFAULTS    0xF6
#define PS2_DEV_SET_SAMPLE_RATE 0xF3
#define PS2_DEV_SET_RESOLUTION  0xE8
#define PS2_DEV_GET_ID          0xF2

/* Mouse packet flags */
#define MOUSE_LEFT_BTN      0x01
#define MOUSE_RIGHT_BTN     0x02
#define MOUSE_MIDDLE_BTN    0x04
#define MOUSE_X_SIGN        0x10
#define MOUSE_Y_SIGN        0x20
#define MOUSE_X_OVERFLOW    0x40
#define MOUSE_Y_OVERFLOW    0x80

/* Keyboard scan codes (Set 1) */
#define KEY_ESC             0x01
#define KEY_1               0x02
#define KEY_2               0x03
#define KEY_3               0x04
#define KEY_4               0x05
#define KEY_5               0x06
#define KEY_6               0x07
#define KEY_7               0x08
#define KEY_8               0x09
#define KEY_9               0x0A
#define KEY_0               0x0B
#define KEY_MINUS           0x0C
#define KEY_EQUALS          0x0D
#define KEY_BACKSPACE       0x0E
#define KEY_TAB             0x0F
#define KEY_Q               0x10
#define KEY_W               0x11
#define KEY_E               0x12
#define KEY_R               0x13
#define KEY_T               0x14
#define KEY_Y               0x15
#define KEY_U               0x16
#define KEY_I               0x17
#define KEY_O               0x18
#define KEY_P               0x19
#define KEY_LBRACKET        0x1A
#define KEY_RBRACKET        0x1B
#define KEY_ENTER           0x1C
#define KEY_LCTRL           0x1D
#define KEY_A               0x1E
#define KEY_S               0x1F
#define KEY_D               0x20
#define KEY_F               0x21
#define KEY_G               0x22
#define KEY_H               0x23
#define KEY_J               0x24
#define KEY_K               0x25
#define KEY_L               0x26
#define KEY_SEMICOLON       0x27
#define KEY_QUOTE           0x28
#define KEY_BACKTICK        0x29
#define KEY_LSHIFT          0x2A
#define KEY_BACKSLASH       0x2B
#define KEY_Z               0x2C
#define KEY_X               0x2D
#define KEY_C               0x2E
#define KEY_V               0x2F
#define KEY_B               0x30
#define KEY_N               0x31
#define KEY_M               0x32
#define KEY_COMMA           0x33
#define KEY_PERIOD          0x34
#define KEY_SLASH           0x35
#define KEY_RSHIFT          0x36
#define KEY_LALT            0x38
#define KEY_SPACE           0x39
#define KEY_CAPSLOCK        0x3A
#define KEY_F1              0x3B
#define KEY_F2              0x3C
#define KEY_F3              0x3D
#define KEY_F4              0x3E
#define KEY_F5              0x3F
#define KEY_F6              0x40
#define KEY_F7              0x41
#define KEY_F8              0x42
#define KEY_F9              0x43
#define KEY_F10             0x44
#define KEY_F11             0x57
#define KEY_F12             0x58

/* ============================================================
 * Input Event Types
 * ============================================================ */

typedef enum {
    INPUT_EVENT_NONE = 0,
    INPUT_EVENT_KEY_DOWN,
    INPUT_EVENT_KEY_UP,
    INPUT_EVENT_MOUSE_MOVE,
    INPUT_EVENT_MOUSE_BUTTON_DOWN,
    INPUT_EVENT_MOUSE_BUTTON_UP,
    INPUT_EVENT_MOUSE_SCROLL
} input_event_type_t;

typedef struct {
    input_event_type_t type;
    uint32_t timestamp;
    
    union {
        /* Keyboard event */
        struct {
            uint8_t scancode;
            uint8_t keycode;
            char ascii;
            uint8_t modifiers;
        } key;
        
        /* Mouse event */
        struct {
            int32_t x, y;           /* Absolute position */
            int32_t dx, dy;         /* Relative movement */
            uint8_t buttons;
            int8_t scroll;
        } mouse;
    };
} input_event_t;

/* ============================================================
 * Input State
 * ============================================================ */

typedef struct {
    /* Mouse state */
    int32_t mouse_x;
    int32_t mouse_y;
    uint8_t mouse_buttons;
    uint8_t mouse_packet[4];
    int mouse_packet_index;
    int mouse_has_wheel;
    
    /* Keyboard state */
    uint8_t key_state[256];     /* 1 = pressed, 0 = released */
    uint8_t modifiers;          /* Shift, Ctrl, Alt flags */
    
    /* Screen bounds */
    int32_t screen_width;
    int32_t screen_height;
    
    /* Event queue */
    input_event_t event_queue[64];
    int queue_head;
    int queue_tail;
    
    /* Callbacks */
    void (*on_mouse_move)(int32_t x, int32_t y);
    void (*on_mouse_button)(uint8_t buttons);
    void (*on_key)(uint8_t scancode, int pressed);
} input_state_t;

static input_state_t input;

/* Modifier flags */
#define MOD_SHIFT   0x01
#define MOD_CTRL    0x02
#define MOD_ALT     0x04
#define MOD_CAPS    0x08

/* Scancode to ASCII table (US layout) */
static const char scancode_to_ascii[128] = {
    0, 27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0, '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

static const char scancode_to_ascii_shift[128] = {
    0, 27, '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\b',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    0, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    0, '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0,
    '*', 0, ' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    '7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

/* ============================================================
 * Low-level PS/2 Functions
 * ============================================================ */

static inline void ps2_wait_write(void) {
    int timeout = 100000;
    while ((inb(PS2_STATUS_PORT) & 0x02) && timeout--);
}

static inline void ps2_wait_read(void) {
    int timeout = 100000;
    while (!(inb(PS2_STATUS_PORT) & 0x01) && timeout--);
}

static void ps2_write_command(uint8_t cmd) {
    ps2_wait_write();
    outb(PS2_COMMAND_PORT, cmd);
}

static void ps2_write_data(uint8_t data) {
    ps2_wait_write();
    outb(PS2_DATA_PORT, data);
}

static uint8_t ps2_read_data(void) {
    ps2_wait_read();
    return inb(PS2_DATA_PORT);
}

static void ps2_mouse_write(uint8_t data) {
    ps2_write_command(PS2_CMD_WRITE_PORT2);
    ps2_write_data(data);
}

/* ============================================================
 * Event Queue
 * ============================================================ */

static void input_queue_event(input_event_t *event) {
    int next = (input.queue_head + 1) % 64;
    if (next != input.queue_tail) {
        input.event_queue[input.queue_head] = *event;
        input.queue_head = next;
    }
}

int input_poll_event(input_event_t *event) {
    if (input.queue_head == input.queue_tail) {
        return 0;  /* No events */
    }
    
    *event = input.event_queue[input.queue_tail];
    input.queue_tail = (input.queue_tail + 1) % 64;
    return 1;
}

/* ============================================================
 * Mouse Handling
 * ============================================================ */

static void mouse_process_packet(void) {
    uint8_t flags = input.mouse_packet[0];
    
    /* Check for overflow */
    if (flags & (MOUSE_X_OVERFLOW | MOUSE_Y_OVERFLOW)) {
        return;
    }
    
    /* Calculate movement */
    int32_t dx = input.mouse_packet[1];
    int32_t dy = input.mouse_packet[2];
    
    /* Apply sign */
    if (flags & MOUSE_X_SIGN) dx |= 0xFFFFFF00;
    if (flags & MOUSE_Y_SIGN) dy |= 0xFFFFFF00;
    
    /* Y is inverted */
    dy = -dy;
    
    /* Update position */
    input.mouse_x += dx;
    input.mouse_y += dy;
    
    /* Clamp to screen */
    if (input.mouse_x < 0) input.mouse_x = 0;
    if (input.mouse_y < 0) input.mouse_y = 0;
    if (input.mouse_x >= input.screen_width) input.mouse_x = input.screen_width - 1;
    if (input.mouse_y >= input.screen_height) input.mouse_y = input.screen_height - 1;
    
    /* Create movement event */
    if (dx != 0 || dy != 0) {
        input_event_t event = {0};
        event.type = INPUT_EVENT_MOUSE_MOVE;
        event.mouse.x = input.mouse_x;
        event.mouse.y = input.mouse_y;
        event.mouse.dx = dx;
        event.mouse.dy = dy;
        event.mouse.buttons = flags & 0x07;
        input_queue_event(&event);
        
        if (input.on_mouse_move) {
            input.on_mouse_move(input.mouse_x, input.mouse_y);
        }
    }
    
    /* Check button changes */
    uint8_t new_buttons = flags & 0x07;
    if (new_buttons != input.mouse_buttons) {
        uint8_t changed = new_buttons ^ input.mouse_buttons;
        
        for (int i = 0; i < 3; i++) {
            if (changed & (1 << i)) {
                input_event_t event = {0};
                event.type = (new_buttons & (1 << i)) ? 
                             INPUT_EVENT_MOUSE_BUTTON_DOWN : 
                             INPUT_EVENT_MOUSE_BUTTON_UP;
                event.mouse.x = input.mouse_x;
                event.mouse.y = input.mouse_y;
                event.mouse.buttons = new_buttons;
                input_queue_event(&event);
            }
        }
        
        input.mouse_buttons = new_buttons;
        
        if (input.on_mouse_button) {
            input.on_mouse_button(new_buttons);
        }
    }
    
    /* Handle scroll wheel (4-byte packet) */
    if (input.mouse_has_wheel && input.mouse_packet_index == 4) {
        int8_t scroll = (int8_t)input.mouse_packet[3];
        if (scroll != 0) {
            input_event_t event = {0};
            event.type = INPUT_EVENT_MOUSE_SCROLL;
            event.mouse.x = input.mouse_x;
            event.mouse.y = input.mouse_y;
            event.mouse.scroll = scroll;
            input_queue_event(&event);
        }
    }
}

void mouse_handle_irq(void) {
    uint8_t data = inb(PS2_DATA_PORT);
    
    /* First byte must have bit 3 set */
    if (input.mouse_packet_index == 0 && !(data & 0x08)) {
        return;  /* Out of sync, skip */
    }
    
    input.mouse_packet[input.mouse_packet_index++] = data;
    
    int packet_size = input.mouse_has_wheel ? 4 : 3;
    if (input.mouse_packet_index >= packet_size) {
        mouse_process_packet();
        input.mouse_packet_index = 0;
    }
}

/* ============================================================
 * Keyboard Handling
 * ============================================================ */

void keyboard_handle_irq(void) {
    uint8_t scancode = inb(PS2_DATA_PORT);
    
    int released = scancode & 0x80;
    uint8_t key = scancode & 0x7F;
    
    /* Update key state */
    input.key_state[key] = !released;
    
    /* Update modifiers */
    switch (key) {
        case KEY_LSHIFT:
        case KEY_RSHIFT:
            if (released) input.modifiers &= ~MOD_SHIFT;
            else input.modifiers |= MOD_SHIFT;
            break;
        case KEY_LCTRL:
            if (released) input.modifiers &= ~MOD_CTRL;
            else input.modifiers |= MOD_CTRL;
            break;
        case KEY_LALT:
            if (released) input.modifiers &= ~MOD_ALT;
            else input.modifiers |= MOD_ALT;
            break;
        case KEY_CAPSLOCK:
            if (!released) input.modifiers ^= MOD_CAPS;
            break;
    }
    
    /* Create event */
    input_event_t event = {0};
    event.type = released ? INPUT_EVENT_KEY_UP : INPUT_EVENT_KEY_DOWN;
    event.key.scancode = scancode;
    event.key.keycode = key;
    event.key.modifiers = input.modifiers;
    
    /* Convert to ASCII */
    if (key < 128) {
        int use_shift = (input.modifiers & MOD_SHIFT) != 0;
        int use_caps = (input.modifiers & MOD_CAPS) != 0;
        
        /* Caps only affects letters */
        if (key >= KEY_A && key <= KEY_Z) {
            use_shift = use_shift ^ use_caps;
        }
        
        event.key.ascii = use_shift ? 
                          scancode_to_ascii_shift[key] : 
                          scancode_to_ascii[key];
    }
    
    input_queue_event(&event);
    
    if (input.on_key) {
        input.on_key(scancode, !released);
    }
}

/* ============================================================
 * Input System API
 * ============================================================ */

void input_get_mouse_pos(int32_t *x, int32_t *y) {
    *x = input.mouse_x;
    *y = input.mouse_y;
}

uint8_t input_get_mouse_buttons(void) {
    return input.mouse_buttons;
}

int input_is_key_pressed(uint8_t keycode) {
    return input.key_state[keycode & 0x7F];
}

uint8_t input_get_modifiers(void) {
    return input.modifiers;
}

void input_set_mouse_callback(void (*callback)(int32_t x, int32_t y)) {
    input.on_mouse_move = callback;
}

void input_set_button_callback(void (*callback)(uint8_t buttons)) {
    input.on_mouse_button = callback;
}

void input_set_key_callback(void (*callback)(uint8_t scancode, int pressed)) {
    input.on_key = callback;
}

/* ============================================================
 * Initialization
 * ============================================================ */

static int mouse_init(void) {
    /* Enable mouse port */
    ps2_write_command(PS2_CMD_ENABLE_PORT2);
    
    /* Enable mouse */
    ps2_mouse_write(PS2_DEV_ENABLE);
    ps2_read_data();  /* ACK */
    
    /* Try to enable scroll wheel (IntelliMouse) */
    ps2_mouse_write(PS2_DEV_SET_SAMPLE_RATE);
    ps2_read_data();
    ps2_mouse_write(200);
    ps2_read_data();
    
    ps2_mouse_write(PS2_DEV_SET_SAMPLE_RATE);
    ps2_read_data();
    ps2_mouse_write(100);
    ps2_read_data();
    
    ps2_mouse_write(PS2_DEV_SET_SAMPLE_RATE);
    ps2_read_data();
    ps2_mouse_write(80);
    ps2_read_data();
    
    /* Get device ID */
    ps2_mouse_write(PS2_DEV_GET_ID);
    ps2_read_data();  /* ACK */
    uint8_t id = ps2_read_data();
    
    input.mouse_has_wheel = (id == 3 || id == 4);
    
    kprintf("[INPUT] Mouse initialized (wheel=%d)\n", input.mouse_has_wheel);
    
    return 0;
}

static int keyboard_init(void) {
    /* Enable keyboard port */
    ps2_write_command(PS2_CMD_ENABLE_PORT1);
    
    /* Flush buffer */
    while (inb(PS2_STATUS_PORT) & 0x01) {
        inb(PS2_DATA_PORT);
    }
    
    kprintf("[INPUT] Keyboard initialized\n");
    
    return 0;
}

int input_init(int32_t screen_width, int32_t screen_height) {
    kmemset(&input, 0, sizeof(input));
    
    input.screen_width = screen_width;
    input.screen_height = screen_height;
    input.mouse_x = screen_width / 2;
    input.mouse_y = screen_height / 2;
    
    /* Initialize PS/2 controller */
    ps2_write_command(PS2_CMD_DISABLE_PORT1);
    ps2_write_command(PS2_CMD_DISABLE_PORT2);
    
    /* Flush buffer */
    while (inb(PS2_STATUS_PORT) & 0x01) {
        inb(PS2_DATA_PORT);
    }
    
    /* Configure controller */
    ps2_write_command(PS2_CMD_READ_CONFIG);
    uint8_t config = ps2_read_data();
    config |= 0x03;  /* Enable IRQs */
    config &= ~0x30; /* Enable clocks */
    ps2_write_command(PS2_CMD_WRITE_CONFIG);
    ps2_write_data(config);
    
    /* Initialize devices */
    keyboard_init();
    mouse_init();
    
    kprintf("[INPUT] Input system ready (%dx%d)\n", screen_width, screen_height);
    
    return 0;
}
