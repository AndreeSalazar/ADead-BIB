/*
 * ADead-BIB Standard Library
 * netinet/in.h - Internet Address Family
 * 
 * Based on: POSIX, BSD
 */

#ifndef _ADEAD_NETINET_IN_H
#define _ADEAD_NETINET_IN_H

#include "../sys/socket.h"
#include "../stdint.h"

/* IP protocols */
#define IPPROTO_IP      0
#define IPPROTO_ICMP    1
#define IPPROTO_TCP     6
#define IPPROTO_UDP     17
#define IPPROTO_IPV6    41
#define IPPROTO_ICMPV6  58
#define IPPROTO_RAW     255

/* IPv4 address */
struct in_addr {
    in_addr_t s_addr;
};

/* IPv4 socket address */
struct sockaddr_in {
    sa_family_t sin_family;
    in_port_t sin_port;
    struct in_addr sin_addr;
    unsigned char sin_zero[8];
};

/* IPv6 address */
struct in6_addr {
    union {
        uint8_t __s6_addr[16];
        uint16_t __s6_addr16[8];
        uint32_t __s6_addr32[4];
    };
};
#define s6_addr __s6_addr

/* IPv6 socket address */
struct sockaddr_in6 {
    sa_family_t sin6_family;
    in_port_t sin6_port;
    uint32_t sin6_flowinfo;
    struct in6_addr sin6_addr;
    uint32_t sin6_scope_id;
};

/* Special addresses */
#define INADDR_ANY       ((in_addr_t)0x00000000)
#define INADDR_BROADCAST ((in_addr_t)0xFFFFFFFF)
#define INADDR_LOOPBACK  ((in_addr_t)0x7F000001)
#define INADDR_NONE      ((in_addr_t)0xFFFFFFFF)

extern const struct in6_addr in6addr_any;
extern const struct in6_addr in6addr_loopback;
#define IN6ADDR_ANY_INIT      { { { 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 } } }
#define IN6ADDR_LOOPBACK_INIT { { { 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1 } } }

/* Byte order conversion */
uint16_t htons(uint16_t hostshort);
uint16_t ntohs(uint16_t netshort);
uint32_t htonl(uint32_t hostlong);
uint32_t ntohl(uint32_t netlong);

/* Inline implementations for x86 (little-endian) */
#define htons(x) __builtin_bswap16(x)
#define ntohs(x) __builtin_bswap16(x)
#define htonl(x) __builtin_bswap32(x)
#define ntohl(x) __builtin_bswap32(x)

/* IP options */
#define IP_TOS          1
#define IP_TTL          2
#define IP_HDRINCL      3
#define IP_OPTIONS      4
#define IP_MULTICAST_IF 32
#define IP_MULTICAST_TTL 33
#define IP_MULTICAST_LOOP 34
#define IP_ADD_MEMBERSHIP 35
#define IP_DROP_MEMBERSHIP 36

/* IPv6 options */
#define IPV6_UNICAST_HOPS   16
#define IPV6_MULTICAST_IF   17
#define IPV6_MULTICAST_HOPS 18
#define IPV6_MULTICAST_LOOP 19
#define IPV6_V6ONLY         26

/* Multicast */
struct ip_mreq {
    struct in_addr imr_multiaddr;
    struct in_addr imr_interface;
};

struct ipv6_mreq {
    struct in6_addr ipv6mr_multiaddr;
    unsigned int ipv6mr_interface;
};

/* Address testing macros */
#define IN6_IS_ADDR_UNSPECIFIED(a) \
    (((uint32_t*)(a))[0] == 0 && ((uint32_t*)(a))[1] == 0 && \
     ((uint32_t*)(a))[2] == 0 && ((uint32_t*)(a))[3] == 0)

#define IN6_IS_ADDR_LOOPBACK(a) \
    (((uint32_t*)(a))[0] == 0 && ((uint32_t*)(a))[1] == 0 && \
     ((uint32_t*)(a))[2] == 0 && ((uint32_t*)(a))[3] == htonl(1))

#define IN6_IS_ADDR_MULTICAST(a) (((uint8_t*)(a))[0] == 0xFF)

#define IN6_IS_ADDR_V4MAPPED(a) \
    (((uint32_t*)(a))[0] == 0 && ((uint32_t*)(a))[1] == 0 && \
     ((uint32_t*)(a))[2] == htonl(0xFFFF))

#endif /* _ADEAD_NETINET_IN_H */
