/*
 * ADead-BIB Standard Library
 * alsa/asoundlib.h - ALSA Sound Library
 * 
 * Based on: ALSA Project (alsa-lib)
 * For FastOS audio support
 */

#ifndef _ADEAD_ALSA_ASOUNDLIB_H
#define _ADEAD_ALSA_ASOUNDLIB_H

#include "../stdint.h"
#include "../stddef.h"

/* ============================================================
 * ALSA Types
 * ============================================================ */

typedef struct _snd_pcm snd_pcm_t;
typedef struct _snd_pcm_hw_params snd_pcm_hw_params_t;
typedef struct _snd_pcm_sw_params snd_pcm_sw_params_t;
typedef struct _snd_pcm_status snd_pcm_status_t;
typedef struct _snd_pcm_info snd_pcm_info_t;

typedef struct _snd_mixer snd_mixer_t;
typedef struct _snd_mixer_elem snd_mixer_elem_t;
typedef struct _snd_mixer_selem_id snd_mixer_selem_id_t;

typedef struct _snd_ctl snd_ctl_t;
typedef struct _snd_ctl_card_info snd_ctl_card_info_t;

typedef unsigned long snd_pcm_uframes_t;
typedef long snd_pcm_sframes_t;

/* ============================================================
 * PCM Stream Types
 * ============================================================ */

typedef enum {
    SND_PCM_STREAM_PLAYBACK = 0,
    SND_PCM_STREAM_CAPTURE = 1
} snd_pcm_stream_t;

typedef enum {
    SND_PCM_ACCESS_MMAP_INTERLEAVED = 0,
    SND_PCM_ACCESS_MMAP_NONINTERLEAVED = 1,
    SND_PCM_ACCESS_MMAP_COMPLEX = 2,
    SND_PCM_ACCESS_RW_INTERLEAVED = 3,
    SND_PCM_ACCESS_RW_NONINTERLEAVED = 4
} snd_pcm_access_t;

typedef enum {
    SND_PCM_FORMAT_UNKNOWN = -1,
    SND_PCM_FORMAT_S8 = 0,
    SND_PCM_FORMAT_U8 = 1,
    SND_PCM_FORMAT_S16_LE = 2,
    SND_PCM_FORMAT_S16_BE = 3,
    SND_PCM_FORMAT_U16_LE = 4,
    SND_PCM_FORMAT_U16_BE = 5,
    SND_PCM_FORMAT_S24_LE = 6,
    SND_PCM_FORMAT_S24_BE = 7,
    SND_PCM_FORMAT_U24_LE = 8,
    SND_PCM_FORMAT_U24_BE = 9,
    SND_PCM_FORMAT_S32_LE = 10,
    SND_PCM_FORMAT_S32_BE = 11,
    SND_PCM_FORMAT_U32_LE = 12,
    SND_PCM_FORMAT_U32_BE = 13,
    SND_PCM_FORMAT_FLOAT_LE = 14,
    SND_PCM_FORMAT_FLOAT_BE = 15,
    SND_PCM_FORMAT_FLOAT64_LE = 16,
    SND_PCM_FORMAT_FLOAT64_BE = 17,
    SND_PCM_FORMAT_IEC958_SUBFRAME_LE = 18,
    SND_PCM_FORMAT_IEC958_SUBFRAME_BE = 19,
    SND_PCM_FORMAT_MU_LAW = 20,
    SND_PCM_FORMAT_A_LAW = 21,
    SND_PCM_FORMAT_IMA_ADPCM = 22,
    SND_PCM_FORMAT_MPEG = 23,
    SND_PCM_FORMAT_GSM = 24,
    SND_PCM_FORMAT_S24_3LE = 32,
    SND_PCM_FORMAT_S24_3BE = 33,
    SND_PCM_FORMAT_U24_3LE = 34,
    SND_PCM_FORMAT_U24_3BE = 35,
    SND_PCM_FORMAT_S20_3LE = 36,
    SND_PCM_FORMAT_S20_3BE = 37,
    SND_PCM_FORMAT_U20_3LE = 38,
    SND_PCM_FORMAT_U20_3BE = 39,
    SND_PCM_FORMAT_S18_3LE = 40,
    SND_PCM_FORMAT_S18_3BE = 41,
    SND_PCM_FORMAT_U18_3LE = 42,
    SND_PCM_FORMAT_U18_3BE = 43
} snd_pcm_format_t;

/* Native endian aliases */
#if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
#define SND_PCM_FORMAT_S16      SND_PCM_FORMAT_S16_LE
#define SND_PCM_FORMAT_U16      SND_PCM_FORMAT_U16_LE
#define SND_PCM_FORMAT_S24      SND_PCM_FORMAT_S24_LE
#define SND_PCM_FORMAT_U24      SND_PCM_FORMAT_U24_LE
#define SND_PCM_FORMAT_S32      SND_PCM_FORMAT_S32_LE
#define SND_PCM_FORMAT_U32      SND_PCM_FORMAT_U32_LE
#define SND_PCM_FORMAT_FLOAT    SND_PCM_FORMAT_FLOAT_LE
#define SND_PCM_FORMAT_FLOAT64  SND_PCM_FORMAT_FLOAT64_LE
#else
#define SND_PCM_FORMAT_S16      SND_PCM_FORMAT_S16_BE
#define SND_PCM_FORMAT_U16      SND_PCM_FORMAT_U16_BE
#define SND_PCM_FORMAT_S24      SND_PCM_FORMAT_S24_BE
#define SND_PCM_FORMAT_U24      SND_PCM_FORMAT_U24_BE
#define SND_PCM_FORMAT_S32      SND_PCM_FORMAT_S32_BE
#define SND_PCM_FORMAT_U32      SND_PCM_FORMAT_U32_BE
#define SND_PCM_FORMAT_FLOAT    SND_PCM_FORMAT_FLOAT_BE
#define SND_PCM_FORMAT_FLOAT64  SND_PCM_FORMAT_FLOAT64_BE
#endif

typedef enum {
    SND_PCM_STATE_OPEN = 0,
    SND_PCM_STATE_SETUP = 1,
    SND_PCM_STATE_PREPARED = 2,
    SND_PCM_STATE_RUNNING = 3,
    SND_PCM_STATE_XRUN = 4,
    SND_PCM_STATE_DRAINING = 5,
    SND_PCM_STATE_PAUSED = 6,
    SND_PCM_STATE_SUSPENDED = 7,
    SND_PCM_STATE_DISCONNECTED = 8
} snd_pcm_state_t;

/* ============================================================
 * PCM Functions
 * ============================================================ */

/* Open/Close */
int snd_pcm_open(snd_pcm_t** pcm, const char* name, snd_pcm_stream_t stream, int mode);
int snd_pcm_close(snd_pcm_t* pcm);

/* Hardware Parameters */
int snd_pcm_hw_params_malloc(snd_pcm_hw_params_t** ptr);
void snd_pcm_hw_params_free(snd_pcm_hw_params_t* obj);
int snd_pcm_hw_params_any(snd_pcm_t* pcm, snd_pcm_hw_params_t* params);
int snd_pcm_hw_params(snd_pcm_t* pcm, snd_pcm_hw_params_t* params);

int snd_pcm_hw_params_set_access(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, snd_pcm_access_t access);
int snd_pcm_hw_params_set_format(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, snd_pcm_format_t format);
int snd_pcm_hw_params_set_channels(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, unsigned int val);
int snd_pcm_hw_params_set_rate(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, unsigned int val, int dir);
int snd_pcm_hw_params_set_rate_near(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, unsigned int* val, int* dir);
int snd_pcm_hw_params_set_buffer_size(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, snd_pcm_uframes_t val);
int snd_pcm_hw_params_set_buffer_size_near(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, snd_pcm_uframes_t* val);
int snd_pcm_hw_params_set_period_size(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, snd_pcm_uframes_t val, int dir);
int snd_pcm_hw_params_set_period_size_near(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, snd_pcm_uframes_t* val, int* dir);
int snd_pcm_hw_params_set_periods(snd_pcm_t* pcm, snd_pcm_hw_params_t* params, unsigned int val, int dir);

int snd_pcm_hw_params_get_channels(const snd_pcm_hw_params_t* params, unsigned int* val);
int snd_pcm_hw_params_get_rate(const snd_pcm_hw_params_t* params, unsigned int* val, int* dir);
int snd_pcm_hw_params_get_buffer_size(const snd_pcm_hw_params_t* params, snd_pcm_uframes_t* val);
int snd_pcm_hw_params_get_period_size(const snd_pcm_hw_params_t* params, snd_pcm_uframes_t* val, int* dir);

/* Software Parameters */
int snd_pcm_sw_params_malloc(snd_pcm_sw_params_t** ptr);
void snd_pcm_sw_params_free(snd_pcm_sw_params_t* obj);
int snd_pcm_sw_params_current(snd_pcm_t* pcm, snd_pcm_sw_params_t* params);
int snd_pcm_sw_params(snd_pcm_t* pcm, snd_pcm_sw_params_t* params);
int snd_pcm_sw_params_set_start_threshold(snd_pcm_t* pcm, snd_pcm_sw_params_t* params, snd_pcm_uframes_t val);
int snd_pcm_sw_params_set_avail_min(snd_pcm_t* pcm, snd_pcm_sw_params_t* params, snd_pcm_uframes_t val);

/* Prepare/Start/Stop */
int snd_pcm_prepare(snd_pcm_t* pcm);
int snd_pcm_start(snd_pcm_t* pcm);
int snd_pcm_drop(snd_pcm_t* pcm);
int snd_pcm_drain(snd_pcm_t* pcm);
int snd_pcm_pause(snd_pcm_t* pcm, int enable);
int snd_pcm_reset(snd_pcm_t* pcm);
int snd_pcm_resume(snd_pcm_t* pcm);

/* State */
snd_pcm_state_t snd_pcm_state(snd_pcm_t* pcm);
int snd_pcm_avail(snd_pcm_t* pcm);
int snd_pcm_avail_update(snd_pcm_t* pcm);
int snd_pcm_delay(snd_pcm_t* pcm, snd_pcm_sframes_t* delayp);

/* Read/Write */
snd_pcm_sframes_t snd_pcm_writei(snd_pcm_t* pcm, const void* buffer, snd_pcm_uframes_t size);
snd_pcm_sframes_t snd_pcm_readi(snd_pcm_t* pcm, void* buffer, snd_pcm_uframes_t size);
snd_pcm_sframes_t snd_pcm_writen(snd_pcm_t* pcm, void** bufs, snd_pcm_uframes_t size);
snd_pcm_sframes_t snd_pcm_readn(snd_pcm_t* pcm, void** bufs, snd_pcm_uframes_t size);

/* MMAP */
int snd_pcm_mmap_begin(snd_pcm_t* pcm, const void** areas, snd_pcm_uframes_t* offset, snd_pcm_uframes_t* frames);
snd_pcm_sframes_t snd_pcm_mmap_commit(snd_pcm_t* pcm, snd_pcm_uframes_t offset, snd_pcm_uframes_t frames);

/* Wait */
int snd_pcm_wait(snd_pcm_t* pcm, int timeout);

/* Recovery */
int snd_pcm_recover(snd_pcm_t* pcm, int err, int silent);

/* Info */
const char* snd_pcm_name(snd_pcm_t* pcm);
snd_pcm_stream_t snd_pcm_stream(snd_pcm_t* pcm);

/* ============================================================
 * Mixer Functions
 * ============================================================ */

int snd_mixer_open(snd_mixer_t** mixer, int mode);
int snd_mixer_close(snd_mixer_t* mixer);
int snd_mixer_attach(snd_mixer_t* mixer, const char* name);
int snd_mixer_detach(snd_mixer_t* mixer, const char* name);
int snd_mixer_load(snd_mixer_t* mixer);
int snd_mixer_free(snd_mixer_t* mixer);

snd_mixer_elem_t* snd_mixer_first_elem(snd_mixer_t* mixer);
snd_mixer_elem_t* snd_mixer_last_elem(snd_mixer_t* mixer);
snd_mixer_elem_t* snd_mixer_elem_next(snd_mixer_elem_t* elem);
snd_mixer_elem_t* snd_mixer_elem_prev(snd_mixer_elem_t* elem);

int snd_mixer_selem_register(snd_mixer_t* mixer, void* options, void** classp);
int snd_mixer_selem_id_malloc(snd_mixer_selem_id_t** ptr);
void snd_mixer_selem_id_free(snd_mixer_selem_id_t* obj);
void snd_mixer_selem_id_set_index(snd_mixer_selem_id_t* obj, unsigned int val);
void snd_mixer_selem_id_set_name(snd_mixer_selem_id_t* obj, const char* val);
snd_mixer_elem_t* snd_mixer_find_selem(snd_mixer_t* mixer, const snd_mixer_selem_id_t* id);

int snd_mixer_selem_get_playback_volume(snd_mixer_elem_t* elem, int channel, long* value);
int snd_mixer_selem_set_playback_volume(snd_mixer_elem_t* elem, int channel, long value);
int snd_mixer_selem_set_playback_volume_all(snd_mixer_elem_t* elem, long value);
int snd_mixer_selem_get_playback_volume_range(snd_mixer_elem_t* elem, long* min, long* max);
int snd_mixer_selem_get_playback_switch(snd_mixer_elem_t* elem, int channel, int* value);
int snd_mixer_selem_set_playback_switch(snd_mixer_elem_t* elem, int channel, int value);
int snd_mixer_selem_set_playback_switch_all(snd_mixer_elem_t* elem, int value);

int snd_mixer_selem_get_capture_volume(snd_mixer_elem_t* elem, int channel, long* value);
int snd_mixer_selem_set_capture_volume(snd_mixer_elem_t* elem, int channel, long value);
int snd_mixer_selem_set_capture_volume_all(snd_mixer_elem_t* elem, long value);
int snd_mixer_selem_get_capture_volume_range(snd_mixer_elem_t* elem, long* min, long* max);
int snd_mixer_selem_get_capture_switch(snd_mixer_elem_t* elem, int channel, int* value);
int snd_mixer_selem_set_capture_switch(snd_mixer_elem_t* elem, int channel, int value);
int snd_mixer_selem_set_capture_switch_all(snd_mixer_elem_t* elem, int value);

/* Mixer channels */
typedef enum {
    SND_MIXER_SCHN_UNKNOWN = -1,
    SND_MIXER_SCHN_FRONT_LEFT = 0,
    SND_MIXER_SCHN_FRONT_RIGHT = 1,
    SND_MIXER_SCHN_REAR_LEFT = 2,
    SND_MIXER_SCHN_REAR_RIGHT = 3,
    SND_MIXER_SCHN_FRONT_CENTER = 4,
    SND_MIXER_SCHN_WOOFER = 5,
    SND_MIXER_SCHN_SIDE_LEFT = 6,
    SND_MIXER_SCHN_SIDE_RIGHT = 7,
    SND_MIXER_SCHN_REAR_CENTER = 8,
    SND_MIXER_SCHN_MONO = SND_MIXER_SCHN_FRONT_LEFT
} snd_mixer_selem_channel_id_t;

/* ============================================================
 * Control Functions
 * ============================================================ */

int snd_ctl_open(snd_ctl_t** ctl, const char* name, int mode);
int snd_ctl_close(snd_ctl_t* ctl);
int snd_ctl_card_info(snd_ctl_t* ctl, snd_ctl_card_info_t* info);
int snd_ctl_card_info_malloc(snd_ctl_card_info_t** ptr);
void snd_ctl_card_info_free(snd_ctl_card_info_t* obj);
const char* snd_ctl_card_info_get_id(const snd_ctl_card_info_t* obj);
const char* snd_ctl_card_info_get_name(const snd_ctl_card_info_t* obj);
const char* snd_ctl_card_info_get_longname(const snd_ctl_card_info_t* obj);
const char* snd_ctl_card_info_get_driver(const snd_ctl_card_info_t* obj);

int snd_card_next(int* card);
int snd_card_get_name(int card, char** name);
int snd_card_get_longname(int card, char** name);

/* ============================================================
 * Error Handling
 * ============================================================ */

const char* snd_strerror(int errnum);

/* Error codes */
#define SND_ERROR_BEGIN         500000
#define SND_ERROR_INCOMPATIBLE_VERSION (SND_ERROR_BEGIN + 0)
#define SND_ERROR_ALISP_NIL     (SND_ERROR_BEGIN + 1)

/* ============================================================
 * Format Helpers
 * ============================================================ */

int snd_pcm_format_width(snd_pcm_format_t format);
int snd_pcm_format_physical_width(snd_pcm_format_t format);
int snd_pcm_format_size(snd_pcm_format_t format, size_t samples);
const char* snd_pcm_format_name(snd_pcm_format_t format);
const char* snd_pcm_format_description(snd_pcm_format_t format);
snd_pcm_format_t snd_pcm_format_value(const char* name);

int snd_pcm_format_signed(snd_pcm_format_t format);
int snd_pcm_format_unsigned(snd_pcm_format_t format);
int snd_pcm_format_linear(snd_pcm_format_t format);
int snd_pcm_format_float(snd_pcm_format_t format);
int snd_pcm_format_little_endian(snd_pcm_format_t format);
int snd_pcm_format_big_endian(snd_pcm_format_t format);
int snd_pcm_format_cpu_endian(snd_pcm_format_t format);

/* ============================================================
 * Open Modes
 * ============================================================ */

#define SND_PCM_NONBLOCK        0x00000001
#define SND_PCM_ASYNC           0x00000002
#define SND_PCM_NO_AUTO_RESAMPLE 0x00010000
#define SND_PCM_NO_AUTO_CHANNELS 0x00020000
#define SND_PCM_NO_AUTO_FORMAT  0x00040000
#define SND_PCM_NO_SOFTVOL      0x00080000

#endif /* _ADEAD_ALSA_ASOUNDLIB_H */
