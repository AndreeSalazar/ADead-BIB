/*
 * ADead-BIB Standard Library
 * sys/socket.h - Socket Interface
 * 
 * Based on: POSIX, BSD sockets
 */

#ifndef _ADEAD_SYS_SOCKET_H
#define _ADEAD_SYS_SOCKET_H

#include "types.h"

/* Socket types */
#define SOCK_STREAM    1   /* TCP */
#define SOCK_DGRAM     2   /* UDP */
#define SOCK_RAW       3   /* Raw */
#define SOCK_SEQPACKET 5   /* Sequenced packet */

/* Socket flags */
#define SOCK_NONBLOCK  0x800
#define SOCK_CLOEXEC   0x80000

/* Address families */
#define AF_UNSPEC   0
#define AF_UNIX     1
#define AF_LOCAL    AF_UNIX
#define AF_INET     2
#define AF_INET6    10
#define AF_PACKET   17
#define AF_NETLINK  16

/* Protocol families (same as AF_*) */
#define PF_UNSPEC   AF_UNSPEC
#define PF_UNIX     AF_UNIX
#define PF_LOCAL    AF_LOCAL
#define PF_INET     AF_INET
#define PF_INET6    AF_INET6

/* Socket options levels */
#define SOL_SOCKET  1
#define SOL_IP      0
#define SOL_TCP     6
#define SOL_UDP     17
#define SOL_IPV6    41

/* Socket options */
#define SO_DEBUG        1
#define SO_REUSEADDR    2
#define SO_TYPE         3
#define SO_ERROR        4
#define SO_DONTROUTE    5
#define SO_BROADCAST    6
#define SO_SNDBUF       7
#define SO_RCVBUF       8
#define SO_KEEPALIVE    9
#define SO_OOBINLINE    10
#define SO_LINGER       13
#define SO_RCVLOWAT     18
#define SO_SNDLOWAT     19
#define SO_RCVTIMEO     20
#define SO_SNDTIMEO     21
#define SO_ACCEPTCONN   30
#define SO_REUSEPORT    15

/* Shutdown how */
#define SHUT_RD   0
#define SHUT_WR   1
#define SHUT_RDWR 2

/* Message flags */
#define MSG_OOB       0x01
#define MSG_PEEK      0x02
#define MSG_DONTROUTE 0x04
#define MSG_DONTWAIT  0x40
#define MSG_WAITALL   0x100
#define MSG_NOSIGNAL  0x4000

/* Generic socket address */
struct sockaddr {
    sa_family_t sa_family;
    char sa_data[14];
};

/* Socket address storage */
struct sockaddr_storage {
    sa_family_t ss_family;
    char __ss_padding[128 - sizeof(sa_family_t)];
};

/* Unix domain socket address */
struct sockaddr_un {
    sa_family_t sun_family;
    char sun_path[108];
};

/* Linger structure */
struct linger {
    int l_onoff;
    int l_linger;
};

/* Message header for sendmsg/recvmsg */
struct msghdr {
    void* msg_name;
    socklen_t msg_namelen;
    struct iovec* msg_iov;
    size_t msg_iovlen;
    void* msg_control;
    size_t msg_controllen;
    int msg_flags;
};

struct iovec {
    void* iov_base;
    size_t iov_len;
};

/* Control message header */
struct cmsghdr {
    size_t cmsg_len;
    int cmsg_level;
    int cmsg_type;
};

/* Socket functions */
int socket(int domain, int type, int protocol);
int socketpair(int domain, int type, int protocol, int sv[2]);
int bind(int sockfd, const struct sockaddr* addr, socklen_t addrlen);
int listen(int sockfd, int backlog);
int accept(int sockfd, struct sockaddr* addr, socklen_t* addrlen);
int accept4(int sockfd, struct sockaddr* addr, socklen_t* addrlen, int flags);
int connect(int sockfd, const struct sockaddr* addr, socklen_t addrlen);

ssize_t send(int sockfd, const void* buf, size_t len, int flags);
ssize_t sendto(int sockfd, const void* buf, size_t len, int flags,
               const struct sockaddr* dest_addr, socklen_t addrlen);
ssize_t sendmsg(int sockfd, const struct msghdr* msg, int flags);

ssize_t recv(int sockfd, void* buf, size_t len, int flags);
ssize_t recvfrom(int sockfd, void* buf, size_t len, int flags,
                 struct sockaddr* src_addr, socklen_t* addrlen);
ssize_t recvmsg(int sockfd, struct msghdr* msg, int flags);

int shutdown(int sockfd, int how);
int getsockopt(int sockfd, int level, int optname, void* optval, socklen_t* optlen);
int setsockopt(int sockfd, int level, int optname, const void* optval, socklen_t optlen);
int getsockname(int sockfd, struct sockaddr* addr, socklen_t* addrlen);
int getpeername(int sockfd, struct sockaddr* addr, socklen_t* addrlen);

#endif /* _ADEAD_SYS_SOCKET_H */
