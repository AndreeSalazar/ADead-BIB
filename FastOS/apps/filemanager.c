/*
 * FastOS v2.0 — File Manager Application
 * Native .po application for FastOS
 * 
 * Hybrid Windows Explorer + Linux Nautilus style
 * 
 * Compile: adB cc filemanager.c -o filemanager.po --po --app
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/po.h"

/* ============================================================
 * File Manager Constants
 * ============================================================ */

#define FM_MAX_FILES        256
#define FM_MAX_PATH         512
#define FM_ICON_SIZE        32
#define FM_ITEM_HEIGHT      20
#define FM_SIDEBAR_WIDTH    200

/* View modes */
#define FM_VIEW_LIST        0
#define FM_VIEW_ICONS       1
#define FM_VIEW_DETAILS     2

/* File types */
#define FT_UNKNOWN          0
#define FT_DIRECTORY        1
#define FT_FILE             2
#define FT_SYMLINK          3
#define FT_EXECUTABLE       4  /* .po files */
#define FT_IMAGE            5
#define FT_TEXT             6
#define FT_ARCHIVE          7

/* ============================================================
 * File Entry Structure
 * ============================================================ */

typedef struct {
    char name[256];
    char path[FM_MAX_PATH];
    uint32_t type;
    uint64_t size;
    uint64_t modified;
    uint32_t permissions;
    int selected;
} file_entry_t;

/* ============================================================
 * File Manager State
 * ============================================================ */

typedef struct {
    char current_path[FM_MAX_PATH];
    file_entry_t files[FM_MAX_FILES];
    int file_count;
    int selected_index;
    int scroll_offset;
    int view_mode;
    int show_hidden;
    
    /* History for back/forward */
    char history[32][FM_MAX_PATH];
    int history_pos;
    int history_count;
    
    /* Clipboard */
    char clipboard_path[FM_MAX_PATH];
    int clipboard_cut;  /* 0 = copy, 1 = cut */
    
    /* Window reference */
    void *window;
} filemanager_t;

static filemanager_t fm;

/* ============================================================
 * File Type Detection
 * ============================================================ */

static uint32_t detect_file_type(const char *name, uint32_t is_dir) {
    if (is_dir) return FT_DIRECTORY;
    
    /* Get extension */
    const char *ext = name;
    const char *p = name;
    while (*p) {
        if (*p == '.') ext = p + 1;
        p++;
    }
    
    /* Check extension */
    if (kstrcmp(ext, "po") == 0) return FT_EXECUTABLE;
    if (kstrcmp(ext, "exe") == 0) return FT_EXECUTABLE;
    if (kstrcmp(ext, "png") == 0 || kstrcmp(ext, "jpg") == 0 || 
        kstrcmp(ext, "bmp") == 0) return FT_IMAGE;
    if (kstrcmp(ext, "txt") == 0 || kstrcmp(ext, "c") == 0 || 
        kstrcmp(ext, "h") == 0 || kstrcmp(ext, "md") == 0) return FT_TEXT;
    if (kstrcmp(ext, "zip") == 0 || kstrcmp(ext, "tar") == 0 || 
        kstrcmp(ext, "gz") == 0) return FT_ARCHIVE;
    
    return FT_FILE;
}

/* ============================================================
 * Directory Operations
 * ============================================================ */

int fm_read_directory(const char *path) {
    kstrncpy(fm.current_path, path, FM_MAX_PATH - 1);
    fm.file_count = 0;
    fm.selected_index = 0;
    fm.scroll_offset = 0;
    
    /* Add parent directory entry if not root */
    if (kstrcmp(path, "/") != 0) {
        kstrcpy(fm.files[fm.file_count].name, "..");
        kstrcpy(fm.files[fm.file_count].path, path);
        fm.files[fm.file_count].type = FT_DIRECTORY;
        fm.files[fm.file_count].size = 0;
        fm.files[fm.file_count].selected = 0;
        fm.file_count++;
    }
    
    /* Read directory using VFS syscall */
    /* In real implementation, this would call sys_readdir() */
    
    /* For demo, add some sample entries */
    const char *demo_files[] = {
        "Documents", "Downloads", "Pictures", "Music", "Videos",
        "system.po", "config.txt", "readme.md", "kernel.bin"
    };
    int demo_types[] = {
        FT_DIRECTORY, FT_DIRECTORY, FT_DIRECTORY, FT_DIRECTORY, FT_DIRECTORY,
        FT_EXECUTABLE, FT_TEXT, FT_TEXT, FT_FILE
    };
    
    for (int i = 0; i < 9 && fm.file_count < FM_MAX_FILES; i++) {
        kstrcpy(fm.files[fm.file_count].name, demo_files[i]);
        fm.files[fm.file_count].type = demo_types[i];
        fm.files[fm.file_count].size = (i + 1) * 1024;
        fm.files[fm.file_count].selected = 0;
        fm.file_count++;
    }
    
    return 0;
}

int fm_navigate(const char *path) {
    /* Save to history */
    if (fm.history_pos < 31) {
        kstrcpy(fm.history[fm.history_pos], fm.current_path);
        fm.history_pos++;
        fm.history_count = fm.history_pos;
    }
    
    return fm_read_directory(path);
}

int fm_go_back(void) {
    if (fm.history_pos > 0) {
        fm.history_pos--;
        return fm_read_directory(fm.history[fm.history_pos]);
    }
    return -1;
}

int fm_go_forward(void) {
    if (fm.history_pos < fm.history_count - 1) {
        fm.history_pos++;
        return fm_read_directory(fm.history[fm.history_pos]);
    }
    return -1;
}

int fm_go_up(void) {
    /* Find parent directory */
    char parent[FM_MAX_PATH];
    kstrcpy(parent, fm.current_path);
    
    int len = kstrlen(parent);
    if (len > 1) {
        /* Remove trailing slash if present */
        if (parent[len-1] == '/') {
            parent[len-1] = '\0';
            len--;
        }
        /* Find last slash */
        while (len > 0 && parent[len-1] != '/') {
            len--;
        }
        if (len > 0) {
            parent[len] = '\0';
            if (len == 0) kstrcpy(parent, "/");
            return fm_navigate(parent);
        }
    }
    return -1;
}

/* ============================================================
 * File Operations
 * ============================================================ */

int fm_open_selected(void) {
    if (fm.selected_index < 0 || fm.selected_index >= fm.file_count) {
        return -1;
    }
    
    file_entry_t *entry = &fm.files[fm.selected_index];
    
    if (entry->type == FT_DIRECTORY) {
        /* Navigate into directory */
        char new_path[FM_MAX_PATH];
        if (kstrcmp(entry->name, "..") == 0) {
            return fm_go_up();
        } else {
            if (kstrcmp(fm.current_path, "/") == 0) {
                kstrcpy(new_path, "/");
                kstrcat(new_path, entry->name);
            } else {
                kstrcpy(new_path, fm.current_path);
                kstrcat(new_path, "/");
                kstrcat(new_path, entry->name);
            }
            return fm_navigate(new_path);
        }
    } else if (entry->type == FT_EXECUTABLE) {
        /* Execute .po file */
        /* syscall(SYS_EXEC, entry->path, 0, 0, 0, 0); */
        kprintf("[FM] Executing: %s\n", entry->name);
        return 0;
    } else {
        /* Open with default application */
        kprintf("[FM] Opening: %s\n", entry->name);
        return 0;
    }
}

int fm_delete_selected(void) {
    if (fm.selected_index < 0 || fm.selected_index >= fm.file_count) {
        return -1;
    }
    
    file_entry_t *entry = &fm.files[fm.selected_index];
    
    /* Would call sys_unlink() or sys_rmdir() */
    kprintf("[FM] Deleting: %s\n", entry->name);
    
    /* Refresh directory */
    return fm_read_directory(fm.current_path);
}

int fm_copy_selected(void) {
    if (fm.selected_index < 0 || fm.selected_index >= fm.file_count) {
        return -1;
    }
    
    file_entry_t *entry = &fm.files[fm.selected_index];
    kstrcpy(fm.clipboard_path, fm.current_path);
    kstrcat(fm.clipboard_path, "/");
    kstrcat(fm.clipboard_path, entry->name);
    fm.clipboard_cut = 0;
    
    kprintf("[FM] Copied: %s\n", entry->name);
    return 0;
}

int fm_cut_selected(void) {
    int result = fm_copy_selected();
    if (result == 0) {
        fm.clipboard_cut = 1;
        kprintf("[FM] Cut: %s\n", fm.files[fm.selected_index].name);
    }
    return result;
}

int fm_paste(void) {
    if (fm.clipboard_path[0] == '\0') {
        return -1;
    }
    
    /* Would call sys_copy() or sys_rename() */
    kprintf("[FM] Pasting to: %s\n", fm.current_path);
    
    if (fm.clipboard_cut) {
        fm.clipboard_path[0] = '\0';
    }
    
    return fm_read_directory(fm.current_path);
}

int fm_create_folder(const char *name) {
    char path[FM_MAX_PATH];
    kstrcpy(path, fm.current_path);
    kstrcat(path, "/");
    kstrcat(path, name);
    
    /* Would call sys_mkdir() */
    kprintf("[FM] Creating folder: %s\n", path);
    
    return fm_read_directory(fm.current_path);
}

/* ============================================================
 * File Manager Drawing
 * ============================================================ */

void fm_draw_sidebar(uint32_t *fb, int width, int height) {
    /* Draw sidebar background */
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < FM_SIDEBAR_WIDTH; x++) {
            fb[y * width + x] = 0xFFF0F0F0;  /* Light gray */
        }
    }
    
    /* Draw quick access items */
    const char *quick_access[] = {
        "Home", "Desktop", "Documents", "Downloads", 
        "Pictures", "Music", "Videos", "Trash"
    };
    
    int y_pos = 10;
    for (int i = 0; i < 8; i++) {
        /* Would draw icon and text here */
        /* fb_string(fb, 10, y_pos, quick_access[i], 0xFF000000, 0); */
        y_pos += 24;
    }
}

void fm_draw_toolbar(uint32_t *fb, int width, int y_start) {
    /* Draw toolbar background */
    for (int y = y_start; y < y_start + 32; y++) {
        for (int x = 0; x < width; x++) {
            fb[y * width + x] = 0xFFE0E0E0;
        }
    }
    
    /* Would draw back, forward, up, refresh buttons */
    /* Would draw address bar */
    /* Would draw search box */
}

void fm_draw_file_list(uint32_t *fb, int width, int height) {
    int x_start = FM_SIDEBAR_WIDTH + 10;
    int y_start = 50;
    int visible_items = (height - y_start - 30) / FM_ITEM_HEIGHT;
    
    for (int i = 0; i < visible_items && (i + fm.scroll_offset) < fm.file_count; i++) {
        int idx = i + fm.scroll_offset;
        file_entry_t *entry = &fm.files[idx];
        
        int y = y_start + i * FM_ITEM_HEIGHT;
        
        /* Highlight selected item */
        if (idx == fm.selected_index) {
            for (int py = y; py < y + FM_ITEM_HEIGHT; py++) {
                for (int px = x_start; px < width - 10; px++) {
                    fb[py * width + px] = 0xFF0078D7;  /* Blue selection */
                }
            }
        }
        
        /* Draw icon based on type */
        uint32_t icon_color;
        switch (entry->type) {
            case FT_DIRECTORY:  icon_color = 0xFFFFD700; break;  /* Gold folder */
            case FT_EXECUTABLE: icon_color = 0xFF00AA00; break;  /* Green exe */
            case FT_IMAGE:      icon_color = 0xFF0088FF; break;  /* Blue image */
            case FT_TEXT:       icon_color = 0xFFAAAAAA; break;  /* Gray text */
            default:            icon_color = 0xFFCCCCCC; break;
        }
        
        /* Draw small icon */
        for (int py = y + 2; py < y + 16; py++) {
            for (int px = x_start; px < x_start + 16; px++) {
                fb[py * width + px] = icon_color;
            }
        }
        
        /* Would draw filename text here */
        /* fb_string(fb, x_start + 20, y + 2, entry->name, text_color, 0); */
    }
}

void fm_draw(uint32_t *fb, int width, int height) {
    /* Clear background */
    for (int i = 0; i < width * height; i++) {
        fb[i] = 0xFFFFFFFF;
    }
    
    fm_draw_sidebar(fb, width, height);
    fm_draw_toolbar(fb, width, 0);
    fm_draw_file_list(fb, width, height);
}

/* ============================================================
 * File Manager Input Handling
 * ============================================================ */

void fm_handle_key(char key) {
    switch (key) {
        case '\n':  /* Enter - open */
            fm_open_selected();
            break;
        case '\b':  /* Backspace - go up */
            fm_go_up();
            break;
        case 'c':   /* Ctrl+C would be copy */
            fm_copy_selected();
            break;
        case 'x':   /* Ctrl+X would be cut */
            fm_cut_selected();
            break;
        case 'v':   /* Ctrl+V would be paste */
            fm_paste();
            break;
        default:
            /* Arrow keys for navigation */
            if (key == 'j' || key == 's') {  /* Down */
                if (fm.selected_index < fm.file_count - 1) {
                    fm.selected_index++;
                }
            } else if (key == 'k' || key == 'w') {  /* Up */
                if (fm.selected_index > 0) {
                    fm.selected_index--;
                }
            }
            break;
    }
}

void fm_handle_click(int x, int y, int button) {
    /* Check sidebar clicks */
    if (x < FM_SIDEBAR_WIDTH) {
        /* Handle quick access clicks */
        return;
    }
    
    /* Check file list clicks */
    int y_start = 50;
    int idx = (y - y_start) / FM_ITEM_HEIGHT + fm.scroll_offset;
    
    if (idx >= 0 && idx < fm.file_count) {
        if (button == 0) {  /* Left click */
            fm.selected_index = idx;
        } else if (button == 1) {  /* Double click or right click */
            fm.selected_index = idx;
            fm_open_selected();
        }
    }
}

/* ============================================================
 * File Manager Initialization
 * ============================================================ */

void fm_init(void) {
    kmemset(&fm, 0, sizeof(fm));
    kstrcpy(fm.current_path, "/");
    fm.view_mode = FM_VIEW_LIST;
    fm.show_hidden = 0;
    
    fm_read_directory("/");
    
    kprintf("[FM] File Manager initialized\n");
}

/* ============================================================
 * File Manager Entry Point (for .po app)
 * ============================================================ */

int main(void) {
    fm_init();
    
    /* Create window */
    /* fm.window = window_create("File Manager", 100, 100, 800, 600, WS_DEFAULT); */
    
    /* Main loop would be handled by window system */
    
    return 0;
}
