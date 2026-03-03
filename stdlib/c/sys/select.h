/*
 * ADead-BIB Standard Library
 * sys/select.h - Synchronous I/O Multiplexing
 * 
 * Based on: POSIX
 */

#ifndef _ADEAD_SYS_SELECT_H
#define _ADEAD_SYS_SELECT_H

#include "types.h"
#include "../time.h"

/* Functions */
int select(int nfds, fd_set* readfds, fd_set* writefds, fd_set* exceptfds,
           struct timeval* timeout);
int pselect(int nfds, fd_set* readfds, fd_set* writefds, fd_set* exceptfds,
            const struct timespec* timeout, const void* sigmask);

#endif /* _ADEAD_SYS_SELECT_H */
