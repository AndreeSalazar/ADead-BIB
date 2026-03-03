/*
 * ADead-BIB Standard Library
 * sys/epoll.h - I/O Event Notification (Linux)
 * 
 * Based on: Linux epoll API
 */

#ifndef _ADEAD_SYS_EPOLL_H
#define _ADEAD_SYS_EPOLL_H

#include "types.h"

/* epoll_event data union */
typedef union epoll_data {
    void* ptr;
    int fd;
    uint32_t u32;
    uint64_t u64;
} epoll_data_t;

/* epoll_event structure */
struct epoll_event {
    uint32_t events;
    epoll_data_t data;
} __attribute__((packed));

/* Event types */
#define EPOLLIN      0x001
#define EPOLLPRI     0x002
#define EPOLLOUT     0x004
#define EPOLLRDNORM  0x040
#define EPOLLRDBAND  0x080
#define EPOLLWRNORM  0x100
#define EPOLLWRBAND  0x200
#define EPOLLMSG     0x400
#define EPOLLERR     0x008
#define EPOLLHUP     0x010
#define EPOLLRDHUP   0x2000
#define EPOLLEXCLUSIVE 0x10000000
#define EPOLLWAKEUP  0x20000000
#define EPOLLONESHOT 0x40000000
#define EPOLLET      0x80000000

/* epoll_ctl operations */
#define EPOLL_CTL_ADD 1
#define EPOLL_CTL_DEL 2
#define EPOLL_CTL_MOD 3

/* epoll_create1 flags */
#define EPOLL_CLOEXEC 0x80000

/* Functions */
int epoll_create(int size);
int epoll_create1(int flags);
int epoll_ctl(int epfd, int op, int fd, struct epoll_event* event);
int epoll_wait(int epfd, struct epoll_event* events, int maxevents, int timeout);
int epoll_pwait(int epfd, struct epoll_event* events, int maxevents,
                int timeout, const void* sigmask);

#endif /* _ADEAD_SYS_EPOLL_H */
