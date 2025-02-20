export interface AdminStats {
    projects: number;
    versions: number;
    files: number;
    images: number;
    indexed_projects: number;
    users: number;
    uptime_secs: number;
    projects_size_bytes: number;
    gallery_size_bytes: number;
    sys_info: SysInfo;
}

/** System information */
export interface SysInfo {
    /** The total memory available to the system in bytes. */
    total_mem: number;

    /** The amount of used memory in bytes. */
    used_mem: number;

    /** The swap size in bytes. */
    total_swap: number;

    /** The amount of used swap in bytes. */
    used_swap: number;

    /** The system uptime in seconds. */
    uptime: number;

    /** The amount of free memory in bytes. */
    free_mem: number;

    /** The amount of free swap in bytes. */
    free_swap: number;

    /** The CPU architecture. */
    cpu_arch: string;

    /** The number of physical processor cores. */
    phys_core_count?: number;

    /** The distro/OS ID. */
    distro_id: string;

    /** The system name. */
    sys_name?: string;

    /** The system's kernel version. */
    kernel?: string;

    /** The OS version. */
    os_version?: string;

    /** The system hostname. */
    hostname?: string;

    /** The number of running processes. */
    processes: number;

    /** The server process ID. */
    pid: number;

    /** System CPU info. */
    cpus: CpuInfo[];

    /** System disk info. */
    disks: DiskInfo[];

    /** System network info. */
    networks: NetworkInfo[];
}

/** Information about a CPU/processor. */
export interface CpuInfo {
    /** The CPU's name. */
    name: string;

    /** The CPU's vendor ID. */
    vendor: string;

    /** The CPU brand. */
    brand: string;

    /** The CPU's frequency. */
    frequency: number;

    /** The CPU usage. */
    usage: number;
}

/** Disk information. */
export interface DiskInfo {
    /** The disk's name. */
    name: string;

    /** The kind of disk (SSD, HDD, etc.). */
    kind: string;

    /** The disk's file system. */
    file_system: string;

    /** The disk's mount point. */
    mount_point: string;

    /** The total size of the disk in bytes. */
    total_space: number;

    /** The available space on the disk in bytes. */
    available_space: number;

    /** Whether the disk is removable. */
    removable: boolean;

    /** Whether the disk is read-only. */
    read_only: boolean;
}

/** Network information. */
export interface NetworkInfo {
    /** The network name. */
    name: string;

    /** The total number of transmitted bytes. */
    up: number;

    /** The total number of received bytes. */
    down: number;
}
