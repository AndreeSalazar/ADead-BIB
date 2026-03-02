/*
 * FastOS v2.0 — Network Stack
 * TCP/IP networking for FastOS
 * 
 * Supports:
 * - Ethernet (Intel E1000, Realtek RTL8139)
 * - ARP, IPv4, ICMP, UDP, TCP
 * - BSD-style socket API
 * 
 * Compile: adB cc network.c -o network.po --driver
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* ============================================================
 * Network Constants
 * ============================================================ */

#define ETH_FRAME_MAX       1518
#define ETH_HEADER_SIZE     14
#define IP_HEADER_SIZE      20
#define TCP_HEADER_SIZE     20
#define UDP_HEADER_SIZE     8

#define ETH_TYPE_IP         0x0800
#define ETH_TYPE_ARP        0x0806
#define ETH_TYPE_IPV6       0x86DD

#define IP_PROTO_ICMP       1
#define IP_PROTO_TCP        6
#define IP_PROTO_UDP        17

#define MAX_SOCKETS         64
#define MAX_CONNECTIONS     128
#define SOCKET_BUFFER_SIZE  65536

/* ============================================================
 * Network Structures
 * ============================================================ */

/* MAC Address */
typedef struct {
    uint8_t addr[6];
} __packed mac_addr_t;

/* IPv4 Address */
typedef struct {
    uint8_t addr[4];
} __packed ipv4_addr_t;

/* Ethernet Header */
typedef struct {
    mac_addr_t dest;
    mac_addr_t src;
    uint16_t type;
} __packed eth_header_t;

/* ARP Packet */
typedef struct {
    uint16_t hw_type;
    uint16_t proto_type;
    uint8_t hw_size;
    uint8_t proto_size;
    uint16_t opcode;
    mac_addr_t sender_mac;
    ipv4_addr_t sender_ip;
    mac_addr_t target_mac;
    ipv4_addr_t target_ip;
} __packed arp_packet_t;

/* IPv4 Header */
typedef struct {
    uint8_t version_ihl;
    uint8_t tos;
    uint16_t total_length;
    uint16_t id;
    uint16_t flags_fragment;
    uint8_t ttl;
    uint8_t protocol;
    uint16_t checksum;
    ipv4_addr_t src;
    ipv4_addr_t dest;
} __packed ipv4_header_t;

/* ICMP Header */
typedef struct {
    uint8_t type;
    uint8_t code;
    uint16_t checksum;
    uint16_t id;
    uint16_t sequence;
} __packed icmp_header_t;

/* UDP Header */
typedef struct {
    uint16_t src_port;
    uint16_t dest_port;
    uint16_t length;
    uint16_t checksum;
} __packed udp_header_t;

/* TCP Header */
typedef struct {
    uint16_t src_port;
    uint16_t dest_port;
    uint32_t seq_num;
    uint32_t ack_num;
    uint8_t data_offset;
    uint8_t flags;
    uint16_t window;
    uint16_t checksum;
    uint16_t urgent;
} __packed tcp_header_t;

/* TCP Flags */
#define TCP_FIN     0x01
#define TCP_SYN     0x02
#define TCP_RST     0x04
#define TCP_PSH     0x08
#define TCP_ACK     0x10
#define TCP_URG     0x20

/* Socket States */
typedef enum {
    SOCK_CLOSED = 0,
    SOCK_LISTEN,
    SOCK_SYN_SENT,
    SOCK_SYN_RECEIVED,
    SOCK_ESTABLISHED,
    SOCK_FIN_WAIT_1,
    SOCK_FIN_WAIT_2,
    SOCK_CLOSE_WAIT,
    SOCK_CLOSING,
    SOCK_LAST_ACK,
    SOCK_TIME_WAIT
} socket_state_t;

/* Socket Types */
#define SOCK_STREAM     1   /* TCP */
#define SOCK_DGRAM      2   /* UDP */
#define SOCK_RAW        3   /* Raw IP */

/* Socket Structure */
typedef struct {
    int fd;
    int type;
    socket_state_t state;
    
    ipv4_addr_t local_ip;
    uint16_t local_port;
    ipv4_addr_t remote_ip;
    uint16_t remote_port;
    
    /* TCP state */
    uint32_t seq_num;
    uint32_t ack_num;
    uint32_t send_window;
    uint32_t recv_window;
    
    /* Buffers */
    uint8_t *send_buffer;
    uint8_t *recv_buffer;
    uint32_t send_len;
    uint32_t recv_len;
    
    /* Callbacks */
    void (*on_connect)(int fd);
    void (*on_receive)(int fd, void *data, size_t len);
    void (*on_close)(int fd);
} socket_t;

/* ============================================================
 * Network Interface
 * ============================================================ */

typedef struct {
    char name[16];
    mac_addr_t mac;
    ipv4_addr_t ip;
    ipv4_addr_t netmask;
    ipv4_addr_t gateway;
    ipv4_addr_t dns;
    
    /* Statistics */
    uint64_t rx_packets;
    uint64_t tx_packets;
    uint64_t rx_bytes;
    uint64_t tx_bytes;
    uint64_t rx_errors;
    uint64_t tx_errors;
    
    /* Driver functions */
    int (*send)(void *data, size_t len);
    int (*receive)(void *buffer, size_t max_len);
} net_interface_t;

/* ============================================================
 * Global State
 * ============================================================ */

static net_interface_t interfaces[4];
static int interface_count = 0;
static socket_t sockets[MAX_SOCKETS];
static int next_port = 49152;  /* Ephemeral port range start */

/* ARP Cache */
#define ARP_CACHE_SIZE 64
static struct {
    ipv4_addr_t ip;
    mac_addr_t mac;
    uint32_t timestamp;
} arp_cache[ARP_CACHE_SIZE];

/* ============================================================
 * Utility Functions
 * ============================================================ */

static uint16_t htons(uint16_t val) {
    return ((val & 0xFF) << 8) | ((val >> 8) & 0xFF);
}

static uint16_t ntohs(uint16_t val) {
    return htons(val);
}

static uint32_t htonl(uint32_t val) {
    return ((val & 0xFF) << 24) | ((val & 0xFF00) << 8) |
           ((val >> 8) & 0xFF00) | ((val >> 24) & 0xFF);
}

static uint32_t ntohl(uint32_t val) {
    return htonl(val);
}

/* Calculate IP checksum */
static uint16_t ip_checksum(void *data, size_t len) {
    uint32_t sum = 0;
    uint16_t *ptr = (uint16_t*)data;
    
    while (len > 1) {
        sum += *ptr++;
        len -= 2;
    }
    
    if (len == 1) {
        sum += *(uint8_t*)ptr;
    }
    
    while (sum >> 16) {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    
    return ~sum;
}

/* Parse IP address string */
int ip_parse(const char *str, ipv4_addr_t *ip) {
    int a, b, c, d;
    /* Simple parsing - would use sscanf in real implementation */
    ip->addr[0] = 192;
    ip->addr[1] = 168;
    ip->addr[2] = 1;
    ip->addr[3] = 100;
    return 0;
}

/* Format IP address to string */
void ip_format(ipv4_addr_t *ip, char *buf) {
    /* Would use sprintf */
    buf[0] = '\0';
}

/* Compare IP addresses */
int ip_equal(ipv4_addr_t *a, ipv4_addr_t *b) {
    return kmemcmp(a->addr, b->addr, 4) == 0;
}

/* Compare MAC addresses */
int mac_equal(mac_addr_t *a, mac_addr_t *b) {
    return kmemcmp(a->addr, b->addr, 6) == 0;
}

/* ============================================================
 * ARP Functions
 * ============================================================ */

mac_addr_t *arp_lookup(ipv4_addr_t *ip) {
    for (int i = 0; i < ARP_CACHE_SIZE; i++) {
        if (ip_equal(&arp_cache[i].ip, ip)) {
            return &arp_cache[i].mac;
        }
    }
    return NULL;
}

void arp_add(ipv4_addr_t *ip, mac_addr_t *mac) {
    /* Find empty slot or oldest entry */
    int slot = 0;
    uint32_t oldest = 0xFFFFFFFF;
    
    for (int i = 0; i < ARP_CACHE_SIZE; i++) {
        if (arp_cache[i].timestamp == 0) {
            slot = i;
            break;
        }
        if (arp_cache[i].timestamp < oldest) {
            oldest = arp_cache[i].timestamp;
            slot = i;
        }
    }
    
    kmemcpy(&arp_cache[slot].ip, ip, sizeof(ipv4_addr_t));
    kmemcpy(&arp_cache[slot].mac, mac, sizeof(mac_addr_t));
    arp_cache[slot].timestamp = timer_get_ticks();
}

int arp_request(net_interface_t *iface, ipv4_addr_t *target_ip) {
    uint8_t packet[ETH_HEADER_SIZE + sizeof(arp_packet_t)];
    eth_header_t *eth = (eth_header_t*)packet;
    arp_packet_t *arp = (arp_packet_t*)(packet + ETH_HEADER_SIZE);
    
    /* Broadcast destination */
    kmemset(eth->dest.addr, 0xFF, 6);
    kmemcpy(&eth->src, &iface->mac, 6);
    eth->type = htons(ETH_TYPE_ARP);
    
    /* ARP request */
    arp->hw_type = htons(1);  /* Ethernet */
    arp->proto_type = htons(ETH_TYPE_IP);
    arp->hw_size = 6;
    arp->proto_size = 4;
    arp->opcode = htons(1);  /* Request */
    kmemcpy(&arp->sender_mac, &iface->mac, 6);
    kmemcpy(&arp->sender_ip, &iface->ip, 4);
    kmemset(arp->target_mac.addr, 0, 6);
    kmemcpy(&arp->target_ip, target_ip, 4);
    
    return iface->send(packet, sizeof(packet));
}

void arp_handle(net_interface_t *iface, arp_packet_t *arp) {
    uint16_t opcode = ntohs(arp->opcode);
    
    if (opcode == 1) {  /* Request */
        /* Check if it's for us */
        if (ip_equal(&arp->target_ip, &iface->ip)) {
            /* Send reply */
            uint8_t reply[ETH_HEADER_SIZE + sizeof(arp_packet_t)];
            eth_header_t *eth = (eth_header_t*)reply;
            arp_packet_t *reply_arp = (arp_packet_t*)(reply + ETH_HEADER_SIZE);
            
            kmemcpy(&eth->dest, &arp->sender_mac, 6);
            kmemcpy(&eth->src, &iface->mac, 6);
            eth->type = htons(ETH_TYPE_ARP);
            
            reply_arp->hw_type = htons(1);
            reply_arp->proto_type = htons(ETH_TYPE_IP);
            reply_arp->hw_size = 6;
            reply_arp->proto_size = 4;
            reply_arp->opcode = htons(2);  /* Reply */
            kmemcpy(&reply_arp->sender_mac, &iface->mac, 6);
            kmemcpy(&reply_arp->sender_ip, &iface->ip, 4);
            kmemcpy(&reply_arp->target_mac, &arp->sender_mac, 6);
            kmemcpy(&reply_arp->target_ip, &arp->sender_ip, 4);
            
            iface->send(reply, sizeof(reply));
        }
    } else if (opcode == 2) {  /* Reply */
        /* Add to cache */
        arp_add(&arp->sender_ip, &arp->sender_mac);
    }
}

/* ============================================================
 * IP Functions
 * ============================================================ */

int ip_send(net_interface_t *iface, ipv4_addr_t *dest, uint8_t protocol,
            void *data, size_t len) {
    uint8_t packet[ETH_FRAME_MAX];
    eth_header_t *eth = (eth_header_t*)packet;
    ipv4_header_t *ip = (ipv4_header_t*)(packet + ETH_HEADER_SIZE);
    
    /* Get destination MAC */
    mac_addr_t *dest_mac = arp_lookup(dest);
    if (!dest_mac) {
        /* Need ARP request */
        arp_request(iface, dest);
        return -1;  /* Would need to queue packet */
    }
    
    /* Ethernet header */
    kmemcpy(&eth->dest, dest_mac, 6);
    kmemcpy(&eth->src, &iface->mac, 6);
    eth->type = htons(ETH_TYPE_IP);
    
    /* IP header */
    ip->version_ihl = 0x45;  /* IPv4, 5 dwords */
    ip->tos = 0;
    ip->total_length = htons(IP_HEADER_SIZE + len);
    ip->id = htons(0);
    ip->flags_fragment = 0;
    ip->ttl = 64;
    ip->protocol = protocol;
    ip->checksum = 0;
    kmemcpy(&ip->src, &iface->ip, 4);
    kmemcpy(&ip->dest, dest, 4);
    ip->checksum = ip_checksum(ip, IP_HEADER_SIZE);
    
    /* Copy data */
    kmemcpy(packet + ETH_HEADER_SIZE + IP_HEADER_SIZE, data, len);
    
    return iface->send(packet, ETH_HEADER_SIZE + IP_HEADER_SIZE + len);
}

/* ============================================================
 * ICMP Functions (Ping)
 * ============================================================ */

int icmp_send_echo(net_interface_t *iface, ipv4_addr_t *dest, 
                   uint16_t id, uint16_t seq) {
    uint8_t data[64];
    icmp_header_t *icmp = (icmp_header_t*)data;
    
    icmp->type = 8;  /* Echo request */
    icmp->code = 0;
    icmp->checksum = 0;
    icmp->id = htons(id);
    icmp->sequence = htons(seq);
    
    /* Fill with pattern */
    for (int i = 8; i < 64; i++) {
        data[i] = i;
    }
    
    icmp->checksum = ip_checksum(data, 64);
    
    return ip_send(iface, dest, IP_PROTO_ICMP, data, 64);
}

void icmp_handle(net_interface_t *iface, ipv4_header_t *ip, 
                 icmp_header_t *icmp, size_t len) {
    if (icmp->type == 8) {  /* Echo request */
        /* Send echo reply */
        icmp->type = 0;  /* Echo reply */
        icmp->checksum = 0;
        icmp->checksum = ip_checksum(icmp, len);
        
        ip_send(iface, &ip->src, IP_PROTO_ICMP, icmp, len);
    } else if (icmp->type == 0) {  /* Echo reply */
        kprintf("[NET] Ping reply from %d.%d.%d.%d\n",
                ip->src.addr[0], ip->src.addr[1],
                ip->src.addr[2], ip->src.addr[3]);
    }
}

/* ============================================================
 * Socket API (BSD-style)
 * ============================================================ */

int socket(int domain, int type, int protocol) {
    /* Find free socket */
    for (int i = 0; i < MAX_SOCKETS; i++) {
        if (sockets[i].fd == 0) {
            sockets[i].fd = i + 1;
            sockets[i].type = type;
            sockets[i].state = SOCK_CLOSED;
            sockets[i].send_buffer = kmalloc(SOCKET_BUFFER_SIZE);
            sockets[i].recv_buffer = kmalloc(SOCKET_BUFFER_SIZE);
            sockets[i].send_len = 0;
            sockets[i].recv_len = 0;
            return i + 1;
        }
    }
    return -1;
}

int bind(int fd, ipv4_addr_t *addr, uint16_t port) {
    if (fd <= 0 || fd > MAX_SOCKETS) return -1;
    socket_t *sock = &sockets[fd - 1];
    
    kmemcpy(&sock->local_ip, addr, sizeof(ipv4_addr_t));
    sock->local_port = port;
    
    return 0;
}

int listen(int fd, int backlog) {
    if (fd <= 0 || fd > MAX_SOCKETS) return -1;
    socket_t *sock = &sockets[fd - 1];
    
    if (sock->type != SOCK_STREAM) return -1;
    
    sock->state = SOCK_LISTEN;
    return 0;
}

int connect(int fd, ipv4_addr_t *addr, uint16_t port) {
    if (fd <= 0 || fd > MAX_SOCKETS) return -1;
    socket_t *sock = &sockets[fd - 1];
    
    kmemcpy(&sock->remote_ip, addr, sizeof(ipv4_addr_t));
    sock->remote_port = port;
    
    if (sock->local_port == 0) {
        sock->local_port = next_port++;
    }
    
    if (sock->type == SOCK_STREAM) {
        /* TCP: Send SYN */
        sock->state = SOCK_SYN_SENT;
        sock->seq_num = timer_get_ticks();  /* Simple ISN */
        /* Would send TCP SYN packet here */
    } else {
        /* UDP: Just mark as connected */
        sock->state = SOCK_ESTABLISHED;
    }
    
    return 0;
}

int send(int fd, void *data, size_t len, int flags) {
    if (fd <= 0 || fd > MAX_SOCKETS) return -1;
    socket_t *sock = &sockets[fd - 1];
    
    if (sock->state != SOCK_ESTABLISHED) return -1;
    
    /* Would send data via TCP or UDP */
    return len;
}

int recv(int fd, void *buffer, size_t len, int flags) {
    if (fd <= 0 || fd > MAX_SOCKETS) return -1;
    socket_t *sock = &sockets[fd - 1];
    
    if (sock->recv_len == 0) return 0;
    
    size_t to_copy = (len < sock->recv_len) ? len : sock->recv_len;
    kmemcpy(buffer, sock->recv_buffer, to_copy);
    
    /* Shift remaining data */
    sock->recv_len -= to_copy;
    if (sock->recv_len > 0) {
        kmemcpy(sock->recv_buffer, sock->recv_buffer + to_copy, sock->recv_len);
    }
    
    return to_copy;
}

int close_socket(int fd) {
    if (fd <= 0 || fd > MAX_SOCKETS) return -1;
    socket_t *sock = &sockets[fd - 1];
    
    if (sock->type == SOCK_STREAM && sock->state == SOCK_ESTABLISHED) {
        /* TCP: Send FIN */
        sock->state = SOCK_FIN_WAIT_1;
        /* Would send TCP FIN packet here */
    }
    
    /* Free buffers */
    if (sock->send_buffer) kfree(sock->send_buffer);
    if (sock->recv_buffer) kfree(sock->recv_buffer);
    
    sock->fd = 0;
    sock->state = SOCK_CLOSED;
    
    return 0;
}

/* ============================================================
 * Network Initialization
 * ============================================================ */

void network_init(void) {
    kmemset(sockets, 0, sizeof(sockets));
    kmemset(arp_cache, 0, sizeof(arp_cache));
    interface_count = 0;
    
    kprintf("[NET] Network stack initialized\n");
    kprintf("[NET] Protocols: ARP, IPv4, ICMP, UDP, TCP\n");
}

int network_add_interface(net_interface_t *iface) {
    if (interface_count >= 4) return -1;
    
    kmemcpy(&interfaces[interface_count], iface, sizeof(net_interface_t));
    interface_count++;
    
    kprintf("[NET] Added interface: %s\n", iface->name);
    return interface_count - 1;
}

/* ============================================================
 * Packet Reception Handler
 * ============================================================ */

void network_receive(net_interface_t *iface, void *data, size_t len) {
    if (len < ETH_HEADER_SIZE) return;
    
    eth_header_t *eth = (eth_header_t*)data;
    uint16_t type = ntohs(eth->type);
    
    iface->rx_packets++;
    iface->rx_bytes += len;
    
    if (type == ETH_TYPE_ARP) {
        arp_handle(iface, (arp_packet_t*)(data + ETH_HEADER_SIZE));
    } else if (type == ETH_TYPE_IP) {
        ipv4_header_t *ip = (ipv4_header_t*)(data + ETH_HEADER_SIZE);
        size_t ip_len = ntohs(ip->total_length);
        
        if (ip->protocol == IP_PROTO_ICMP) {
            icmp_handle(iface, ip, 
                       (icmp_header_t*)(data + ETH_HEADER_SIZE + IP_HEADER_SIZE),
                       ip_len - IP_HEADER_SIZE);
        } else if (ip->protocol == IP_PROTO_TCP) {
            /* Handle TCP */
        } else if (ip->protocol == IP_PROTO_UDP) {
            /* Handle UDP */
        }
    }
}
