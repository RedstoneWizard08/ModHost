//! System information utilities.

use serde::{Deserialize, Serialize};
use std::process;
use sysinfo::{Disks, Networks, System};
use utoipa::{ToResponse, ToSchema};

/// System metrics and statistics.
#[derive(
    Debug, Clone, PartialEq, PartialOrd, Default, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct SysInfo {
    /// The total memory available to the system in bytes.
    pub total_mem: u64,

    /// The amount of used memory in bytes.
    pub used_mem: u64,

    /// The swap size in bytes.
    pub total_swap: u64,

    /// The amount of used swap in bytes.
    pub used_swap: u64,

    /// The system uptime in seconds.
    pub uptime: u64,

    /// The amount of free memory in bytes.
    pub free_mem: u64,

    /// The amount of free swap in bytes.
    pub free_swap: u64,

    /// The CPU architecture.
    pub cpu_arch: String,

    /// The number of physical processor cores.
    pub phys_core_count: Option<usize>,

    /// The distro/OS ID.
    pub distro_id: String,

    /// The system name.
    pub sys_name: Option<String>,

    /// The system's kernel version.
    pub kernel: Option<String>,

    /// The OS version.
    pub os_version: Option<String>,

    /// The system hostname.
    pub hostname: Option<String>,

    /// The number of running processes.
    pub processes: usize,

    /// The server process ID.
    pub pid: u32,

    /// System CPU info.
    pub cpus: Vec<CpuInfo>,

    /// System disk info.
    pub disks: Vec<DiskInfo>,

    /// System network info.
    pub networks: Vec<NetworkInfo>,
}

/// Information about a CPU/processor.
#[derive(
    Debug, Clone, PartialEq, PartialOrd, Default, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct CpuInfo {
    /// The CPU's name.
    pub name: String,

    /// The CPU's vendor ID.
    pub vendor: String,

    /// The CPU brand.
    pub brand: String,

    /// The CPU's frequency.
    pub frequency: u64,

    /// The CPU usage.
    pub usage: f32,
}

/// Disk information.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
)]
pub struct DiskInfo {
    /// The disk's name.
    pub name: String,

    /// The kind of disk (SSD, HDD, etc.).
    pub kind: String,

    /// The disk's file system.
    pub file_system: String,

    /// The disk's mount point.
    pub mount_point: String,

    /// The total size of the disk in bytes.
    pub total_space: u64,

    /// The available space on the disk in bytes.
    pub available_space: u64,

    /// Whether the disk is removable.
    pub removable: bool,

    /// Whether the disk is read-only.
    pub read_only: bool,
}

/// Network information.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
)]
pub struct NetworkInfo {
    /// The network name.
    pub name: String,

    /// The total number of transmitted bytes.
    pub up: u64,

    /// The total number of received bytes.
    pub down: u64,
}

/// Get system info.
pub fn get_sys_info() -> SysInfo {
    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();

    sys.refresh_all();

    SysInfo {
        total_mem: sys.total_memory(),
        used_mem: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        uptime: System::uptime(),
        free_mem: sys.free_memory(),
        free_swap: sys.free_swap(),
        cpu_arch: System::cpu_arch(),
        phys_core_count: sys.physical_core_count(),
        distro_id: System::distribution_id(),
        sys_name: System::name(),
        kernel: System::kernel_version(),
        os_version: System::os_version(),
        hostname: System::host_name(),
        processes: sys.processes().len(),
        pid: process::id(),
        cpus: sys
            .cpus()
            .into_iter()
            .map(|cpu| CpuInfo {
                name: cpu.name().into(),
                vendor: cpu.vendor_id().into(),
                brand: cpu.brand().into(),
                frequency: cpu.frequency(),
                usage: cpu.cpu_usage(),
            })
            .collect(),
        disks: disks
            .into_iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().into(),
                kind: disk.kind().to_string(),
                file_system: disk.file_system().to_string_lossy().into(),
                mount_point: disk.mount_point().to_string_lossy().into(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                removable: disk.is_removable(),
                read_only: disk.is_read_only(),
            })
            .collect(),
        networks: networks
            .into_iter()
            .map(|(name, net)| NetworkInfo {
                name: name.into(),
                up: net.total_transmitted(),
                down: net.total_received(),
            })
            .collect(),
    }
}
