/*
 * fs/vfs.c — Virtual File System
 * FastOS v2.0
 *
 * VFS unifica FAT32, EXT2 y el formato .Po nativo bajo una interfaz única.
 * Ningún proceso accede al hardware de disco directo — todo pasa por VFS.
 *
 * Compilar con ADead-BIB:
 *   adb cc fs/vfs.c -o fs_vfs.bin --target fastos
 *   adb step fs/vfs.c   ← ver pipeline completo de 7 fases
 */

#include <kernel.h>
#include <types.h>
#include <fastos.h>

/* ─── Tipos de Sistema de Archivos Soportados ─── */
typedef enum {
    FS_TYPE_NONE   = 0,
    FS_TYPE_FAT32  = 1,
    FS_TYPE_EXT2   = 2,
    FS_TYPE_FASTOS = 3   /* .Po nativo */
} fs_type_t;

/* ─── Nodo VFS (Archivo o Directorio) ─── */
typedef struct vfs_node {
    char          name[256];
    uint32_t      flags;       /* FS_FILE | FS_DIRECTORY */
    uint32_t      inode;
    uint64_t      size;
    fs_type_t     fs_type;

    /* Operaciones — cada FS implementa estas funciones */
    uint64_t (*read) (struct vfs_node *node, uint64_t offset,
                      uint64_t size, uint8_t *buf);
    uint64_t (*write)(struct vfs_node *node, uint64_t offset,
                      uint64_t size, const uint8_t *buf);
    struct vfs_node *(*finddir)(struct vfs_node *node, const char *name);
    int       (*mkdir)(struct vfs_node *node, const char *name, uint32_t perm);
} vfs_node_t;

#define FS_FILE       0x01
#define FS_DIRECTORY  0x02
#define FS_MOUNTPOINT 0x08

/* ─── Tabla de Montajes ─── */
#define VFS_MAX_MOUNTS 16

typedef struct {
    char       mountpoint[256];  /* ej: "/", "/disk", "/boot" */
    vfs_node_t *root;
    fs_type_t   type;
    int         active;
} vfs_mount_t;

static vfs_mount_t vfs_mounts[VFS_MAX_MOUNTS];
static vfs_node_t *vfs_root = 0;

/* ─── Inicialización ─── */
void vfs_init(void) {
    for (int i = 0; i < VFS_MAX_MOUNTS; i++) {
        vfs_mounts[i].active = 0;
        vfs_mounts[i].root   = 0;
    }
    vfs_root = 0;
}

/* ─── Montar un sistema de archivos ─── */
int vfs_mount(const char *path, vfs_node_t *root_node, fs_type_t type) {
    for (int i = 0; i < VFS_MAX_MOUNTS; i++) {
        if (!vfs_mounts[i].active) {
            /* Copiar path */
            int j = 0;
            while (path[j] && j < 255) {
                vfs_mounts[i].mountpoint[j] = path[j];
                j++;
            }
            vfs_mounts[i].mountpoint[j] = '\0';
            vfs_mounts[i].root   = root_node;
            vfs_mounts[i].type   = type;
            vfs_mounts[i].active = 1;

            /* Si es "/", es el root del VFS */
            if (path[0] == '/' && path[1] == '\0') {
                vfs_root = root_node;
            }
            return 0; /* OK */
        }
    }
    return -1; /* Sin espacio en tabla de montajes */
}

/* ─── Leer un archivo ─── */
uint64_t vfs_read(vfs_node_t *node, uint64_t offset,
                  uint64_t size, uint8_t *buf) {
    if (!node || !node->read) return 0;
    return node->read(node, offset, size, buf);
}

/* ─── Escribir en un archivo ─── */
uint64_t vfs_write(vfs_node_t *node, uint64_t offset,
                   uint64_t size, const uint8_t *buf) {
    if (!node || !node->write) return 0;
    return node->write(node, offset, size, buf);
}

/* ─── Buscar un archivo/directorio por path ─── */
vfs_node_t *vfs_findpath(const char *path) {
    if (!vfs_root || !path || path[0] != '/') return 0;
    if (path[1] == '\0') return vfs_root;

    /* Buscar en el filesystem montado en "/" */
    if (vfs_root->finddir) {
        /* Simplificado: un nivel de profundidad */
        const char *name = path + 1; /* saltar '/' inicial */
        return vfs_root->finddir(vfs_root, name);
    }
    return 0;
}

/* ─── Crear directorio ─── */
int vfs_mkdir(const char *path, uint32_t permissions) {
    /* Encontrar el directorio padre */
    vfs_node_t *parent = vfs_root;
    if (!parent || !parent->mkdir) return -1;
    return parent->mkdir(parent, path, permissions);
}
