/*
 * FastOS v2.0 — Programmable Interval Timer (PIT)
 * Intel 8253/8254 Timer Driver
 * 
 * Compile: adB cc pit.c -o pit.po --driver
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* ============================================================
 * PIT Constants
 * ============================================================ */

#define PIT_CHANNEL0    0x40
#define PIT_CHANNEL1    0x41
#define PIT_CHANNEL2    0x42
#define PIT_COMMAND     0x43

/* PIT base frequency: 1.193182 MHz */
#define PIT_FREQUENCY   1193182

/* Command byte bits */
#define PIT_CMD_CHANNEL0    0x00
#define PIT_CMD_CHANNEL1    0x40
#define PIT_CMD_CHANNEL2    0x80
#define PIT_CMD_LATCH       0x00
#define PIT_CMD_LOBYTE      0x10
#define PIT_CMD_HIBYTE      0x20
#define PIT_CMD_LOHI        0x30
#define PIT_CMD_MODE0       0x00    /* Interrupt on terminal count */
#define PIT_CMD_MODE1       0x02    /* Hardware retriggerable one-shot */
#define PIT_CMD_MODE2       0x04    /* Rate generator */
#define PIT_CMD_MODE3       0x06    /* Square wave generator */
#define PIT_CMD_MODE4       0x08    /* Software triggered strobe */
#define PIT_CMD_MODE5       0x0A    /* Hardware triggered strobe */
#define PIT_CMD_BINARY      0x00
#define PIT_CMD_BCD         0x01

/* ============================================================
 * Timer State
 * ============================================================ */

typedef struct {
    uint64_t ticks;             /* Total ticks since boot */
    uint64_t seconds;           /* Seconds since boot */
    uint32_t frequency;         /* Timer frequency in Hz */
    uint32_t ms_per_tick;       /* Milliseconds per tick */
    
    /* Time of day */
    uint8_t hours;
    uint8_t minutes;
    uint8_t seconds_tod;
    
    /* Callbacks */
    void (*tick_callback)(uint64_t ticks);
    void (*second_callback)(uint64_t seconds);
} timer_state_t;

static timer_state_t timer;

/* ============================================================
 * RTC (Real Time Clock) for initial time
 * ============================================================ */

#define RTC_ADDRESS     0x70
#define RTC_DATA        0x71

#define RTC_SECONDS     0x00
#define RTC_MINUTES     0x02
#define RTC_HOURS       0x04
#define RTC_DAY         0x07
#define RTC_MONTH       0x08
#define RTC_YEAR        0x09
#define RTC_STATUS_A    0x0A
#define RTC_STATUS_B    0x0B

static uint8_t rtc_read(uint8_t reg) {
    outb(RTC_ADDRESS, reg);
    return inb(RTC_DATA);
}

static uint8_t bcd_to_binary(uint8_t bcd) {
    return ((bcd >> 4) * 10) + (bcd & 0x0F);
}

static void rtc_get_time(uint8_t *hours, uint8_t *minutes, uint8_t *seconds) {
    /* Wait for RTC update to complete */
    while (rtc_read(RTC_STATUS_A) & 0x80);
    
    uint8_t status_b = rtc_read(RTC_STATUS_B);
    int is_bcd = !(status_b & 0x04);
    int is_24h = status_b & 0x02;
    
    *seconds = rtc_read(RTC_SECONDS);
    *minutes = rtc_read(RTC_MINUTES);
    *hours = rtc_read(RTC_HOURS);
    
    if (is_bcd) {
        *seconds = bcd_to_binary(*seconds);
        *minutes = bcd_to_binary(*minutes);
        *hours = bcd_to_binary(*hours & 0x7F);
    }
    
    /* Convert 12h to 24h if needed */
    if (!is_24h && (*hours & 0x80)) {
        *hours = ((*hours & 0x7F) + 12) % 24;
    }
}

/* ============================================================
 * PIT Functions
 * ============================================================ */

void pit_set_frequency(uint32_t frequency) {
    if (frequency < 19) frequency = 19;      /* Minimum ~18.2 Hz */
    if (frequency > 1193182) frequency = 1193182;
    
    uint16_t divisor = PIT_FREQUENCY / frequency;
    
    /* Configure channel 0 for rate generator */
    outb(PIT_COMMAND, PIT_CMD_CHANNEL0 | PIT_CMD_LOHI | PIT_CMD_MODE2 | PIT_CMD_BINARY);
    outb(PIT_CHANNEL0, divisor & 0xFF);
    outb(PIT_CHANNEL0, (divisor >> 8) & 0xFF);
    
    timer.frequency = PIT_FREQUENCY / divisor;
    timer.ms_per_tick = 1000 / timer.frequency;
    
    kprintf("[PIT] Frequency set to %d Hz (divisor=%d)\n", timer.frequency, divisor);
}

uint16_t pit_read_count(void) {
    outb(PIT_COMMAND, PIT_CMD_CHANNEL0 | PIT_CMD_LATCH);
    uint8_t lo = inb(PIT_CHANNEL0);
    uint8_t hi = inb(PIT_CHANNEL0);
    return (hi << 8) | lo;
}

/* ============================================================
 * Timer IRQ Handler
 * ============================================================ */

void timer_handle_irq(void) {
    timer.ticks++;
    
    /* Update time of day */
    if (timer.ticks % timer.frequency == 0) {
        timer.seconds++;
        timer.seconds_tod++;
        
        if (timer.seconds_tod >= 60) {
            timer.seconds_tod = 0;
            timer.minutes++;
            
            if (timer.minutes >= 60) {
                timer.minutes = 0;
                timer.hours++;
                
                if (timer.hours >= 24) {
                    timer.hours = 0;
                }
            }
        }
        
        if (timer.second_callback) {
            timer.second_callback(timer.seconds);
        }
    }
    
    if (timer.tick_callback) {
        timer.tick_callback(timer.ticks);
    }
}

/* ============================================================
 * Timer API
 * ============================================================ */

uint64_t timer_get_ticks(void) {
    return timer.ticks;
}

uint64_t timer_get_seconds(void) {
    return timer.seconds;
}

uint64_t timer_get_ms(void) {
    return timer.ticks * timer.ms_per_tick;
}

void timer_get_time(uint8_t *hours, uint8_t *minutes, uint8_t *seconds) {
    *hours = timer.hours;
    *minutes = timer.minutes;
    *seconds = timer.seconds_tod;
}

void timer_get_time_string(char *buf, size_t size) {
    if (size < 9) return;
    
    buf[0] = '0' + (timer.hours / 10);
    buf[1] = '0' + (timer.hours % 10);
    buf[2] = ':';
    buf[3] = '0' + (timer.minutes / 10);
    buf[4] = '0' + (timer.minutes % 10);
    buf[5] = ':';
    buf[6] = '0' + (timer.seconds_tod / 10);
    buf[7] = '0' + (timer.seconds_tod % 10);
    buf[8] = '\0';
}

void timer_sleep_ms(uint32_t ms) {
    uint64_t target = timer_get_ms() + ms;
    while (timer_get_ms() < target) {
        __asm__ volatile("hlt");
    }
}

void timer_sleep_ticks(uint32_t ticks) {
    uint64_t target = timer.ticks + ticks;
    while (timer.ticks < target) {
        __asm__ volatile("hlt");
    }
}

void timer_set_tick_callback(void (*callback)(uint64_t)) {
    timer.tick_callback = callback;
}

void timer_set_second_callback(void (*callback)(uint64_t)) {
    timer.second_callback = callback;
}

/* ============================================================
 * PC Speaker (uses PIT channel 2)
 * ============================================================ */

#define SPEAKER_PORT    0x61

void speaker_beep(uint32_t frequency, uint32_t duration_ms) {
    if (frequency == 0) return;
    
    uint16_t divisor = PIT_FREQUENCY / frequency;
    
    /* Configure channel 2 */
    outb(PIT_COMMAND, PIT_CMD_CHANNEL2 | PIT_CMD_LOHI | PIT_CMD_MODE3 | PIT_CMD_BINARY);
    outb(PIT_CHANNEL2, divisor & 0xFF);
    outb(PIT_CHANNEL2, (divisor >> 8) & 0xFF);
    
    /* Enable speaker */
    uint8_t tmp = inb(SPEAKER_PORT);
    outb(SPEAKER_PORT, tmp | 0x03);
    
    /* Wait */
    timer_sleep_ms(duration_ms);
    
    /* Disable speaker */
    outb(SPEAKER_PORT, tmp & ~0x03);
}

/* ============================================================
 * Initialization
 * ============================================================ */

int timer_init(uint32_t frequency) {
    kmemset(&timer, 0, sizeof(timer));
    
    /* Get initial time from RTC */
    rtc_get_time(&timer.hours, &timer.minutes, &timer.seconds_tod);
    
    /* Set PIT frequency */
    pit_set_frequency(frequency);
    
    kprintf("[PIT] Timer initialized\n");
    kprintf("[PIT] Current time: %02d:%02d:%02d\n", 
            timer.hours, timer.minutes, timer.seconds_tod);
    
    return 0;
}
