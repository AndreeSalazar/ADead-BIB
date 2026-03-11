/*
 * fs/ext2.c — EXT2 Filesystem Driver
 * FastOS v2.0
 *
 * Soporte de lectura EXT2 (Linux). Permite leer discos Linux en FastOS.
 * Montado via VFS: vfs_mount("/linux", ext2_init_root(), FS_TYPE_EXT2)
 *
 * Compilar:  adb cc fs/ext2.c --target fastos
 * Ver steps: adb step fs/ext2.c
 */

#include <kernel.h>
#include <types.h>
#include <fastos.h>

/* ─── EXT2 Superblock (offset 1024 en el disco) ─── */
typedef struct __attribute__((packed)) {
    uint32_t s_inodes_count;
    uint32_t s_blocks_count;
    uint32_t s_r_blocks_count;
    uint32_t s_free_blocks_count;
    uint32_t s_free_inodes_count;
    uint32_t s_first_data_block;  /* 0 si block_size > 1024, 1 si = 1024 */
    uint32_t s_log_block_size;    /* block_size = 1024 << s_log_block_size */
    uint32_t s_log_frag_size;
    uint32_t s_blocks_per_group;
    uint32_t s_frags_per_group;
    uint32_t s_inodes_per_group;
    uint32_t s_mtime;
    uint32_t s_wtime;
    uint16_t s_mnt_count;
    uint16_t s_max_mnt_count;
    uint16_t s_magic;             /* 0xEF53 */
    uint16_t s_state;
    uint16_t s_errors;
    uint16_t s_minor_rev_level;
    uint32_t s_lastcheck;
    uint32_t s_checkinterval;
    uint32_t s_creator_os;
    uint32_t s_rev_level;
    uint16_t s_def_resuid;
    uint16_t s_def_resgid;
    /* EXT2_DYNAMIC_REV */
    uint32_t s_first_ino;         /* primer inode no reservado */
    uint16_t s_inode_size;        /* tamaño del inode en bytes */
    uint16_t s_block_group_nr;
    uint32_t s_feature_compat;
    uint32_t s_feature_incompat;
    uint32_t s_feature_ro_compat;
    uint8_t  s_uuid[16];
    char     s_volume_name[16];
    char     s_last_mounted[64];
} ext2_superblock_t;

#define EXT2_MAGIC 0xEF53

/* ─── EXT2 Inode ─── */
typedef struct __attribute__((packed)) {
    uint16_t i_mode;        /* tipo + permisos */
    uint16_t i_uid;
    uint32_t i_size;
    uint32_t i_atime;
    uint32_t i_ctime;
    uint32_t i_mtime;
    uint32_t i_dtime;
    uint16_t i_gid;
    uint16_t i_links_count;
    uint32_t i_blocks;      /* en bloques de 512 bytes */
    uint32_t i_flags;
    uint32_t i_osd1;
    uint32_t i_block[15];   /* 12 directos + 1 indir + 1 doble + 1 triple */
    uint32_t i_generation;
    uint32_t i_file_acl;
    uint32_t i_dir_acl;
    uint32_t i_faddr;
} ext2_inode_t;

#define EXT2_S_IFREG 0x8000  /* archivo regular */
#define EXT2_S_IFDIR 0x4000  /* directorio */

/* ─── Group Descriptor ─── */
typedef struct __attribute__((packed)) {
    uint32_t bg_block_bitmap;
    uint32_t bg_inode_bitmap;
    uint32_t bg_inode_table;
    uint16_t bg_free_blocks_count;
    uint16_t bg_free_inodes_count;
    uint16_t bg_used_dirs_count;
    uint16_t bg_pad;
    uint8_t  bg_reserved[12];
} ext2_group_desc_t;

/* ─── Directory Entry ─── */
typedef struct __attribute__((packed)) {
    uint32_t inode;
    uint16_t rec_len;
    uint8_t  name_len;
    uint8_t  file_type;
    char     name[255];
} ext2_dir_entry_t;

/* ─── Estado EXT2 ─── */
typedef struct {
    ext2_superblock_t sb;
    uint32_t block_size;        /* bytes por bloque */
    uint32_t inodes_per_group;
    uint32_t inode_size;
} ext2_fs_t;

static ext2_fs_t ext2_state;

/* ─── Inicializar desde superblock ─── */
int ext2_init(const uint8_t *disk, ext2_fs_t *out) {
    /* Superblock está en el offset 1024 siempre */
    const ext2_superblock_t *sb =
        (const ext2_superblock_t *)(disk + 1024);

    if (sb->s_magic != EXT2_MAGIC) return -1;

    out->sb              = *sb;
    out->block_size      = 1024u << sb->s_log_block_size;
    out->inodes_per_group = sb->s_inodes_per_group;
    out->inode_size      = (sb->s_rev_level >= 1) ? sb->s_inode_size : 128u;

    ext2_state = *out;
    return 0;
}

/* ─── Leer inode N ─── */
static ext2_inode_t ext2_read_inode(uint32_t inode_num,
                                     const uint8_t *disk) {
    uint32_t group = (inode_num - 1) / ext2_state.inodes_per_group;
    uint32_t idx   = (inode_num - 1) % ext2_state.inodes_per_group;

    /* Group descriptor está justo después del superblock */
    uint32_t gd_block = (ext2_state.block_size == 1024) ? 2 : 1;
    const ext2_group_desc_t *gd =
        (const ext2_group_desc_t *)(disk + gd_block * ext2_state.block_size
                                    + group * sizeof(ext2_group_desc_t));

    uint32_t inode_offset = gd->bg_inode_table * ext2_state.block_size
                          + idx * ext2_state.inode_size;
    return *(const ext2_inode_t *)(disk + inode_offset);
}

/* ─── Leer bloque de datos ─── */
static const uint8_t *ext2_block_ptr(uint32_t block_num,
                                      const uint8_t *disk) {
    return disk + block_num * ext2_state.block_size;
}

/* ─── Leer archivo dado inode ─── */
uint64_t ext2_read_file(uint32_t inode_num, uint8_t *out_buf,
                         uint64_t max_size, const uint8_t *disk) {
    ext2_inode_t inode = ext2_read_inode(inode_num, disk);
    uint64_t file_size = inode.i_size;
    if (file_size > max_size) file_size = max_size;

    uint64_t bytes_read = 0;

    /* Solo bloques directos (12 bloques × block_size) */
    for (int i = 0; i < 12 && bytes_read < file_size; i++) {
        if (inode.i_block[i] == 0) break;
        const uint8_t *block = ext2_block_ptr(inode.i_block[i], disk);
        uint64_t to_read = ext2_state.block_size;
        if (bytes_read + to_read > file_size)
            to_read = file_size - bytes_read;
        for (uint64_t j = 0; j < to_read; j++) {
            out_buf[bytes_read + j] = block[j];
        }
        bytes_read += to_read;
    }
    return bytes_read;
}

/* ─── Buscar archivo en directorio ─── */
uint32_t ext2_find_in_dir(uint32_t dir_inode, const char *name,
                           const uint8_t *disk) {
    ext2_inode_t inode = ext2_read_inode(dir_inode, disk);

    for (int i = 0; i < 12; i++) {
        if (inode.i_block[i] == 0) break;
        const uint8_t *block = ext2_block_ptr(inode.i_block[i], disk);
        uint32_t offset = 0;

        while (offset < ext2_state.block_size) {
            const ext2_dir_entry_t *entry =
                (const ext2_dir_entry_t *)(block + offset);
            if (entry->inode == 0) break;
            if (entry->rec_len == 0) break;

            /* Comparar nombre */
            uint8_t nlen = entry->name_len;
            int match = 1;
            for (uint8_t k = 0; k < nlen; k++) {
                if (entry->name[k] != name[k]) { match = 0; break; }
            }
            if (match && name[nlen] == '\0') return entry->inode;

            offset += entry->rec_len;
        }
    }
    return 0; /* no encontrado */
}
