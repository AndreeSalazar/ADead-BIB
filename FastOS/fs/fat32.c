/*
 * fs/fat32.c — FAT32 Filesystem Driver
 * FastOS v2.0
 *
 * Soporte de lectura/escritura FAT32.
 * Se monta via VFS: vfs_mount("/", fat32_init_root(), FS_TYPE_FAT32)
 *
 * Compilar:  adb cc fs/fat32.c --target fastos
 * Ver steps: adb step fs/fat32.c
 */

#include <kernel.h>
#include <types.h>
#include <fastos.h>

/* ─── Estructuras FAT32 (layout on-disk) ─── */

typedef struct __attribute__((packed)) {
    uint8_t  jump[3];           /* 0xEB 0x58 0x90 */
    char     oem_name[8];       /* "FASTOS  " */
    uint16_t bytes_per_sector;  /* 512 */
    uint8_t  sectors_per_cluster;
    uint16_t reserved_sectors;
    uint8_t  num_fats;          /* 2 */
    uint16_t root_entry_count;  /* 0 en FAT32 */
    uint16_t total_sectors_16;  /* 0 en FAT32 */
    uint8_t  media_type;        /* 0xF8 (disco fijo) */
    uint16_t fat_size_16;       /* 0 en FAT32 */
    uint16_t sectors_per_track;
    uint16_t num_heads;
    uint32_t hidden_sectors;
    uint32_t total_sectors_32;
    /* FAT32 BPB Extension */
    uint32_t fat_size_32;
    uint16_t ext_flags;
    uint16_t fs_version;        /* 0x0000 */
    uint32_t root_cluster;      /* cluster 2 */
    uint16_t fs_info;           /* sector 1 */
    uint16_t backup_boot;       /* sector 6 */
    uint8_t  reserved[12];
    uint8_t  drive_number;
    uint8_t  reserved1;
    uint8_t  boot_sig;          /* 0x29 */
    uint32_t volume_id;
    char     volume_label[11];  /* "FASTOS     " */
    char     fs_type_label[8];  /* "FAT32   " */
} fat32_boot_record_t;

typedef struct __attribute__((packed)) {
    char     name[8];
    char     ext[3];
    uint8_t  attributes;
    uint8_t  reserved;
    uint8_t  creation_time_ms;
    uint16_t creation_time;
    uint16_t creation_date;
    uint16_t last_access_date;
    uint16_t cluster_high;      /* cluster alto (FAT32) */
    uint16_t write_time;
    uint16_t write_date;
    uint16_t cluster_low;       /* cluster bajo */
    uint32_t file_size;
} fat32_dir_entry_t;

#define FAT32_ATTR_READ_ONLY 0x01
#define FAT32_ATTR_HIDDEN    0x02
#define FAT32_ATTR_SYSTEM    0x04
#define FAT32_ATTR_DIRECTORY 0x10
#define FAT32_ATTR_ARCHIVE   0x20

#define FAT32_EOC  0x0FFFFFF8   /* End of Cluster chain */
#define FAT32_FREE 0x00000000

/* ─── Estado del filesystem FAT32 montado ─── */
typedef struct {
    fat32_boot_record_t bpb;
    uint32_t  fat_start_sector;
    uint32_t  data_start_sector;
    uint32_t  root_cluster;
    uint32_t  sectors_per_cluster;
    uint32_t  bytes_per_cluster;
} fat32_fs_t;

static fat32_fs_t fat32_state;

/* ─── Calcular sector del cluster N ─── */
static uint32_t fat32_cluster_to_sector(const fat32_fs_t *fs,
                                         uint32_t cluster) {
    return fs->data_start_sector +
           (cluster - 2) * fs->sectors_per_cluster;
}

/* ─── Leer una entrada de la FAT ─── */
static uint32_t fat32_read_fat_entry(const fat32_fs_t *fs,
                                      uint32_t cluster,
                                      const uint8_t *disk_buf) {
    uint32_t fat_offset = cluster * 4;
    uint32_t fat_sector = fs->fat_start_sector + (fat_offset / 512);
    uint32_t entry_offset = fat_offset % 512;

    /* En producción: leer fat_sector del disco */
    /* Por ahora: demo con disk_buf */
    (void)fat_sector;
    const uint8_t *entry = disk_buf + entry_offset;
    return (*(const uint32_t *)entry) & 0x0FFFFFFF;
}

/* ─── Inicializar FAT32 desde BPB ─── */
int fat32_init(const uint8_t *boot_sector, fat32_fs_t *out_fs) {
    const fat32_boot_record_t *bpb = (const fat32_boot_record_t *)boot_sector;

    /* Verificar firma FAT32 */
    if (bpb->boot_sig != 0x29) return -1;
    if (bpb->root_entry_count != 0) return -1;  /* Debe ser 0 en FAT32 */

    out_fs->bpb = *bpb;
    out_fs->fat_start_sector   = bpb->reserved_sectors;
    out_fs->sectors_per_cluster = bpb->sectors_per_cluster;
    out_fs->bytes_per_cluster  = bpb->bytes_per_sector * bpb->sectors_per_cluster;
    out_fs->root_cluster       = bpb->root_cluster;
    out_fs->data_start_sector  = bpb->reserved_sectors +
                                  (bpb->num_fats * bpb->fat_size_32);

    fat32_state = *out_fs;
    return 0;
}

/* ─── Leer archivo desde cluster chain ─── */
uint64_t fat32_read_file(uint32_t start_cluster, uint64_t file_size,
                          uint8_t *out_buf, const uint8_t *disk) {
    uint32_t cluster = start_cluster;
    uint64_t bytes_read = 0;

    while (cluster < FAT32_EOC && bytes_read < file_size) {
        uint32_t sector = fat32_cluster_to_sector(&fat32_state, cluster);
        uint64_t to_read = fat32_state.bytes_per_cluster;
        if (bytes_read + to_read > file_size) {
            to_read = file_size - bytes_read;
        }

        /* Leer datos del sector → out_buf */
        /* En producción: disk_read(sector, buf) */
        const uint8_t *sector_data = disk + (sector * 512);
        for (uint64_t i = 0; i < to_read; i++) {
            out_buf[bytes_read + i] = sector_data[i];
        }
        bytes_read += to_read;

        /* Siguiente cluster */
        cluster = fat32_read_fat_entry(&fat32_state, cluster, disk);
    }
    return bytes_read;
}
