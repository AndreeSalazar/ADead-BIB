// ============================================================
// fastos_linux.rs — Linux Platform API (C ABI) for ADead-BIB
// ============================================================
// POSIX / Linux syscall wrappers and X11/Wayland basics
// ============================================================

// ── POSIX / libc Functions ──
pub const POSIX_FUNCTIONS: &[&str] = &[
    // Process
    "fork", "exec", "execv", "execve", "execvp",
    "wait", "waitpid", "exit", "_exit",
    "getpid", "getppid", "getuid", "geteuid",
    "kill", "raise",
    // File I/O (syscall level)
    "open", "close", "read", "write",
    "lseek", "fstat", "stat", "lstat",
    "access", "unlink", "rename", "mkdir", "rmdir",
    "opendir", "readdir", "closedir",
    "dup", "dup2", "pipe",
    "fcntl", "ioctl",
    "mmap", "munmap", "mprotect", "msync",
    "readlink", "symlink", "link",
    "chmod", "chown", "chdir", "getcwd",
    "fsync", "fdatasync",
    "truncate", "ftruncate",
    // Memory
    "brk", "sbrk",
    // Threading (pthreads)
    "pthread_create", "pthread_join", "pthread_exit",
    "pthread_detach", "pthread_self", "pthread_equal",
    "pthread_mutex_init", "pthread_mutex_destroy",
    "pthread_mutex_lock", "pthread_mutex_unlock",
    "pthread_mutex_trylock",
    "pthread_rwlock_init", "pthread_rwlock_destroy",
    "pthread_rwlock_rdlock", "pthread_rwlock_wrlock",
    "pthread_rwlock_unlock",
    "pthread_cond_init", "pthread_cond_destroy",
    "pthread_cond_wait", "pthread_cond_signal", "pthread_cond_broadcast",
    "pthread_key_create", "pthread_key_delete",
    "pthread_getspecific", "pthread_setspecific",
    "pthread_attr_init", "pthread_attr_destroy",
    "pthread_attr_setdetachstate",
    // Signals
    "signal", "sigaction",
    "sigprocmask", "sigemptyset", "sigfillset", "sigaddset",
    // Time
    "time", "clock_gettime", "clock_getres",
    "nanosleep", "usleep", "sleep",
    "gettimeofday",
    // Dynamic linking
    "dlopen", "dlclose", "dlsym", "dlerror",
    // Network (basic)
    "socket", "bind", "listen", "accept", "connect",
    "send", "recv", "sendto", "recvfrom",
    "setsockopt", "getsockopt",
    "select", "poll", "epoll_create", "epoll_ctl", "epoll_wait",
    "getaddrinfo", "freeaddrinfo",
    "inet_ntop", "inet_pton",
    // Misc
    "sysconf", "getenv", "setenv", "unsetenv",
    "system",
    "strerror", "perror",
];

// ── Linux-specific functions ──
pub const LINUX_FUNCTIONS: &[&str] = &[
    "epoll_create1",
    "eventfd",
    "timerfd_create", "timerfd_settime",
    "signalfd",
    "inotify_init", "inotify_add_watch", "inotify_rm_watch",
    "prctl",
    "getrandom",
    "memfd_create",
    "copy_file_range",
    "io_uring_setup", "io_uring_enter", "io_uring_register",
    "clone", "clone3",
    "futex",
];

// ── X11 Functions (libX11.so) ──
pub const X11_FUNCTIONS: &[&str] = &[
    "XOpenDisplay", "XCloseDisplay",
    "XCreateWindow", "XCreateSimpleWindow",
    "XDestroyWindow",
    "XMapWindow", "XUnmapWindow",
    "XNextEvent", "XPending", "XCheckWindowEvent",
    "XSelectInput",
    "XFlush", "XSync",
    "XStoreName",
    "XCreateGC", "XFreeGC",
    "XDrawLine", "XDrawRectangle", "XFillRectangle",
    "XDrawString",
    "XSetForeground", "XSetBackground",
    "XDefaultRootWindow", "XDefaultScreen",
    "XBlackPixel", "XWhitePixel",
    "XInternAtom",
    "XSetWMProtocols",
    "XGetWindowAttributes",
    "XMoveResizeWindow",
    "XChangeProperty",
    "XGetDefault",
    "XLookupKeysym",
    "XLookupString",
    "XGrabKeyboard", "XUngrabKeyboard",
    // GLX
    "glXChooseVisual", "glXCreateContext", "glXDestroyContext",
    "glXMakeCurrent", "glXSwapBuffers",
    "glXGetProcAddress", "glXGetProcAddressARB",
    "glXChooseFBConfig", "glXGetVisualFromFBConfig",
    "glXCreateNewContext",
    "glXQueryExtensionsString",
];

// ── Wayland Functions (libwayland-client.so) ──
pub const WAYLAND_FUNCTIONS: &[&str] = &[
    "wl_display_connect", "wl_display_disconnect",
    "wl_display_dispatch", "wl_display_roundtrip",
    "wl_display_flush",
    "wl_display_get_fd",
    "wl_registry_bind",
    "wl_compositor_create_surface",
    "wl_surface_attach", "wl_surface_commit", "wl_surface_damage",
    "wl_shell_get_shell_surface",
    "wl_shell_surface_set_toplevel",
    "xdg_wm_base_get_xdg_surface",
    "xdg_surface_get_toplevel",
    "xdg_toplevel_set_title",
];

// ── Linux Types ──
pub const LINUX_TYPES: &[&str] = &[
    "pid_t", "uid_t", "gid_t", "mode_t", "off_t", "ssize_t", "size_t",
    "time_t", "clock_t", "clockid_t",
    "pthread_t", "pthread_mutex_t", "pthread_cond_t",
    "pthread_rwlock_t", "pthread_key_t", "pthread_attr_t",
    "sigset_t", "sigaction",
    "fd_set", "nfds_t",
    "sockaddr", "sockaddr_in", "sockaddr_in6", "sockaddr_un",
    "addrinfo",
    "epoll_event", "pollfd",
    "timespec", "timeval", "itimerspec",
    "stat", "dirent",
    "Display", "Window", "GC", "Visual", "Colormap",
    "XEvent", "XKeyEvent", "XButtonEvent", "XMotionEvent",
    "XWindowAttributes",
    "Atom", "KeySym",
    "GLXContext", "GLXFBConfig",
    "wl_display", "wl_registry", "wl_compositor", "wl_surface",
    "wl_shell", "wl_shell_surface", "wl_seat", "wl_output",
    "xdg_wm_base", "xdg_surface", "xdg_toplevel",
];

pub fn is_linux_symbol(name: &str) -> bool {
    POSIX_FUNCTIONS.contains(&name)
        || LINUX_FUNCTIONS.contains(&name)
        || X11_FUNCTIONS.contains(&name)
        || WAYLAND_FUNCTIONS.contains(&name)
        || LINUX_TYPES.contains(&name)
}
