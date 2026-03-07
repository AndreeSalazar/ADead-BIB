#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
int main() { struct sockaddr_in addr; memset(&addr, 0, sizeof(addr)); addr.sin_family = 2; printf("Network ok\n"); return 0; }