/* FastOS v3.0 — PS/2 Mouse Driver (i8042 auxiliary port)
 * ADead-BIB Native OS — Framebuffer GUI mode
 *
 * PS/2 mouse via i8042 controller auxiliary port.
 * Polling-based (no IRQ12 yet — no IDT installed).
 *
 * Mouse initialization sequence:
 *   1. Enable auxiliary port:     cmd 0xA8 to port 0x64
 *   2. Read controller config:    cmd 0x20 → read 0x60
 *   3. Set bit 1 (IRQ12 enable):  OR 0x02
 *   4. Clear bit 5 (mouse clock): AND 0xDF
 *   5. Write config back:         cmd 0x60 → write 0x60
 *   6. Reset mouse defaults:      cmd 0xD4, then 0xF6
 *   7. Enable streaming:          cmd 0xD4, then 0xF4
 *
 * Mouse packet format (3 bytes):
 *   Byte 1 (status): buttons + sign + overflow
 *     Bit 0: Left button
 *     Bit 1: Right button
 *     Bit 2: Middle button
 *     Bit 3: Always 1 (sync bit)
 *     Bit 4: X sign bit
 *     Bit 5: Y sign bit
 *   Byte 2: X delta (unsigned, sign in byte 1 bit 4)
 *   Byte 3: Y delta (unsigned, sign in byte 1 bit 5)
 *
 * Framebuffer cursor:
 *   Position clamped to [0, fb_width-1] × [0, fb_height-1]
 *   Cursor rendered via fb_cursor_draw() from drivers/fb.c
 *
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#include "../include/types.h"

/* ================================================================
 * Mouse State
 * ================================================================ */

typedef struct {
    int32_t  x;             /* current X position (pixels) */
    int32_t  y;             /* current Y position (pixels) */
    int32_t  max_x;         /* right bound (fb_width - 1) */
    int32_t  max_y;         /* bottom bound (fb_height - 1) */
    uint8_t  buttons;       /* bit0=left, bit1=right, bit2=middle */
    uint8_t  prev_buttons;  /* previous frame buttons (for click detect) */
    int      initialized;   /* 1 after successful init */
    int      packet_byte;   /* 0,1,2 — tracks which byte of packet */
    uint8_t  packet[3];     /* raw packet bytes */
    int32_t  sensitivity;   /* movement multiplier (default 1) */
} mouse_state_t;

static mouse_state_t mouse;

/* ================================================================
 * i8042 Helpers
 *
 * Wait for controller ready before sending commands.
 * Port 0x64 bit 1 = input buffer full (don't write).
 * Port 0x64 bit 0 = output buffer full (data ready).
 * ================================================================ */

static void mouse_wait_input(void)
{
    int timeout;
    timeout = 100000;
    while (timeout > 0) {
        if ((__inb(0x64) & 2) == 0) return;
        timeout = timeout - 1;
    }
}

static void mouse_wait_output(void)
{
    int timeout;
    timeout = 100000;
    while (timeout > 0) {
        if (__inb(0x64) & 1) return;
        timeout = timeout - 1;
    }
}

/* Send command to i8042 controller (port 0x64) */
static void mouse_cmd(uint8_t cmd)
{
    mouse_wait_input();
    __outb(0x64, cmd);
}

/* Send data byte to mouse via controller (0xD4 prefix) */
static void mouse_write(uint8_t data)
{
    mouse_wait_input();
    __outb(0x64, 0xD4);   /* "write next byte to auxiliary device" */
    mouse_wait_input();
    __outb(0x60, data);
}

/* Read response byte from mouse */
static uint8_t mouse_read(void)
{
    mouse_wait_output();
    return __inb(0x60);
}

/* ================================================================
 * Mouse Init
 *
 * Called from kernel_main() during GUI init.
 * Sets up PS/2 mouse in streaming mode.
 * Returns 1 on success, 0 on failure.
 * ================================================================ */

static int mouse_init(int32_t screen_w, int32_t screen_h)
{
    uint8_t config;
    uint8_t ack;

    mouse.x = screen_w / 2;
    mouse.y = screen_h / 2;
    mouse.max_x = screen_w - 1;
    mouse.max_y = screen_h - 1;
    mouse.buttons = 0;
    mouse.prev_buttons = 0;
    mouse.packet_byte = 0;
    mouse.sensitivity = 1;
    mouse.initialized = 0;

    /* Step 1: Enable auxiliary port */
    mouse_cmd(0xA8);

    /* Step 2: Read controller config */
    mouse_cmd(0x20);
    config = mouse_read();

    /* Step 3-4: Enable IRQ12, enable mouse clock */
    config = config | 0x02;    /* set bit 1: enable IRQ12 */
    config = config & 0xDF;    /* clear bit 5: enable mouse clock */

    /* Step 5: Write config back */
    mouse_cmd(0x60);
    mouse_wait_input();
    __outb(0x60, config);

    /* Step 6: Reset mouse to defaults */
    mouse_write(0xF6);
    ack = mouse_read();
    if (ack != 0xFA) {
        /* Serial debug: "MOUSE:FAIL\r\n" */
        __outb(0x3F8, 77); __outb(0x3F8, 79); __outb(0x3F8, 85);
        __outb(0x3F8, 83); __outb(0x3F8, 69); __outb(0x3F8, 58);
        __outb(0x3F8, 70); __outb(0x3F8, 65); __outb(0x3F8, 73);
        __outb(0x3F8, 76); __outb(0x3F8, 13); __outb(0x3F8, 10);
        return 0;
    }

    /* Step 7: Enable streaming mode */
    mouse_write(0xF4);
    ack = mouse_read();
    if (ack != 0xFA) return 0;

    mouse.initialized = 1;

    /* Serial debug: "MOUSE:OK\r\n" */
    __outb(0x3F8, 77); __outb(0x3F8, 79); __outb(0x3F8, 85);
    __outb(0x3F8, 83); __outb(0x3F8, 69); __outb(0x3F8, 58);
    __outb(0x3F8, 79); __outb(0x3F8, 75);
    __outb(0x3F8, 13); __outb(0x3F8, 10);

    return 1;
}

/* ================================================================
 * Mouse Poll — Read one byte if available
 *
 * Called from the main event loop. Accumulates bytes into a
 * 3-byte packet. Returns 1 when a complete packet is ready
 * and position/buttons have been updated.
 *
 * Checks bit 5 of status register to distinguish mouse data
 * from keyboard data (both share port 0x60).
 * ================================================================ */

static int mouse_poll(void)
{
    uint8_t status;
    uint8_t data;
    int32_t dx, dy;
    int packet_complete;

    if (!mouse.initialized) return 0;

    status = __inb(0x64);

    /* Check: output buffer full AND auxiliary data bit set */
    if ((status & 0x01) == 0) return 0;  /* no data */
    if ((status & 0x20) == 0) return 0;  /* keyboard data, not mouse */

    data = __inb(0x60);
    packet_complete = 0;

    /* Byte 0: status byte — must have bit 3 set (sync) */
    if (mouse.packet_byte == 0) {
        if (data & 0x08) {
            /* Valid sync bit — start packet */
            mouse.packet[0] = data;
            mouse.packet_byte = 1;
        }
        /* else: out of sync, discard and wait for valid byte 0 */
        return 0;
    }

    if (mouse.packet_byte == 1) {
        mouse.packet[1] = data;
        mouse.packet_byte = 2;
        return 0;
    }

    if (mouse.packet_byte == 2) {
        mouse.packet[2] = data;
        mouse.packet_byte = 0;
        packet_complete = 1;
    }

    if (!packet_complete) return 0;

    /* Decode packet */
    mouse.prev_buttons = mouse.buttons;
    mouse.buttons = mouse.packet[0] & 0x07;

    /* X delta with sign extension */
    dx = (int32_t)mouse.packet[1];
    if (mouse.packet[0] & 0x10) {
        dx = dx | 0xFFFFFF00;  /* sign extend negative */
    }

    /* Y delta with sign extension (inverted: PS/2 Y+ = up, screen Y+ = down) */
    dy = (int32_t)mouse.packet[2];
    if (mouse.packet[0] & 0x20) {
        dy = dy | 0xFFFFFF00;
    }
    dy = -dy;  /* invert: PS/2 up → screen down */

    /* Discard overflow packets */
    if (mouse.packet[0] & 0x40) return 0;  /* X overflow */
    if (mouse.packet[0] & 0x80) return 0;  /* Y overflow */

    /* Apply movement with sensitivity */
    mouse.x = mouse.x + dx * mouse.sensitivity;
    mouse.y = mouse.y + dy * mouse.sensitivity;

    /* Clamp to screen bounds */
    if (mouse.x < 0) mouse.x = 0;
    if (mouse.y < 0) mouse.y = 0;
    if (mouse.x > mouse.max_x) mouse.x = mouse.max_x;
    if (mouse.y > mouse.max_y) mouse.y = mouse.max_y;

    return 1;
}

/* ================================================================
 * Button Query Functions
 * ================================================================ */

/* Returns 1 if left button is currently held */
static int mouse_left_down(void)
{
    return (mouse.buttons & 0x01) ? 1 : 0;
}

/* Returns 1 if right button is currently held */
static int mouse_right_down(void)
{
    return (mouse.buttons & 0x02) ? 1 : 0;
}

/* Returns 1 on left click (pressed this frame, not last) */
static int mouse_left_clicked(void)
{
    return ((mouse.buttons & 0x01) && !(mouse.prev_buttons & 0x01)) ? 1 : 0;
}

/* Returns 1 on right click */
static int mouse_right_clicked(void)
{
    return ((mouse.buttons & 0x02) && !(mouse.prev_buttons & 0x02)) ? 1 : 0;
}

/* Returns 1 on left button release */
static int mouse_left_released(void)
{
    return (!(mouse.buttons & 0x01) && (mouse.prev_buttons & 0x01)) ? 1 : 0;
}

/* Get current position */
static void mouse_get_pos(int32_t *out_x, int32_t *out_y)
{
    *out_x = mouse.x;
    *out_y = mouse.y;
}

/* Hit test: is mouse inside rectangle? */
static int mouse_in_rect(int32_t rx, int32_t ry, int32_t rw, int32_t rh)
{
    if (mouse.x >= rx && mouse.x < rx + rw &&
        mouse.y >= ry && mouse.y < ry + rh) {
        return 1;
    }
    return 0;
}
