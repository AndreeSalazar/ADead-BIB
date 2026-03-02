/*
 * FastOS v2.0 — Virtual File System (VFS)
 * Abstraction layer for multiple filesystems
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ============================================================
 * File Types and Flags
 * ============================================================ */

#define VFS_FILE      0x01
#define VFS_DIRECTORY 0x02
#define VFS_CHARDEV   0x03
#define VFS_BLOCKDEV  0x04
#define VFS_PIPE      0x05
#define VFS_SYMLINK   0x06
#define VFS_MOUNTPOINT 0x08

#define O_RDONLY  0x0000
#define O_WRONLY  0x0001
#define O_RDWR    0x0002
#define O_CREAT   0x0040
#define O_TRUNC   0x0200
#define O_APPEND  0x0400

#define SEEK_SET  0
#define SEEK_CUR  1
#define SEEK_END  2

/* ============================================================
 * VFS Structures
 * ============================================================ */

struct vfs_node;
struct vfs_dirent;

typedef int64_t (*read_fn)(struct vfs_node*, uint64_t, uint64_t, uint8_t*);
typedef int64_t (*write_fn)(struct vfs_node*, uint64_t, uint64_t, uint8_t*);
typedef int (*open_fn)(struct vfs_node*, uint32_t);
typedef int (*close_fn)(struct vfs_node*);
typedef struct vfs_dirent* (*readdir_fn)(struct vfs_node*, uint32_t);
typedef struct vfs_node* (*finddir_fn)(struct vfs_node*, const char*);

typedef struct vfs_node {
    char name[128];
    uint32_t mask;        /* Permissions */
    uint32_t uid;
    uint32_t gid;
    uint32_t flags;       /* VFS_FILE, VFS_DIRECTORY, etc. */
    uint32_t inode;
    uint64_t length;
    uint32_t impl;        /* Implementation-specific */
    
    read_fn read;
    write_fn write;
    open_fn open;
    close_fn close;
    readdir_fn readdir;
    finddir_fn finddir;
    
    struct vfs_node *ptr;  /* For symlinks/mountpoints */
} vfs_node_t;

typedef struct vfs_dirent {
    char name[128];
    uint32_t inode;
} vfs_dirent_t;

/* ============================================================
 * File Descriptor Table
 * ============================================================ */

#define MAX_FDS 256

typedef struct {
    vfs_node_t *node;
    uint64_t offset;
    uint32_t flags;
    int in_use;
} fd_entry_t;

static fd_entry_t fd_table[MAX_FDS];
static vfs_node_t *vfs_root = NULL;

/* ============================================================
 * VFS Operations
 * ============================================================ */

int64_t vfs_read(vfs_node_t *node, uint64_t offset, uint64_t size, uint8_t *buffer) {
    if (!node || !node->read) return -1;
    return node->read(node, offset, size, buffer);
}

int64_t vfs_write(vfs_node_t *node, uint64_t offset, uint64_t size, uint8_t *buffer) {
    if (!node || !node->write) return -1;
    return node->write(node, offset, size, buffer);
}

int vfs_open(vfs_node_t *node, uint32_t flags) {
    if (!node) return -1;
    if (node->open) {
        return node->open(node, flags);
    }
    return 0;
}

int vfs_close(vfs_node_t *node) {
    if (!node) return -1;
    if (node->close) {
        return node->close(node);
    }
    return 0;
}

vfs_dirent_t* vfs_readdir(vfs_node_t *node, uint32_t index) {
    if (!node || !(node->flags & VFS_DIRECTORY) || !node->readdir) {
        return NULL;
    }
    return node->readdir(node, index);
}

vfs_node_t* vfs_finddir(vfs_node_t *node, const char *name) {
    if (!node || !(node->flags & VFS_DIRECTORY) || !node->finddir) {
        return NULL;
    }
    return node->finddir(node, name);
}

/* ============================================================
 * Path Resolution
 * ============================================================ */

static int strcmp_simple(const char *s1, const char *s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

vfs_node_t* vfs_resolve_path(const char *path) {
    if (!path || !vfs_root) return NULL;
    
    /* Handle absolute path */
    if (path[0] == '/') {
        path++;
    }
    
    vfs_node_t *node = vfs_root;
    char component[128];
    int i = 0;
    
    while (*path) {
        /* Skip slashes */
        while (*path == '/') path++;
        if (!*path) break;
        
        /* Extract path component */
        i = 0;
        while (*path && *path != '/' && i < 127) {
            component[i++] = *path++;
        }
        component[i] = '\0';
        
        /* Handle . and .. */
        if (strcmp_simple(component, ".") == 0) {
            continue;
        }
        if (strcmp_simple(component, "..") == 0) {
            /* TODO: Handle parent directory */
            continue;
        }
        
        /* Find in current directory */
        node = vfs_finddir(node, component);
        if (!node) return NULL;
        
        /* Follow mountpoints */
        if (node->flags & VFS_MOUNTPOINT) {
            node = node->ptr;
        }
    }
    
    return node;
}

/* ============================================================
 * File Descriptor Operations
 * ============================================================ */

static int alloc_fd(void) {
    for (int i = 3; i < MAX_FDS; i++) {  /* 0,1,2 reserved for stdin/out/err */
        if (!fd_table[i].in_use) {
            fd_table[i].in_use = 1;
            return i;
        }
    }
    return -1;
}

int vfs_open_path(const char *path, uint32_t flags) {
    vfs_node_t *node = vfs_resolve_path(path);
    if (!node) {
        kprintf("[VFS] File not found: %s\n", path);
        return -1;
    }
    
    int fd = alloc_fd();
    if (fd < 0) {
        kprintf("[VFS] No free file descriptors\n");
        return -1;
    }
    
    if (vfs_open(node, flags) != 0) {
        fd_table[fd].in_use = 0;
        return -1;
    }
    
    fd_table[fd].node = node;
    fd_table[fd].offset = 0;
    fd_table[fd].flags = flags;
    
    return fd;
}

int vfs_close_fd(int fd) {
    if (fd < 0 || fd >= MAX_FDS || !fd_table[fd].in_use) {
        return -1;
    }
    
    vfs_close(fd_table[fd].node);
    fd_table[fd].in_use = 0;
    fd_table[fd].node = NULL;
    
    return 0;
}

int64_t vfs_read_fd(int fd, void *buffer, uint64_t size) {
    if (fd < 0 || fd >= MAX_FDS || !fd_table[fd].in_use) {
        return -1;
    }
    
    int64_t bytes = vfs_read(fd_table[fd].node, fd_table[fd].offset, size, buffer);
    if (bytes > 0) {
        fd_table[fd].offset += bytes;
    }
    return bytes;
}

int64_t vfs_write_fd(int fd, const void *buffer, uint64_t size) {
    if (fd < 0 || fd >= MAX_FDS || !fd_table[fd].in_use) {
        return -1;
    }
    
    int64_t bytes = vfs_write(fd_table[fd].node, fd_table[fd].offset, size, (uint8_t*)buffer);
    if (bytes > 0) {
        fd_table[fd].offset += bytes;
    }
    return bytes;
}

int64_t vfs_seek_fd(int fd, int64_t offset, int whence) {
    if (fd < 0 || fd >= MAX_FDS || !fd_table[fd].in_use) {
        return -1;
    }
    
    vfs_node_t *node = fd_table[fd].node;
    int64_t new_offset;
    
    switch (whence) {
        case SEEK_SET:
            new_offset = offset;
            break;
        case SEEK_CUR:
            new_offset = fd_table[fd].offset + offset;
            break;
        case SEEK_END:
            new_offset = node->length + offset;
            break;
        default:
            return -1;
    }
    
    if (new_offset < 0) return -1;
    fd_table[fd].offset = new_offset;
    return new_offset;
}

/* ============================================================
 * RAM Filesystem (Initial Root)
 * ============================================================ */

#define RAMFS_MAX_FILES 64

static vfs_node_t ramfs_nodes[RAMFS_MAX_FILES];
static vfs_dirent_t ramfs_dirent;
static int ramfs_node_count = 0;

static int64_t ramfs_read(vfs_node_t *node, uint64_t offset, uint64_t size, uint8_t *buffer) {
    if (offset >= node->length) return 0;
    if (offset + size > node->length) {
        size = node->length - offset;
    }
    /* Data stored at node->impl as pointer */
    uint8_t *data = (uint8_t*)(uintptr_t)node->impl;
    if (data) {
        for (uint64_t i = 0; i < size; i++) {
            buffer[i] = data[offset + i];
        }
    }
    return size;
}

static vfs_dirent_t* ramfs_readdir(vfs_node_t *node, uint32_t index) {
    (void)node;
    if (index >= (uint32_t)ramfs_node_count) return NULL;
    
    int i;
    for (i = 0; ramfs_nodes[index].name[i] && i < 127; i++) {
        ramfs_dirent.name[i] = ramfs_nodes[index].name[i];
    }
    ramfs_dirent.name[i] = '\0';
    ramfs_dirent.inode = ramfs_nodes[index].inode;
    
    return &ramfs_dirent;
}

static vfs_node_t* ramfs_finddir(vfs_node_t *node, const char *name) {
    (void)node;
    for (int i = 0; i < ramfs_node_count; i++) {
        if (strcmp_simple(ramfs_nodes[i].name, name) == 0) {
            return &ramfs_nodes[i];
        }
    }
    return NULL;
}

vfs_node_t* ramfs_create_file(const char *name, uint8_t *data, uint64_t size) {
    if (ramfs_node_count >= RAMFS_MAX_FILES) return NULL;
    
    vfs_node_t *node = &ramfs_nodes[ramfs_node_count];
    
    int i;
    for (i = 0; name[i] && i < 127; i++) {
        node->name[i] = name[i];
    }
    node->name[i] = '\0';
    
    node->flags = VFS_FILE;
    node->inode = ramfs_node_count;
    node->length = size;
    node->impl = (uint32_t)(uintptr_t)data;
    node->read = ramfs_read;
    node->write = NULL;
    node->open = NULL;
    node->close = NULL;
    node->readdir = NULL;
    node->finddir = NULL;
    
    ramfs_node_count++;
    return node;
}

/* ============================================================
 * VFS Initialization
 * ============================================================ */

void vfs_init(void) {
    kprintf("[VFS] Initializing virtual file system...\n");
    
    /* Clear FD table */
    for (int i = 0; i < MAX_FDS; i++) {
        fd_table[i].in_use = 0;
        fd_table[i].node = NULL;
    }
    
    /* Setup stdin/stdout/stderr (placeholder) */
    fd_table[0].in_use = 1;  /* stdin */
    fd_table[1].in_use = 1;  /* stdout */
    fd_table[2].in_use = 1;  /* stderr */
    
    /* Create root node */
    vfs_root = &ramfs_nodes[0];
    vfs_root->name[0] = '/';
    vfs_root->name[1] = '\0';
    vfs_root->flags = VFS_DIRECTORY;
    vfs_root->inode = 0;
    vfs_root->length = 0;
    vfs_root->readdir = ramfs_readdir;
    vfs_root->finddir = ramfs_finddir;
    ramfs_node_count = 1;
    
    /* Create some initial files */
    static const char *readme = "FastOS v2.0 - ADead-BIB Native OS\n";
    ramfs_create_file("README", (uint8_t*)readme, 35);
    
    static const char *version = "2.0.0\n";
    ramfs_create_file("VERSION", (uint8_t*)version, 6);
    
    kprintf("[VFS] Root filesystem mounted\n");
    kprintf("[VFS] Files: %d\n", ramfs_node_count - 1);
}

/* List directory */
void vfs_list(const char *path) {
    vfs_node_t *dir = vfs_resolve_path(path);
    if (!dir || !(dir->flags & VFS_DIRECTORY)) {
        kprintf("Not a directory: %s\n", path);
        return;
    }
    
    kprintf("\nDirectory: %s\n", path);
    kprintf("  TYPE  SIZE      NAME\n");
    kprintf("  ----  --------  ----\n");
    
    uint32_t i = 0;
    vfs_dirent_t *entry;
    while ((entry = vfs_readdir(dir, i++)) != NULL) {
        vfs_node_t *node = vfs_finddir(dir, entry->name);
        if (node) {
            const char *type = (node->flags & VFS_DIRECTORY) ? "DIR " : "FILE";
            kprintf("  %s  %8llu  %s\n", type, node->length, entry->name);
        }
    }
}
