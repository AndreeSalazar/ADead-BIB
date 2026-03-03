/*
 * FastOS v2.0 — AHCI (SATA) Driver
 * ADead-BIB Native Operating System
 * 
 * Advanced Host Controller Interface for SATA drives
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* AHCI Registers (HBA Memory) */
#define AHCI_CAP        0x00    /* Host Capabilities */
#define AHCI_GHC        0x04    /* Global Host Control */
#define AHCI_IS         0x08    /* Interrupt Status */
#define AHCI_PI         0x0C    /* Ports Implemented */
#define AHCI_VS         0x10    /* Version */
#define AHCI_CCC_CTL    0x14    /* Command Completion Coalescing Control */
#define AHCI_CCC_PORTS  0x18    /* CCC Ports */
#define AHCI_EM_LOC     0x1C    /* Enclosure Management Location */
#define AHCI_EM_CTL     0x20    /* Enclosure Management Control */
#define AHCI_CAP2       0x24    /* Host Capabilities Extended */
#define AHCI_BOHC       0x28    /* BIOS/OS Handoff Control */

/* Port Registers (offset from port base) */
#define PORT_CLB        0x00    /* Command List Base Address */
#define PORT_CLBU       0x04    /* Command List Base Address Upper */
#define PORT_FB         0x08    /* FIS Base Address */
#define PORT_FBU        0x0C    /* FIS Base Address Upper */
#define PORT_IS         0x10    /* Interrupt Status */
#define PORT_IE         0x14    /* Interrupt Enable */
#define PORT_CMD        0x18    /* Command and Status */
#define PORT_TFD        0x20    /* Task File Data */
#define PORT_SIG        0x24    /* Signature */
#define PORT_SSTS       0x28    /* SATA Status */
#define PORT_SCTL       0x2C    /* SATA Control */
#define PORT_SERR       0x30    /* SATA Error */
#define PORT_SACT       0x34    /* SATA Active */
#define PORT_CI         0x38    /* Command Issue */
#define PORT_SNTF       0x3C    /* SATA Notification */
#define PORT_FBS        0x40    /* FIS-based Switching Control */

/* Port Command bits */
#define PORT_CMD_ST     (1 << 0)    /* Start */
#define PORT_CMD_SUD    (1 << 1)    /* Spin-Up Device */
#define PORT_CMD_POD    (1 << 2)    /* Power On Device */
#define PORT_CMD_CLO    (1 << 3)    /* Command List Override */
#define PORT_CMD_FRE    (1 << 4)    /* FIS Receive Enable */
#define PORT_CMD_FR     (1 << 14)   /* FIS Receive Running */
#define PORT_CMD_CR     (1 << 15)   /* Command List Running */

/* Device signatures */
#define SATA_SIG_ATA    0x00000101  /* SATA drive */
#define SATA_SIG_ATAPI  0xEB140101  /* SATAPI drive */
#define SATA_SIG_SEMB   0xC33C0101  /* Enclosure management bridge */
#define SATA_SIG_PM     0x96690101  /* Port multiplier */

/* AHCI device types */
typedef enum {
    AHCI_DEV_NULL = 0,
    AHCI_DEV_SATA,
    AHCI_DEV_SATAPI,
    AHCI_DEV_SEMB,
    AHCI_DEV_PM
} ahci_dev_type_t;

/* Port info */
typedef struct {
    int port_num;
    ahci_dev_type_t type;
    uint32_t signature;
    int present;
    uint64_t sector_count;
    char model[41];
} ahci_port_info_t;

/* AHCI controller state */
static uint64_t ahci_base = 0;
static uint32_t ports_implemented = 0;
static int ahci_initialized = 0;
static ahci_port_info_t ahci_ports[32];
static int ahci_port_count = 0;

/* Read AHCI register */
static uint32_t ahci_read(uint32_t reg) {
    return *(volatile uint32_t*)(ahci_base + reg);
}

/* Write AHCI register */
static void ahci_write(uint32_t reg, uint32_t value) {
    *(volatile uint32_t*)(ahci_base + reg) = value;
}

/* Read port register */
static uint32_t port_read(int port, uint32_t reg) {
    uint64_t port_base = ahci_base + 0x100 + (port * 0x80);
    return *(volatile uint32_t*)(port_base + reg);
}

/* Write port register */
static void port_write(int port, uint32_t reg, uint32_t value) {
    uint64_t port_base = ahci_base + 0x100 + (port * 0x80);
    *(volatile uint32_t*)(port_base + reg) = value;
}

/* Get device type from signature */
static ahci_dev_type_t ahci_check_type(int port) {
    uint32_t ssts = port_read(port, PORT_SSTS);
    
    uint8_t ipm = (ssts >> 8) & 0x0F;
    uint8_t det = ssts & 0x0F;
    
    /* Check device detection */
    if (det != 3) return AHCI_DEV_NULL;  /* No device */
    if (ipm != 1) return AHCI_DEV_NULL;  /* Not active */
    
    uint32_t sig = port_read(port, PORT_SIG);
    
    switch (sig) {
        case SATA_SIG_ATAPI: return AHCI_DEV_SATAPI;
        case SATA_SIG_SEMB:  return AHCI_DEV_SEMB;
        case SATA_SIG_PM:    return AHCI_DEV_PM;
        default:             return AHCI_DEV_SATA;
    }
}

/* Get device type name */
const char* ahci_type_name(ahci_dev_type_t type) {
    switch (type) {
        case AHCI_DEV_SATA:   return "SATA";
        case AHCI_DEV_SATAPI: return "SATAPI";
        case AHCI_DEV_SEMB:   return "SEMB";
        case AHCI_DEV_PM:     return "PM";
        default:              return "None";
    }
}

/* Stop port command engine */
static void port_stop_cmd(int port) {
    uint32_t cmd = port_read(port, PORT_CMD);
    
    /* Clear ST */
    cmd &= ~PORT_CMD_ST;
    port_write(port, PORT_CMD, cmd);
    
    /* Wait for CR to clear */
    int timeout = 500;
    while (timeout--) {
        if (!(port_read(port, PORT_CMD) & PORT_CMD_CR)) break;
        /* Small delay */
        for (volatile int i = 0; i < 1000; i++);
    }
    
    /* Clear FRE */
    cmd = port_read(port, PORT_CMD);
    cmd &= ~PORT_CMD_FRE;
    port_write(port, PORT_CMD, cmd);
    
    /* Wait for FR to clear */
    timeout = 500;
    while (timeout--) {
        if (!(port_read(port, PORT_CMD) & PORT_CMD_FR)) break;
        for (volatile int i = 0; i < 1000; i++);
    }
}

/* Start port command engine */
static void port_start_cmd(int port) {
    /* Wait for CR to clear */
    int timeout = 500;
    while (timeout--) {
        if (!(port_read(port, PORT_CMD) & PORT_CMD_CR)) break;
        for (volatile int i = 0; i < 1000; i++);
    }
    
    uint32_t cmd = port_read(port, PORT_CMD);
    cmd |= PORT_CMD_FRE;
    cmd |= PORT_CMD_ST;
    port_write(port, PORT_CMD, cmd);
}

/* Print hex */
static void ahci_print_hex32(uint32_t val) {
    const char hex[] = "0123456789ABCDEF";
    for (int i = 28; i >= 0; i -= 4) {
        vga_putchar(hex[(val >> i) & 0xF]);
    }
}

/* Probe ports */
void ahci_probe_ports(void) {
    ports_implemented = ahci_read(AHCI_PI);
    
    kputs("[AHCI] Probing ports (PI=0x");
    ahci_print_hex32(ports_implemented);
    kputs(")...\n");
    
    ahci_port_count = 0;
    
    for (int i = 0; i < 32; i++) {
        if (ports_implemented & (1 << i)) {
            ahci_dev_type_t type = ahci_check_type(i);
            
            if (type != AHCI_DEV_NULL) {
                ahci_ports[ahci_port_count].port_num = i;
                ahci_ports[ahci_port_count].type = type;
                ahci_ports[ahci_port_count].signature = port_read(i, PORT_SIG);
                ahci_ports[ahci_port_count].present = 1;
                
                kputs("      Port ");
                vga_putchar('0' + i);
                kputs(": ");
                kputs(ahci_type_name(type));
                kputs(" device\n");
                
                ahci_port_count++;
            }
        }
    }
    
    kputs("[AHCI] Found ");
    vga_putchar('0' + ahci_port_count);
    kputs(" device(s)\n");
}

/* Initialize AHCI controller */
int ahci_init_controller(uint64_t bar) {
    ahci_base = bar & ~0xFFF;  /* Align to page */
    
    kputs("[AHCI] Controller at 0x");
    ahci_print_hex32((uint32_t)(ahci_base >> 32));
    ahci_print_hex32((uint32_t)ahci_base);
    kputs("\n");
    
    /* Read version */
    uint32_t version = ahci_read(AHCI_VS);
    uint16_t major = (version >> 16) & 0xFFFF;
    uint16_t minor = version & 0xFFFF;
    
    kputs("[AHCI] Version ");
    vga_putchar('0' + major);
    kputs(".");
    vga_putchar('0' + (minor >> 8));
    vga_putchar('0' + (minor & 0xF));
    kputs("\n");
    
    /* Read capabilities */
    uint32_t cap = ahci_read(AHCI_CAP);
    int num_ports = (cap & 0x1F) + 1;
    int num_slots = ((cap >> 8) & 0x1F) + 1;
    int supports_64bit = (cap >> 31) & 1;
    int supports_ncq = (cap >> 30) & 1;
    
    kputs("[AHCI] Ports: ");
    vga_putchar('0' + (num_ports / 10));
    vga_putchar('0' + (num_ports % 10));
    kputs(", Slots: ");
    vga_putchar('0' + (num_slots / 10));
    vga_putchar('0' + (num_slots % 10));
    if (supports_64bit) kputs(", 64-bit");
    if (supports_ncq) kputs(", NCQ");
    kputs("\n");
    
    /* Enable AHCI mode */
    uint32_t ghc = ahci_read(AHCI_GHC);
    ghc |= (1 << 31);  /* AHCI Enable */
    ahci_write(AHCI_GHC, ghc);
    
    /* Probe ports */
    ahci_probe_ports();
    
    ahci_initialized = 1;
    return 0;
}

/* Initialize from PCI device */
void ahci_init(void) {
    /* Get AHCI controller from PCI */
    extern pci_device_t* pci_get_ahci_controller(void);
    pci_device_t *dev = pci_get_ahci_controller();
    
    if (!dev) {
        kputs("[AHCI] No AHCI controller found\n");
        return;
    }
    
    /* BAR5 contains ABAR (AHCI Base Address) */
    uint64_t abar = dev->bar[5] & ~0xF;
    
    if (abar == 0) {
        kputs("[AHCI] Invalid ABAR\n");
        return;
    }
    
    ahci_init_controller(abar);
}

/* Get port count */
int ahci_get_port_count(void) {
    return ahci_port_count;
}

/* Get port info */
ahci_port_info_t* ahci_get_port(int index) {
    if (index < 0 || index >= ahci_port_count) return NULL;
    return &ahci_ports[index];
}

/* Check if initialized */
int ahci_is_initialized(void) {
    return ahci_initialized;
}
