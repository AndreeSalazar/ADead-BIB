/*
 * ADead-BIB Standard Library
 * arpa/inet.h - Internet Address Manipulation
 * 
 * Based on: POSIX, BSD
 */

#ifndef _ADEAD_ARPA_INET_H
#define _ADEAD_ARPA_INET_H

#include "../netinet/in.h"

/* Address conversion */
in_addr_t inet_addr(const char* cp);
int inet_aton(const char* cp, struct in_addr* inp);
char* inet_ntoa(struct in_addr in);

/* Modern versions */
int inet_pton(int af, const char* src, void* dst);
const char* inet_ntop(int af, const void* src, char* dst, socklen_t size);

/* Network/host byte order */
in_addr_t inet_network(const char* cp);
in_addr_t inet_lnaof(struct in_addr in);
in_addr_t inet_netof(struct in_addr in);
struct in_addr inet_makeaddr(in_addr_t net, in_addr_t lna);

/* Buffer sizes */
#define INET_ADDRSTRLEN  16
#define INET6_ADDRSTRLEN 46

#endif /* _ADEAD_ARPA_INET_H */
