// ============================================================
// ADead-BIB C Example — Network Programming
// ============================================================
// Sockets, HTTP client logic, DNS resolution patterns —
// lo que necesita FastOS para red desde cero.
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Network headers
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <unistd.h>
#include <fcntl.h>
#include <poll.h>

// ==================== HTTP Request Builder ====================

struct HttpRequest {
    char method[16];
    char path[256];
    char host[256];
    int port;
    char headers[2048];
    int header_count;
};

void http_init(struct HttpRequest *req, const char *method, const char *host, const char *path) {
    strcpy(req->method, method);
    strcpy(req->host, host);
    strcpy(req->path, path);
    req->port = 80;
    req->headers[0] = '\0';
    req->header_count = 0;
}

void http_add_header(struct HttpRequest *req, const char *name, const char *value) {
    char line[512];
    sprintf(line, "%s: %s\r\n", name, value);
    strcat(req->headers, line);
    req->header_count++;
}

int http_build(struct HttpRequest *req, char *buffer, int buf_size) {
    return snprintf(buffer, buf_size,
        "%s %s HTTP/1.1\r\n"
        "Host: %s\r\n"
        "%s"
        "Connection: close\r\n"
        "\r\n",
        req->method, req->path, req->host, req->headers);
}

// ==================== IP Address Utilities ====================

struct IPv4Addr {
    unsigned char octets[4];
};

struct IPv4Addr ipv4_from_string(const char *str) {
    struct IPv4Addr addr;
    int a, b, c, d;
    sscanf(str, "%d.%d.%d.%d", &a, &b, &c, &d);
    addr.octets[0] = (unsigned char)a;
    addr.octets[1] = (unsigned char)b;
    addr.octets[2] = (unsigned char)c;
    addr.octets[3] = (unsigned char)d;
    return addr;
}

unsigned int ipv4_to_uint(struct IPv4Addr addr) {
    return ((unsigned int)addr.octets[0] << 24) |
           ((unsigned int)addr.octets[1] << 16) |
           ((unsigned int)addr.octets[2] << 8)  |
           ((unsigned int)addr.octets[3]);
}

int ipv4_is_private(struct IPv4Addr addr) {
    // 10.0.0.0/8
    if (addr.octets[0] == 10) return 1;
    // 172.16.0.0/12
    if (addr.octets[0] == 172 && (addr.octets[1] >= 16 && addr.octets[1] <= 31)) return 1;
    // 192.168.0.0/16
    if (addr.octets[0] == 192 && addr.octets[1] == 168) return 1;
    // 127.0.0.0/8
    if (addr.octets[0] == 127) return 1;
    return 0;
}

int ipv4_is_multicast(struct IPv4Addr addr) {
    return addr.octets[0] >= 224 && addr.octets[0] <= 239;
}

// ==================== Simple URL Parser ====================

struct URL {
    char scheme[16];
    char host[256];
    int port;
    char path[512];
    char query[512];
};

int url_parse(const char *url_str, struct URL *url) {
    memset(url, 0, sizeof(struct URL));
    url->port = 80;

    const char *p = url_str;

    // Parse scheme
    const char *colon = strchr(p, ':');
    if (colon != NULL && colon[1] == '/' && colon[2] == '/') {
        int len = (int)(colon - p);
        if (len > 15) len = 15;
        strncpy(url->scheme, p, len);
        url->scheme[len] = '\0';
        p = colon + 3;
        if (strcmp(url->scheme, "https") == 0) {
            url->port = 443;
        }
    } else {
        strcpy(url->scheme, "http");
    }

    // Parse host (until / or :)
    int hi = 0;
    while (*p && *p != '/' && *p != ':' && *p != '?' && hi < 255) {
        url->host[hi++] = *p++;
    }
    url->host[hi] = '\0';

    // Parse port
    if (*p == ':') {
        p++;
        url->port = atoi(p);
        while (*p && *p >= '0' && *p <= '9') p++;
    }

    // Parse path
    if (*p == '/') {
        int pi = 0;
        while (*p && *p != '?' && pi < 511) {
            url->path[pi++] = *p++;
        }
        url->path[pi] = '\0';
    } else {
        strcpy(url->path, "/");
    }

    // Parse query
    if (*p == '?') {
        p++;
        int qi = 0;
        while (*p && qi < 511) {
            url->query[qi++] = *p++;
        }
        url->query[qi] = '\0';
    }

    return 1;
}

// ==================== Byte Order ====================

unsigned short my_htons(unsigned short val) {
    return (val >> 8) | (val << 8);
}

unsigned int my_htonl(unsigned int val) {
    return ((val & 0xFF000000) >> 24) |
           ((val & 0x00FF0000) >> 8)  |
           ((val & 0x0000FF00) << 8)  |
           ((val & 0x000000FF) << 24);
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Network Programming ===\n\n");

    // HTTP Request
    printf("HTTP Request Builder:\n");
    struct HttpRequest req;
    http_init(&req, "GET", "example.com", "/api/data");
    http_add_header(&req, "Accept", "application/json");
    http_add_header(&req, "User-Agent", "ADead-BIB/1.0");

    char buffer[4096];
    int len = http_build(&req, buffer, sizeof(buffer));
    printf("  Request (%d bytes):\n%s", len, buffer);

    // IP addresses
    printf("IP Addresses:\n");
    const char *ips[] = {
        "192.168.1.1",
        "10.0.0.1",
        "172.16.0.1",
        "8.8.8.8",
        "127.0.0.1",
        "224.0.0.1"
    };
    int num_ips = 6;

    for (int i = 0; i < num_ips; i++) {
        struct IPv4Addr addr = ipv4_from_string(ips[i]);
        unsigned int numeric = ipv4_to_uint(addr);
        printf("  %-16s → 0x%08x  private=%d  multicast=%d\n",
               ips[i], numeric, ipv4_is_private(addr), ipv4_is_multicast(addr));
    }

    // URL parsing
    printf("\nURL Parser:\n");
    const char *urls[] = {
        "https://example.com/path/to/resource?key=value",
        "http://localhost:8080/api",
        "https://api.adead-bib.dev:443/v1/compile"
    };
    int num_urls = 3;

    for (int i = 0; i < num_urls; i++) {
        struct URL url;
        url_parse(urls[i], &url);
        printf("  URL: %s\n", urls[i]);
        printf("    scheme=%s host=%s port=%d path=%s query=%s\n",
               url.scheme, url.host, url.port, url.path, url.query);
    }

    // Byte order
    printf("\nByte Order:\n");
    printf("  htons(0x1234) = 0x%04x\n", my_htons(0x1234));
    printf("  htonl(0x12345678) = 0x%08x\n", my_htonl(0x12345678));

    // sockaddr_in structure
    printf("\nSocket Address:\n");
    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = 2;
    addr.sin_port = my_htons(8080);
    printf("  Family: %d\n", addr.sin_family);
    printf("  Port: %d (network order: 0x%04x)\n", 8080, addr.sin_port);
    printf("  sizeof(sockaddr_in) = %lu\n", (unsigned long)sizeof(struct sockaddr_in));

    printf("\n=== Complete ===\n");
    return 0;
}
