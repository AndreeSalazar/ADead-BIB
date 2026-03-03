/*
 * ADead-BIB Standard Library
 * signal.h - Signal Handling
 * 
 * Based on: POSIX signals
 */

#ifndef _ADEAD_SIGNAL_H
#define _ADEAD_SIGNAL_H

/* Signal types */
typedef int sig_atomic_t;
typedef void (*sighandler_t)(int);

/* Standard signals */
#define SIGHUP      1   /* Hangup */
#define SIGINT      2   /* Interrupt (Ctrl+C) */
#define SIGQUIT     3   /* Quit */
#define SIGILL      4   /* Illegal instruction */
#define SIGTRAP     5   /* Trace trap */
#define SIGABRT     6   /* Abort */
#define SIGBUS      7   /* Bus error */
#define SIGFPE      8   /* Floating point exception */
#define SIGKILL     9   /* Kill (cannot be caught) */
#define SIGUSR1     10  /* User defined 1 */
#define SIGSEGV     11  /* Segmentation fault */
#define SIGUSR2     12  /* User defined 2 */
#define SIGPIPE     13  /* Broken pipe */
#define SIGALRM     14  /* Alarm clock */
#define SIGTERM     15  /* Termination */
#define SIGSTKFLT   16  /* Stack fault */
#define SIGCHLD     17  /* Child status changed */
#define SIGCONT     18  /* Continue */
#define SIGSTOP     19  /* Stop (cannot be caught) */
#define SIGTSTP     20  /* Terminal stop */
#define SIGTTIN     21  /* Background read */
#define SIGTTOU     22  /* Background write */
#define SIGURG      23  /* Urgent data */
#define SIGXCPU     24  /* CPU time limit */
#define SIGXFSZ     25  /* File size limit */
#define SIGVTALRM   26  /* Virtual timer */
#define SIGPROF     27  /* Profiling timer */
#define SIGWINCH    28  /* Window size change */
#define SIGIO       29  /* I/O possible */
#define SIGPWR      30  /* Power failure */
#define SIGSYS      31  /* Bad system call */

#define NSIG        32

/* Signal handler values */
#define SIG_DFL     ((sighandler_t)0)
#define SIG_IGN     ((sighandler_t)1)
#define SIG_ERR     ((sighandler_t)-1)

/* Signal functions */
sighandler_t signal(int signum, sighandler_t handler);
int raise(int signum);

/* POSIX signal set */
typedef unsigned long sigset_t;

int sigemptyset(sigset_t* set);
int sigfillset(sigset_t* set);
int sigaddset(sigset_t* set, int signum);
int sigdelset(sigset_t* set, int signum);
int sigismember(const sigset_t* set, int signum);

/* POSIX sigaction */
struct sigaction {
    sighandler_t sa_handler;
    void (*sa_sigaction)(int, void*, void*);
    sigset_t sa_mask;
    int sa_flags;
    void (*sa_restorer)(void);
};

#define SA_NOCLDSTOP  1
#define SA_NOCLDWAIT  2
#define SA_SIGINFO    4
#define SA_RESTART    0x10000000
#define SA_NODEFER    0x40000000
#define SA_RESETHAND  0x80000000

int sigaction(int signum, const struct sigaction* act, struct sigaction* oldact);
int sigprocmask(int how, const sigset_t* set, sigset_t* oldset);
int sigpending(sigset_t* set);
int sigsuspend(const sigset_t* mask);

#define SIG_BLOCK   0
#define SIG_UNBLOCK 1
#define SIG_SETMASK 2

#endif /* _ADEAD_SIGNAL_H */
