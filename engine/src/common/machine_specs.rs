use sysinfo::System;

#[derive(Debug, Copy, Clone)]
pub struct MachineSpecs {
    pub total_memory_bytes: u64,
    pub cpu_cores_logical: usize,
    pub cpu_cores_physical: usize,
    #[cfg(target_os = "macos")]
    pub mac_perf_cores: Option<u32>,
}

impl MachineSpecs {
    pub fn probe() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_memory_bytes = sys.total_memory();
        let cpu_cores_logical = sys.cpus().len();
        let cpu_cores_physical = sys.physical_core_count().unwrap_or(cpu_cores_logical);

        #[cfg(target_os = "macos")]
        let (mac_perf_cores, _mac_eff_cores) = mac_perf_eff_cores();

        Self {
            total_memory_bytes,
            cpu_cores_logical,
            cpu_cores_physical,
            #[cfg(target_os = "macos")]
            mac_perf_cores,
        }
    }

    /// Get available memory after reserving percentage for OS
    pub fn available_memory(&self, reserve_os: f64) -> u64 {
        ((self.total_memory_bytes as f64) * (1.0 - reserve_os))
            .max(0.0)
            .round() as u64
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
