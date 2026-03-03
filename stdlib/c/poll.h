/*
 * ADead-BIB Standard Library
 * poll.h - I/O Multiplexing
 * 
 * Based on: POSIX
 */

#ifndef _ADEAD_POLL_H
#define _ADEAD_POLL_H

/* Poll structure */
struct pollfd {
    int fd;
    short events;
    short revents;
};

typedef unsigned long nfds_t;

/* Event flags */
#define POLLIN     0x0001  /* Data to read */
#define POLLPRI    0x0002  /* Urgent data */
#define POLLOUT    0x0004  /* Writing possible */
#define POLLERR    0x0008  /* Error */
#define POLLHUP    0x0010  /* Hang up */
#define POLLNVAL   0x0020  /* Invalid fd */
#define POLLRDNORM 0x0040  /* Normal data */
#define POLLRDBAND 0x0080  /* Priority data */
#define POLLWRNORM 0x0100  /* Normal write */
#define POLLWRBAND 0x0200  /* Priority write */

/* Functions */
int poll(struct pollfd* fds, nfds_t nfds, int timeout);
int ppoll(struct pollfd* fds, nfds_t nfds, const struct timespec* tmo_p,
          const void* sigmask);

#endif /* _ADEAD_POLL_H */
