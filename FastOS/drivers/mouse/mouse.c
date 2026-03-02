/*
 * FastOS v2.0 — PS/2 Mouse Driver
 * Generic mouse driver for x86/x86-64
 * Works on all PCs with PS/2 or USB legacy support
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* PS/2 Controller Ports */
#define MOUSE_DATA_PORT    0x60
#define MOUSE_STATUS_PORT  0x64
#define MOUSE_CMD_PORT     0x64

/* Controller Commands */
#define PS2_CMD_WRITE_MOUSE    0xD4
#define PS2_CMD_ENABLE_AUX     0xA8
#define PS2_CMD_DISABLE_AUX    0xA7
#define PS2_CMD_READ_CONFIG    0x20
#define PS2_CMD_WRITE_CONFIG   0x60

/* Mouse Commands */
#define MOUSE_CMD_RESET        0xFF
#define MOUSE_CMD_RESEND       0xFE
#define MOUSE_CMD_SET_DEFAULTS 0xF6
#define MOUSE_CMD_DISABLE      0xF5
#define MOUSE_CMD_ENABLE       0xF4
#define MOUSE_CMD_SET_RATE     0xF3
#define MOUSE_CMD_GET_ID       0xF2
#define MOUSE_CMD_SET_REMOTE   0xF0
#define MOUSE_CMD_SET_WRAP     0xEE
#define MOUSE_CMD_RESET_WRAP   0xEC
#define MOUSE_CMD_READ_DATA    0xEB
#define MOUSE_CMD_SET_STREAM   0xEA
#define MOUSE_CMD_STATUS_REQ   0xE9
#define MOUSE_CMD_SET_RES      0xE8
#define MOUSE_CMD_SET_SCALING  0xE6

/* Mouse State */
typedef struct {
    int32_t x;
    int32_t y;
    int32_t scroll;
    uint8_t buttons;
    uint8_t left_button;
    uint8_t right_button;
    uint8_t middle_button;
} mouse_state_t;

static mouse_state_t mouse = {0};
static uint8_t mouse_cycle = 0;
static uint8_t mouse_packet[4];
static uint8_t mouse_id = 0;  /* 0=standard, 3=scroll, 4=5-button */

/* Screen bounds */
static int32_t screen_width = 80;   /* Text mode default */
static int32_t screen_height = 25;

/* Wait for controller */
static void mouse_wait_input(void) {
    int timeout = 100000;
    while ((inb(MOUSE_STATUS_PORT) & 0x02) && timeout--) {
        io_wait();
    }
}

static void mouse_wait_output(void) {
    int timeout = 100000;
    while (!(inb(MOUSE_STATUS_PORT) & 0x01) && timeout--) {
        io_wait();
    }
}

/* Send command to mouse */
static void mouse_write(uint8_t data) {
    mouse_wait_input();
    outb(MOUSE_CMD_PORT, PS2_CMD_WRITE_MOUSE);
    mouse_wait_input();
    outb(MOUSE_DATA_PORT, data);
}

/* Read from mouse */
static uint8_t mouse_read(void) {
    mouse_wait_output();
    return inb(MOUSE_DATA_PORT);
}

/* Send command and wait for ACK */
static int mouse_send_cmd(uint8_t cmd) {
    mouse_write(cmd);
    uint8_t response = mouse_read();
    return (response == 0xFA) ? 0 : -1;  /* 0xFA = ACK */
}

/* Try to enable scroll wheel (becomes mouse ID 3) */
static void mouse_enable_scroll(void) {
    /* Magic sequence to enable scroll wheel */
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(200);
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(100);
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(80);
    
    /* Get mouse ID */
    mouse_send_cmd(MOUSE_CMD_GET_ID);
    mouse_id = mouse_read();
}

/* Try to enable 5-button mouse (becomes mouse ID 4) */
static void mouse_enable_5button(void) {
    /* Magic sequence for 5-button */
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(200);
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(200);
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(80);
    
    mouse_send_cmd(MOUSE_CMD_GET_ID);
    mouse_id = mouse_read();
}

/* Mouse interrupt handler (IRQ12) */
void mouse_handler(void) {
    uint8_t status = inb(MOUSE_STATUS_PORT);
    
    /* Check if data is from mouse (bit 5 set) */
    if (!(status & 0x20)) {
        return;
    }
    
    uint8_t data = inb(MOUSE_DATA_PORT);
    
    switch (mouse_cycle) {
        case 0:
            /* First byte: buttons and sign bits */
            if (data & 0x08) {  /* Bit 3 should always be set */
                mouse_packet[0] = data;
                mouse_cycle++;
            }
            break;
            
        case 1:
            /* Second byte: X movement */
            mouse_packet[1] = data;
            mouse_cycle++;
            break;
            
        case 2:
            /* Third byte: Y movement */
            mouse_packet[2] = data;
            
            if (mouse_id == 0) {
                /* Standard 3-byte packet complete */
                mouse_cycle = 0;
                
                /* Process packet */
                mouse.buttons = mouse_packet[0] & 0x07;
                mouse.left_button = mouse_packet[0] & 0x01;
                mouse.right_button = (mouse_packet[0] >> 1) & 0x01;
                mouse.middle_button = (mouse_packet[0] >> 2) & 0x01;
                
                /* Calculate movement with sign extension */
                int32_t dx = mouse_packet[1];
                int32_t dy = mouse_packet[2];
                
                if (mouse_packet[0] & 0x10) dx |= 0xFFFFFF00;  /* X sign */
                if (mouse_packet[0] & 0x20) dy |= 0xFFFFFF00;  /* Y sign */
                
                /* Update position (Y is inverted) */
                mouse.x += dx;
                mouse.y -= dy;
                
                /* Clamp to screen bounds */
                if (mouse.x < 0) mouse.x = 0;
                if (mouse.y < 0) mouse.y = 0;
                if (mouse.x >= screen_width) mouse.x = screen_width - 1;
                if (mouse.y >= screen_height) mouse.y = screen_height - 1;
            } else {
                mouse_cycle++;
            }
            break;
            
        case 3:
            /* Fourth byte: scroll wheel (for ID 3/4) */
            mouse_packet[3] = data;
            mouse_cycle = 0;
            
            /* Process 4-byte packet */
            mouse.buttons = mouse_packet[0] & 0x07;
            mouse.left_button = mouse_packet[0] & 0x01;
            mouse.right_button = (mouse_packet[0] >> 1) & 0x01;
            mouse.middle_button = (mouse_packet[0] >> 2) & 0x01;
            
            int32_t dx = mouse_packet[1];
            int32_t dy = mouse_packet[2];
            
            if (mouse_packet[0] & 0x10) dx |= 0xFFFFFF00;
            if (mouse_packet[0] & 0x20) dy |= 0xFFFFFF00;
            
            mouse.x += dx;
            mouse.y -= dy;
            
            /* Scroll wheel */
            int8_t scroll = (int8_t)(mouse_packet[3] & 0x0F);
            if (scroll & 0x08) scroll |= 0xF0;  /* Sign extend */
            mouse.scroll += scroll;
            
            /* Clamp position */
            if (mouse.x < 0) mouse.x = 0;
            if (mouse.y < 0) mouse.y = 0;
            if (mouse.x >= screen_width) mouse.x = screen_width - 1;
            if (mouse.y >= screen_height) mouse.y = screen_height - 1;
            break;
    }
}

/* Get mouse position */
void mouse_get_position(int32_t *x, int32_t *y) {
    *x = mouse.x;
    *y = mouse.y;
}

/* Get button state */
int mouse_get_buttons(void) {
    return mouse.buttons;
}

int mouse_left_pressed(void) {
    return mouse.left_button;
}

int mouse_right_pressed(void) {
    return mouse.right_button;
}

int mouse_middle_pressed(void) {
    return mouse.middle_button;
}

/* Get scroll wheel delta */
int32_t mouse_get_scroll(void) {
    int32_t s = mouse.scroll;
    mouse.scroll = 0;
    return s;
}

/* Set screen bounds for mouse */
void mouse_set_bounds(int32_t width, int32_t height) {
    screen_width = width;
    screen_height = height;
    
    /* Clamp current position */
    if (mouse.x >= width) mouse.x = width - 1;
    if (mouse.y >= height) mouse.y = height - 1;
}

/* Set mouse position */
void mouse_set_position(int32_t x, int32_t y) {
    mouse.x = x;
    mouse.y = y;
}

/* Initialize mouse */
void mouse_init(void) {
    kprintf("[MOUSE] Initializing PS/2 mouse driver...\n");
    
    /* Enable auxiliary device (mouse) */
    mouse_wait_input();
    outb(MOUSE_CMD_PORT, PS2_CMD_ENABLE_AUX);
    
    /* Enable interrupts for mouse */
    mouse_wait_input();
    outb(MOUSE_CMD_PORT, PS2_CMD_READ_CONFIG);
    uint8_t config = mouse_read();
    config |= 0x02;  /* Enable IRQ12 */
    config &= ~0x20; /* Enable mouse clock */
    mouse_wait_input();
    outb(MOUSE_CMD_PORT, PS2_CMD_WRITE_CONFIG);
    mouse_wait_input();
    outb(MOUSE_DATA_PORT, config);
    
    /* Reset mouse */
    mouse_send_cmd(MOUSE_CMD_RESET);
    mouse_read();  /* 0xAA = self-test passed */
    mouse_read();  /* 0x00 = mouse ID */
    
    /* Set defaults */
    mouse_send_cmd(MOUSE_CMD_SET_DEFAULTS);
    
    /* Try to enable scroll wheel */
    mouse_enable_scroll();
    if (mouse_id == 3) {
        kprintf("[MOUSE] Scroll wheel detected\n");
        /* Try 5-button */
        mouse_enable_5button();
        if (mouse_id == 4) {
            kprintf("[MOUSE] 5-button mouse detected\n");
        }
    }
    
    /* Set sample rate (100 samples/sec) */
    mouse_send_cmd(MOUSE_CMD_SET_RATE);
    mouse_write(100);
    
    /* Set resolution (8 counts/mm) */
    mouse_send_cmd(MOUSE_CMD_SET_RES);
    mouse_write(3);
    
    /* Enable data reporting */
    mouse_send_cmd(MOUSE_CMD_ENABLE);
    
    /* Set initial position to center */
    mouse.x = screen_width / 2;
    mouse.y = screen_height / 2;
    
    kprintf("[MOUSE] Mouse initialized (ID=%d)\n", mouse_id);
}
