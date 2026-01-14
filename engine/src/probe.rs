use crate::cli::ProbeCmd;
use sysinfo::System;

const TT_BYTES_PER_ENTRY: u64 = 16;

#[derive(Debug, Copy, Clone)]
pub struct MachineSpecs {
    pub total_memory_bytes: u64,
    pub cpu_cores_logical: usize,
    pub cpu_cores_physical: usize,
    #[cfg(target_os = "macos")]
    pub mac_perf_cores: Option<u32>,
    #[cfg(target_os = "macos")]
    pub mac_eff_cores: Option<u32>,
}

impl MachineSpecs {
    pub fn probe() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_memory_bytes = sys.total_memory();
        let cpu_cores_logical = sys.cpus().len();
        let cpu_cores_physical = sys.physical_core_count().unwrap_or(cpu_cores_logical);

        #[cfg(target_os = "macos")]
        let (mac_perf_cores, mac_eff_cores) = mac_perf_eff_cores();

        Self {
            total_memory_bytes,
            cpu_cores_logical,
            cpu_cores_physical,
            #[cfg(target_os = "macos")]
            mac_perf_cores,
            #[cfg(target_os = "macos")]
            mac_eff_cores,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BudgetPlan {
    pub esc_budget_bytes: u64,
    pub tt_bytes: u64,
    pub tb_bytes: u64,
    pub remainder_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct ProbeReport {
    pub specs: MachineSpecs,
    pub plan: BudgetPlan,
}

pub fn run(cmd: ProbeCmd) -> ProbeReport {
    let specs = MachineSpecs::probe();

    // Budget after OS reserve
    let esc = ((specs.total_memory_bytes as f64) * (1.0 - cmd.reserve_os))
        .max(0.0)
        .round() as u64;

    // TT sizing (power-of-two, rounded down in ENTRY COUNT, then scaled by entry size)
    let tt_target = cmd
        .tt_bytes
        .unwrap_or_else(|| (esc as f64 * cmd.tt_percent).floor() as u64);

    let tt_entries = tt_target / TT_BYTES_PER_ENTRY; // floor by integer division
    let (tt_entries_po2, _exp) = round_down_pow2_with_exponent(tt_entries);
    let tt_bytes_po2 = tt_entries_po2 * TT_BYTES_PER_ENTRY;

    // TB sizing (cap to remaining after TT)
    let remaining_after_tt = esc.saturating_sub(tt_bytes_po2);
    let tb_target = cmd.tb_bytes.unwrap_or(remaining_after_tt);
    let tb_final = tb_target.min(remaining_after_tt);

    let remainder = esc.saturating_sub(tt_bytes_po2 + tb_final);

    let plan = BudgetPlan {
        esc_budget_bytes: esc,
        tt_bytes: tt_bytes_po2,
        tb_bytes: tb_final,
        remainder_bytes: remainder,
    };

    ProbeReport { specs, plan }
}

pub fn print(report: &ProbeReport) {
    println!("Machine Specifications:");
    println!(
        "  Total RAM: {} ({:.2} GiB)",
        report.specs.total_memory_bytes,
        report.specs.total_memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    println!("  Logical cores: {}", report.specs.cpu_cores_logical);
    println!("  Physical cores: {}", report.specs.cpu_cores_physical);
    #[cfg(target_os = "macos")]
    {
        if let (Some(p), Some(e)) = (report.specs.mac_perf_cores, report.specs.mac_eff_cores) {
            println!("  macOS P-cores: {p}, E-cores: {e}");
        }
    }
    println!("Budget plan:");
    println!(
        "  Total: {} ({:.2} GiB)",
        report.plan.esc_budget_bytes,
        bytes_to_gib(report.plan.esc_budget_bytes)
    );
    println!(
        "  Transposition Table bytes: {} ({:.2} GiB)",
        report.plan.tt_bytes,
        bytes_to_gib(report.plan.tt_bytes)
    );
    println!(
        "  Tablebase bytes: {} ({:.2} GiB)",
        report.plan.tb_bytes,
        bytes_to_gib(report.plan.tb_bytes)
    );
    println!(
        "  Unallocated remainder: {} ({:.2} GiB)",
        report.plan.remainder_bytes,
        bytes_to_gib(report.plan.remainder_bytes)
    );
}

fn bytes_to_gib(b: u64) -> f64 {
    b as f64 / (1024.0 * 1024.0 * 1024.0)
}

fn round_down_pow2_with_exponent(x: u64) -> (u64, Option<u32>) {
    if x == 0 {
        (0, None)
    } else {
        let e = 63 - x.leading_zeros();
        (1u64 << e, Some(e))
    }
}

#[cfg(target_os = "macos")]
fn mac_perf_eff_cores() -> (Option<u32>, Option<u32>) {
    use libc::size_t;
    use std::mem::size_of_val;
    use std::ptr::null_mut;

    fn sysctl_u32(name: &str) -> Option<u32> {
        let cstr = std::ffi::CString::new(name).ok()?;
        let mut val: u32 = 0;
        let mut len: size_t = size_of_val(&val);
        let rc = unsafe {
            libc::sysctlbyname(
                cstr.as_ptr(),
                &mut val as *mut u32 as *mut _,
                &mut len as *mut size_t,
                null_mut(),
                0 as size_t,
            )
        };
        if rc == 0 && len as usize == size_of_val(&val) {
            Some(val)
        } else {
            None
        }
    }

    // Apple Silicon: perflevel0 ~ performance cluster, perflevel1 ~ efficiency cluster.
    let p = sysctl_u32("hw.perflevel0.physicalcpu");
    let e = sysctl_u32("hw.perflevel1.physicalcpu");
    (p, e)
}
