/*
 * ADead-BIB Standard Library
 * sys/wait.h - Process Wait
 * 
 * Based on: POSIX
 */

#ifndef _ADEAD_SYS_WAIT_H
#define _ADEAD_SYS_WAIT_H

#include "types.h"

/* Wait options */
#define WNOHANG    1
#define WUNTRACED  2
#define WCONTINUED 8

/* Status macros */
#define WEXITSTATUS(s)  (((s) & 0xFF00) >> 8)
#define WTERMSIG(s)     ((s) & 0x7F)
#define WSTOPSIG(s)     WEXITSTATUS(s)
#define WIFEXITED(s)    (WTERMSIG(s) == 0)
#define WIFSIGNALED(s)  (((signed char)(((s) & 0x7F) + 1) >> 1) > 0)
#define WIFSTOPPED(s)   (((s) & 0xFF) == 0x7F)
#define WIFCONTINUED(s) ((s) == 0xFFFF)
#define WCOREDUMP(s)    ((s) & 0x80)

/* Functions */
pid_t wait(int* wstatus);
pid_t waitpid(pid_t pid, int* wstatus, int options);
int waitid(int idtype, pid_t id, void* infop, int options);
pid_t wait3(int* wstatus, int options, void* rusage);
pid_t wait4(pid_t pid, int* wstatus, int options, void* rusage);

/* idtype values */
#define P_ALL  0
#define P_PID  1
#define P_PGID 2

#endif /* _ADEAD_SYS_WAIT_H */
