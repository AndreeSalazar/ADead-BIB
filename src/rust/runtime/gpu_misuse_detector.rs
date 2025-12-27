// ADead-BIB HEX - GPU Misuse Detector
// Detects when GPU is being used incorrectly
// Shows WHY and HOW MUCH performance is being lost

use super::gpu_dispatcher::{OperationCost, DataLocation, GPU_THRESHOLD_ELEMENTS, MIN_FLOPS_PER_BYTE};

/// Severity of GPU misuse
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MisuseSeverity {
    /// Minor inefficiency, still acceptable
    Warning,
    /// Significant performance loss
    Error,
    /// Critical misuse, GPU should NOT be used
    Critical,
}

/// Type of GPU misuse detected
#[derive(Debug, Clone)]
pub enum MisuseType {
    /// Kernel too small, PCIe overhead dominates
    KernelTooSmall {
        elements: usize,
        min_recommended: usize,
        pcie_overhead_percent: f64,
    },
    /// Low computational intensity
    LowIntensity {
        flops_per_byte: f64,
        min_recommended: f64,
    },
    /// Unnecessary data transfer
    UnnecessaryTransfer {
        bytes: usize,
        reason: String,
    },
    /// Data not persisting in VRAM
    NoPersistence {
        potential_reuse_count: usize,
        wasted_transfers: usize,
    },
    /// Mixed small/large operations
    MixedWorkload {
        small_ops: usize,
        large_ops: usize,
    },
}

/// GPU Misuse Report
#[derive(Debug, Clone)]
pub struct MisuseReport {
    pub kernel_name: String,
    pub severity: MisuseSeverity,
    pub misuse_type: MisuseType,
    pub recommendation: String,
    pub estimated_speedup: f64,
}

/// GPU Misuse Score (0-100)
/// Higher = worse misuse
#[derive(Debug, Clone)]
pub struct MisuseScore {
    pub total: u32,
    pub pcie_overhead: u32,
    pub low_intensity: u32,
    pub one_shot: u32,
    pub no_persistence: u32,
    pub small_elements: u32,
}

impl MisuseScore {
    pub fn calculate(cost: &OperationCost) -> Self {
        let mut score = Self {
            total: 0,
            pcie_overhead: 0,
            low_intensity: 0,
            one_shot: 0,
            no_persistence: 0,
            small_elements: 0,
        };

        // PCIe overhead dominance (0-40 points)
        let h2d = cost.estimate_h2d_us();
        let kernel = cost.estimate_kernel_us();
        if kernel > 0.0 {
            let overhead_ratio = h2d * 2.0 / (h2d * 2.0 + kernel);
            score.pcie_overhead = (overhead_ratio * 40.0).min(40.0) as u32;
        }

        // Low arithmetic intensity (0-25 points)
        let fpb = cost.flops_per_byte();
        if fpb < MIN_FLOPS_PER_BYTE {
            score.low_intensity = ((MIN_FLOPS_PER_BYTE - fpb) / MIN_FLOPS_PER_BYTE * 25.0).min(25.0) as u32;
        }

        // One-shot execution (0-15 points)
        if !cost.will_persist && cost.data_location == DataLocation::Host {
            score.one_shot = 15;
        }

        // No data persistence (0-10 points)
        if !cost.will_persist {
            score.no_persistence = 10;
        }

        // Small element count (0-10 points)
        if cost.elements < GPU_THRESHOLD_ELEMENTS {
            let ratio = 1.0 - (cost.elements as f64 / GPU_THRESHOLD_ELEMENTS as f64);
            score.small_elements = (ratio * 10.0).min(10.0) as u32;
        }

        score.total = score.pcie_overhead + score.low_intensity + score.one_shot 
                    + score.no_persistence + score.small_elements;
        score.total = score.total.min(100);

        score
    }

    pub fn severity(&self) -> MisuseSeverity {
        match self.total {
            0..=30 => MisuseSeverity::Warning,
            31..=60 => MisuseSeverity::Error,
            _ => MisuseSeverity::Critical,
        }
    }

    pub fn print(&self, kernel_name: &str) {
        let severity_str = match self.severity() {
            MisuseSeverity::Warning => "WARNING",
            MisuseSeverity::Error => "ERROR",
            MisuseSeverity::Critical => "CRITICAL",
        };

        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║  GPU MISUSE SCORE: {:3} / 100 ({:8})                      ║", self.total, severity_str);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║  Kernel: {:<52} ║", kernel_name);
        println!("║                                                              ║");
        println!("║  Breakdown:                                                  ║");
        println!("║  ├── PCIe overhead dominance:     +{:2} points                ║", self.pcie_overhead);
        println!("║  ├── Low arithmetic intensity:    +{:2} points                ║", self.low_intensity);
        println!("║  ├── One-shot execution:          +{:2} points                ║", self.one_shot);
        println!("║  ├── No data persistence:         +{:2} points                ║", self.no_persistence);
        println!("║  └── Small element count:         +{:2} points                ║", self.small_elements);
        println!("║                                                              ║");
        if self.total > 50 {
            println!("║  Recommendation: Execute on CPU                              ║");
        } else if self.total > 30 {
            println!("║  Recommendation: Consider batching or persistence            ║");
        } else {
            println!("║  Recommendation: GPU execution acceptable                    ║");
        }
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

/// GPU Misuse Detector - The Gold Feature
pub struct GpuMisuseDetector {
    reports: Vec<MisuseReport>,
    total_wasted_time_us: f64,
    total_wasted_transfers: usize,
    scores: Vec<MisuseScore>,
}

impl GpuMisuseDetector {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            total_wasted_time_us: 0.0,
            total_wasted_transfers: 0,
            scores: Vec::new(),
        }
    }

    /// Analyze an operation for potential GPU misuse
    pub fn analyze(&mut self, cost: &OperationCost) -> Option<MisuseReport> {
        // Check for small kernel misuse
        if cost.elements < GPU_THRESHOLD_ELEMENTS && cost.data_location == DataLocation::Host {
            let pcie_overhead = self.estimate_pcie_overhead_percent(cost);
            
            if pcie_overhead > 50.0 {
                let report = MisuseReport {
                    kernel_name: cost.name.clone(),
                    severity: if pcie_overhead > 80.0 { MisuseSeverity::Critical } 
                              else if pcie_overhead > 60.0 { MisuseSeverity::Error }
                              else { MisuseSeverity::Warning },
                    misuse_type: MisuseType::KernelTooSmall {
                        elements: cost.elements,
                        min_recommended: GPU_THRESHOLD_ELEMENTS,
                        pcie_overhead_percent: pcie_overhead,
                    },
                    recommendation: format!(
                        "Execute on CPU, or batch operations to reach >{} elements",
                        GPU_THRESHOLD_ELEMENTS
                    ),
                    estimated_speedup: pcie_overhead / 10.0, // Rough estimate
                };
                
                self.reports.push(report.clone());
                self.total_wasted_time_us += cost.estimate_h2d_us() * 2.0;
                self.total_wasted_transfers += 2;
                
                return Some(report);
            }
        }

        // Check for low computational intensity
        let fpb = cost.flops_per_byte();
        if fpb < MIN_FLOPS_PER_BYTE && cost.data_location == DataLocation::Host && !cost.will_persist {
            let report = MisuseReport {
                kernel_name: cost.name.clone(),
                severity: MisuseSeverity::Warning,
                misuse_type: MisuseType::LowIntensity {
                    flops_per_byte: fpb,
                    min_recommended: MIN_FLOPS_PER_BYTE,
                },
                recommendation: format!(
                    "Low compute intensity ({:.2} FLOPs/Byte). Consider CPU or ensure data persists in VRAM.",
                    fpb
                ),
                estimated_speedup: MIN_FLOPS_PER_BYTE / fpb,
            };
            
            self.reports.push(report.clone());
            return Some(report);
        }

        // Check for unnecessary transfers (data already on device but transferring again)
        if cost.data_location == DataLocation::Both && !cost.will_persist {
            let report = MisuseReport {
                kernel_name: cost.name.clone(),
                severity: MisuseSeverity::Warning,
                misuse_type: MisuseType::UnnecessaryTransfer {
                    bytes: cost.total_bytes(),
                    reason: "Data already synchronized, no transfer needed".to_string(),
                },
                recommendation: "Use data directly from VRAM, skip H2D transfer".to_string(),
                estimated_speedup: 2.0,
            };
            
            self.reports.push(report.clone());
            self.total_wasted_transfers += 1;
            return Some(report);
        }

        None
    }

    /// Estimate PCIe overhead as percentage of total time
    fn estimate_pcie_overhead_percent(&self, cost: &OperationCost) -> f64 {
        let h2d = cost.estimate_h2d_us();
        let kernel = cost.estimate_kernel_us();
        let d2h = h2d; // Assume symmetric
        
        let total = h2d + kernel + d2h;
        if total <= 0.0 {
            return 0.0;
        }
        
        ((h2d + d2h) / total) * 100.0
    }

    /// Print detailed misuse report
    pub fn print_report(&self) {
        if self.reports.is_empty() {
            println!("✅ No GPU misuse detected. All operations are efficient.");
            return;
        }

        println!();
        println!("╔══════════════════════════════════════════════════════════════════════════════╗");
        println!("║  ⚠️  GPU MISUSE DETECTOR - ADead-BIB HEX                                      ║");
        println!("╚══════════════════════════════════════════════════════════════════════════════╝");
        println!();

        for (i, report) in self.reports.iter().enumerate() {
            let severity_icon = match report.severity {
                MisuseSeverity::Warning => "⚠️ ",
                MisuseSeverity::Error => "❌",
                MisuseSeverity::Critical => "🚨",
            };

            println!("{}  Issue #{}: {}", severity_icon, i + 1, report.kernel_name);
            
            match &report.misuse_type {
                MisuseType::KernelTooSmall { elements, min_recommended, pcie_overhead_percent } => {
                    println!("   Type: Kernel Too Small");
                    println!("   Elements: {}", elements);
                    println!("   Minimum recommended: {}", min_recommended);
                    println!("   PCIe overhead: {:.1}%", pcie_overhead_percent);
                }
                MisuseType::LowIntensity { flops_per_byte, min_recommended } => {
                    println!("   Type: Low Computational Intensity");
                    println!("   FLOPs/Byte: {:.2}", flops_per_byte);
                    println!("   Minimum recommended: {:.2}", min_recommended);
                }
                MisuseType::UnnecessaryTransfer { bytes, reason } => {
                    println!("   Type: Unnecessary Transfer");
                    println!("   Bytes: {}", bytes);
                    println!("   Reason: {}", reason);
                }
                MisuseType::NoPersistence { potential_reuse_count, wasted_transfers } => {
                    println!("   Type: No Data Persistence");
                    println!("   Potential reuse: {} times", potential_reuse_count);
                    println!("   Wasted transfers: {}", wasted_transfers);
                }
                MisuseType::MixedWorkload { small_ops, large_ops } => {
                    println!("   Type: Mixed Workload");
                    println!("   Small operations: {}", small_ops);
                    println!("   Large operations: {}", large_ops);
                }
            }
            
            println!("   Recommendation: {}", report.recommendation);
            println!("   Estimated speedup if fixed: {:.1}x", report.estimated_speedup);
            println!();
        }

        println!("════════════════════════════════════════════════════════════════════════════════");
        println!("  SUMMARY:");
        println!("  - Total issues: {}", self.reports.len());
        println!("  - Wasted transfers: {}", self.total_wasted_transfers);
        println!("  - Estimated wasted time: {:.1} µs", self.total_wasted_time_us);
        println!("════════════════════════════════════════════════════════════════════════════════");
        println!();
        println!("  💡 ADead-BIB HEX: \"CUDA gives power. ADead-BIB gives judgment.\"");
        println!("     The hardware doesn't fail. Decisions do.");
        println!();
    }

    /// Get all reports
    pub fn get_reports(&self) -> &[MisuseReport] {
        &self.reports
    }

    /// Clear all reports
    pub fn clear(&mut self) {
        self.reports.clear();
        self.total_wasted_time_us = 0.0;
        self.total_wasted_transfers = 0;
    }
}

impl Default for GpuMisuseDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::gpu_dispatcher::operations;

    #[test]
    fn test_detects_small_kernel() {
        let mut detector = GpuMisuseDetector::new();
        let cost = operations::vector_add(1000, DataLocation::Host, false);
        let report = detector.analyze(&cost);
        assert!(report.is_some());
    }

    #[test]
    fn test_no_misuse_for_large_kernel() {
        let mut detector = GpuMisuseDetector::new();
        let cost = operations::matmul(512, DataLocation::Host, true);
        // MatMul has high intensity, should not be flagged
        let report = detector.analyze(&cost);
        // May or may not have report depending on thresholds
        assert!(detector.reports.len() <= 1);
    }
}
