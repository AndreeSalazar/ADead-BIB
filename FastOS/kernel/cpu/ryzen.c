/* FastOS v2.1 — Ryzen 5 5600X CPU Module
 * ADead-BIB Native OS
 *
 * AMD Ryzen 5 5600X (Vermeer / Zen 3):
 *   - 6 cores / 12 threads (SMT)
 *   - Base: 3.7 GHz, Boost: 4.6 GHz
 *   - L1i: 32KB/core, L1d: 32KB/core
 *   - L2: 512KB/core (3MB total)
 *   - L3: 32MB shared (1 CCD, 1 CCX)
 *   - TDP: 65W
 *   - Socket: AM4
 *   - Infinity Fabric: up to 2000 MHz
 *   - Memory: DDR4-3200 dual channel
 *
 * CPUID Detection:
 *   Leaf 0: Vendor = "AuthenticAMD"
 *   Leaf 1: Family 0x19, Model 0x21 (Zen 3 Vermeer)
 *     EAX bits: [27:20]=ExtFamily, [19:16]=ExtModel, [11:8]=Family, [7:4]=Model
 *     Family = BaseFamily + ExtFamily = 0x0F + 0x0A = 0x19
 *     Model  = (ExtModel << 4) | BaseModel = (0x2 << 4) | 0x1 = 0x21
 *   Leaf 7: EBX bit 5 = AVX2
 *   Leaf 0x80000002-4: Brand "AMD Ryzen 5 5600X 6-Core Processor"
 *   Leaf 0x80000005: L1 cache info
 *   Leaf 0x80000006: L2/L3 cache info
 *     ECX[31:16] = L2 size in KB (512)
 *     EDX[31:18] = L3 size in 512KB units (64 = 32MB)
 *
 * Core detection (CPUID leaf 0x8000001E):
 *   EBX[15:8] = threads per core - 1
 *   ECX[10:8] = nodes per processor - 1
 *
 * Topology (CPUID leaf 0x0B):
 *   Subleaf 0: SMT level (threads per core)
 *   Subleaf 1: Core level (logical processors per package)
 *
 * Feature flags (inline detection):
 *   Leaf 1 ECX: SSE3(0), SSSE3(9), SSE4.1(19), SSE4.2(20), AES(25), AVX(28)
 *   Leaf 7 EBX: AVX2(5), BMI1(3), BMI2(8), SHA(29)
 *   Leaf 7 ECX: (empty for 5600X)
 *   Leaf 0x80000001 ECX: ABM/LZCNT(5), SSE4a(6), XOP(11), FMA4(16)
 *
 * Power management (CPUID leaf 6):
 *   Precision Boost 2 (PB2) — automatic single/multi core boost
 *   CPPC2 (Collaborative Power Performance Control)
 *
 * Inline kernel_main() integration:
 *   int cpu_family, cpu_model, cpu_cores, cpu_l3;
 *   tmp = __cpuid_eax(1);
 *   cpu_family = ((tmp >> 8) & 0xF) + ((tmp >> 20) & 0xFF);
 *   cpu_model = (((tmp >> 16) & 0xF) << 4) | ((tmp >> 4) & 0xF);
 *   // L3 cache
 *   tmp = __cpuid_edx(0x80000006);
 *   cpu_l3 = ((tmp >> 18) & 0x3FFF) / 2;  // in MB
 *   // Core count from leaf 0x8000001E or brand string parsing
 */
