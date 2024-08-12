use std::{ vec, error::Error };
use sysinfo::{
    System,
    CpuExt,
    DiskExt,
    SystemExt,
    ProcessExt,
};
use serde::{Deserialize, Serialize};

/// @Description 系统信息的响应模型，当收到下发的此任务，需要解析数据时使用该模型
///
/// @Param kind 需要获取的信息类型
/// 0 系统基础信息
/// 1 ram 和 swap 信息
/// 2 disk 信息
/// 3 cpu 信息
/// 4 network 信息
/// 5 process 信息
/// 6 全部信息
#[derive(Deserialize)]
pub struct SystemInfoResponse {
    pub kind: Option<usize>
}

/// @Description 系统信息
///
/// @Param system 信息
/// @Param ram_swap 信息
/// @Param disk 信息
/// @Param cpu 信息
/// @Param network 信息
/// @Param process 信息
#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub system_basic: SystemBasic,
    pub ram_swap: RamAndSwap,
    pub disk: Disk,
    pub cpu: Vec<CPU>,
    pub network: Vec<Network>,
    pub process: Vec<Process>
}

/// @Description 系统基本信息
///
/// @Param pub name 系统名称
/// @Param pub host_name 基于 DNS 的系统主机名
/// @Param pub kernel_version 系统的内核版本
/// @Param pub os_version 系统版本（对于 MacOS，这将返回 13.0.1 而不是内核版本）
/// @Param pub long_os_version 系统长操作系统版本（如“MacOS 13.0.1”）
/// @Param pub boot_at_seconds 系统从 UNIX 纪元开始启动的时间（以秒为单位）
/// @Param pub running_at_seconds 系统正常运行时间（以秒为单位）
#[derive(Debug, Serialize)]
pub struct SystemBasic {
    pub name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub long_os_version: String,
    pub boot_at_seconds: u64,
    pub running_at_seconds: u64,
}

/// @Description ram 和 swap 信息
///
/// @Param total_memory 以字节为单位返回 RAM 大小
/// @Param free_memory 返回空闲 RAM 的字节数
/// @Param available_memory 返回可用 RAM 的字节数
/// @Param used_memory 以字节为单位返回已用 RAM 的数量
/// @Param total_swap 以字节为单位返回 SWAP 大小
/// @Param free_swap 以字节为单位返回空闲 SWAP 的数量
/// @Param used_swap 以字节为单位返回已使用的 SWAP 数量
#[derive(Debug, Serialize)]
pub struct RamAndSwap {
    pub total_memory: u64,
    pub free_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
}

/// @Description 系统信息
///
/// @Param storage_type 磁盘类型
/// @Param name 磁盘名称
/// @Param file_system 此磁盘上使用的文件系统（例如: EXT4、NTFS等)
/// @Param mount_point 返回磁盘的安装点（例如: /)
/// @Param total_space 总磁盘大小，以字节为单位
/// @Param available_space 可用磁盘大小，以字节为单位
/// @Param is_removable 磁盘是否可移动
#[derive(Debug, Serialize)]
pub struct Disk {
    pub storage_type: String,
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total_space: String,
    pub available_space: String,
    pub is_removable: String,
}

/// @Description CPU 信息
///
/// @Param CPU 的名称
/// @Param CPU 的供应商 ID
/// @Param CPU 的品牌
/// @Param CPU 的频率
#[derive(Debug, Serialize)]
pub struct CPU {
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: String,
}

/// @Description Network 信息
///
/// @Param interface_name 接口名称
#[derive(Debug, Serialize)]
pub struct Network {
    pub interface_name: String,
}

/// @Description Process 信息
///
/// @Param parent_pid 进程的父 pid
/// @Param pid 进程的 pid
/// @Param user_id 进程的所有者用户的 ID
/// @Param group_id 进程的进程组 ID
/// @Param name 进程的名称
/// @Param status 进程的状态
/// @Param session_id 当前进程的会话 ID
/// @Param running_seconds 进程运行的时间（以秒为单位）
/// @Param execute_path 进程的路径
/// @Param command 命令行
/// @Param environ 进程的环境变量
/// @Param cwd_path 当前工作目录
/// @Param root_path 根目录的路径
/// @Param memory 内存使用情况（以字节为单位）
/// @Param virtual_memory 虚拟内存使用情况（以字节为单位）
/// @Param disk_usage_read_bytes 读取磁盘的字节数
/// @Param disk_usage_total_read_bytes 总共读取磁盘的字节数
#[derive(Debug, Serialize)]
pub struct Process {
    pub parent_pid: String,
    pub pid: String,
    pub user_id: String,
    pub group_id: String,
    pub name: String,
    pub status: String,
    pub session_id: String,
    pub running_seconds: u64,
    pub execute_path: String,
    pub command: String,
    pub environ: String,
    pub cwd_path: String,
    pub root_path: String,
    pub memory: u64,
    pub virtual_memory: u64,
    pub disk_usage_read_bytes: u64,
    pub disk_usage_total_read_bytes: u64,
}


impl SystemInfo {
    /// @Description 获取系统信息
    ///
    /// @return 返回获取的数据以及捕获可能的异常
    pub fn run(kind: usize) -> Result<String, Box<dyn Error>> {
        // 解析 json 数据并获取数据
        // let kind: usize = match serde_json::from_str(&task.data)? {
        //     Some(data @ SystemInfoResponse { .. } ) => {
        //         match data.kind {
        //             Some(data) => data,
        //             None => return Err("Data is null".into()),
        //         }
        //     },
        //     None => return Err("Data is null.".into()),
        // };

        // 获取系统信息
        let mut sys: System = System::new_all();

        // 刷新一下
        sys.refresh_all();

        // 匹配要获取的数据，然后序列化为 json 字符串
        let result = match kind {
            0 => serde_json::to_string(&Self::get_system(&sys)?)?,
            1 => serde_json::to_string(&Self::get_ram_swap(&sys))?,
            2 => serde_json::to_string(&Self::get_disk(&sys))?,
            3 => serde_json::to_string(&Self::get_cpu(&sys))?,
            4 => serde_json::to_string(&Self::get_network(&sys))?,
            5 => serde_json::to_string(&Self::get_process(&sys))?,
            6 => serde_json::to_string(&Self {
                system_basic: Self::get_system(&sys)?,
                ram_swap: Self::get_ram_swap(&sys),
                disk: Self::get_disk(&sys),
                cpu: Self::get_cpu(&sys),
                network: Self::get_network(&sys),
                process: Self::get_process(&sys),
            })?,
            _ => String::new(),
        };

        // 获取相关信息
        Ok(result)
    }

    
    /// @Description 获取磁盘信息
    ///
    /// @Param sys 系统信息
    /// @return 返回获取的数据以及捕获可能的异常
    fn get_disk(sys: &System) -> Disk {
        let mut disk_info: Disk = Disk{
            storage_type: String::new(),
            name: String::new(),
            file_system: String::new(),
            mount_point: String::new(),
            total_space: String::new(),
            available_space: String::new(),
            is_removable: String::new(),
        };

        for disk in sys.disks() {
            disk_info = Disk {
                storage_type: format!("{:?}", disk.type_()),            // 磁盘类型
                name: format!("{:?}", disk.name()),                     // 磁盘名称
                file_system: format!("{:?}", disk.file_system()),       // 此磁盘上使用的文件系统（例如: EXT4、NTFS等)
                mount_point: format!("{:?}", disk.mount_point()),       // 返回磁盘的安装点（例如: /)
                total_space: format!("{}", disk.total_space() / 1000 / 1000),           // 总磁盘大小，以字节为单位
                available_space: format!("{}", disk.available_space() / 1000 / 1000),   // 可用磁盘大小，以字节为单位
                is_removable: format!("{}", disk.is_removable()),       // 磁盘是否可移动
            };
        }

        disk_info
    }
    
    /// @Description 获取 cpu 信息
    ///
    /// @Param sys 系统信息
    /// @return 返回获取的数据以及捕获可能的异常
    fn get_cpu(sys: &System) -> Vec<CPU> {
        let mut cpu_info: Vec<CPU> = Vec::new();

        for cpu in sys.cpus() {
            cpu_info.push(CPU{
                name: format!("{}", cpu.name()),            // CPU 的名称
                vendor_id: format!("{}", cpu.vendor_id()),  // CPU 的供应商 ID
                brand: format!("{}", cpu.brand()),          // CPU 的品牌
                frequency: format!("{}", cpu.frequency()),  // CPU 的频率
            });
        }

        cpu_info
    }

    
    /// @Description 获取网络信息
    ///
    /// @Param sys 系统信息
    /// @return 返回获取的数据以及捕获可能的异常
    fn get_network(sys: &System) -> Vec<Network> {
        let mut network_info: Vec<Network> = vec![];

        for (interface_name, _data) in sys.networks() {
            network_info.push(Network{
                interface_name: interface_name.to_string(), // 接口名称
            });
        }

        network_info
    }

    
    /// @Description 获取 ram 和 swap 信息
    ///
    /// @Param sys 系统信息
    /// @return 返回获取的数据以及捕获可能的异常
    fn get_ram_swap(sys: &System) -> RamAndSwap {
        RamAndSwap{
            total_memory: sys.total_memory() / 1048576,          // 以字节为单位返回 RAM 大小
            free_memory: sys.free_memory() / 1048576,            // 返回空闲 RAM 的字节数
            available_memory: sys.available_memory() / 1048576,  // 返回可用 RAM 的字节数
            used_memory: sys.used_memory() / 1048576,            // 以字节为单位返回已用 RAM 的数量
            total_swap: sys.total_swap() / 1048576,              // 以字节为单位返回 SWAP 大小
            free_swap: sys.free_swap() / 1048576,                // 以字节为单位返回空闲 SWAP 的数量
            used_swap: sys.used_swap() / 1048576,                // 以字节为单位返回已使用的 SWAP 数量
        }
    }

    
    /// @Description 获取系统基本信息
    ///
    /// @Param sys 系统信息
    /// @return 返回获取的数据以及捕获可能的异常
    fn get_system(sys: &System) -> Result<SystemBasic, Box<dyn Error>> {
        Ok(SystemBasic {
            name: sys.name().unwrap_or_else(|| "name is null".to_string()),
            host_name: sys.host_name().unwrap(),                // 基于 DNS 的系统主机名
            kernel_version: sys.kernel_version().unwrap(),      // 系统的内核版本
            os_version: sys.os_version().unwrap(),              // 系统版本（对于 MacOS，这将返回 13.0.1 而不是内核版本）
            long_os_version: sys.long_os_version().unwrap(),    // 系统长操作系统版本（如“MacOS 13.0.1”）
            boot_at_seconds: sys.boot_time() / 3600,            // 系统从 UNIX 纪元开始启动的时间（以秒为单位）
            running_at_seconds: sys.uptime() / 3600,            // 系统正常运行时间（以秒为单位）
        })
    }

    
    /// @Description 获取进程信息
    ///
    /// @Param sys 系统信息
    /// @return 返回获取的数据以及捕获可能的异常
    fn get_process(sys: &System) -> Vec<Process> {
        let mut process_info = vec![];

        for (pid, process) in sys.processes() {
            let disk_usage = process.disk_usage();

            process_info.push(Process{
                parent_pid: format!("{:?}", process.parent()),              // 进程的父 pid
                pid: pid.to_string(),                                       // 进程的 pid
                user_id: format!("{:?}", process.user_id()),                // 进程的所有者用户的 ID
                group_id: format!("{:?}", process.group_id()),              // 进程的进程组 ID
                name: format!("{}", process.name()),                        // 进程的名称
                status: format!("{:?}", process.status()),                  // 进程的状态
                session_id: format!("{:?}", process.session_id()),          // 当前进程的会话 ID
                running_seconds: process.run_time(),                        // 进程运行的时间（以秒为单位）
                execute_path: format!("{:?}", process.exe().display()),     // 进程的路径
                command: format!("{:?}", process.cmd()),                    // 命令行
                environ: format!("{:?}", process.environ()),                // 进程的环境变量
                cwd_path: format!("{}", process.cwd().display()),           // 当前工作目录
                root_path: format!("{}", process.root().display()),         // 根目录的路径
                memory: process.memory(),                                   // 内存使用情况（以字节为单位）
                virtual_memory: process.virtual_memory(),                   // 虚拟内存使用情况（以字节为单位）
                disk_usage_read_bytes: disk_usage.read_bytes,               // 读取磁盘的字节数
                disk_usage_total_read_bytes: disk_usage.total_read_bytes,   // 总共读取磁盘的字节数
            });
        }

        process_info
    }
}

#[test]
fn test() {
    let result = SystemInfo::run(3).unwrap();
    println!("+[SystemInfo] System Info: {:?}", result);
}