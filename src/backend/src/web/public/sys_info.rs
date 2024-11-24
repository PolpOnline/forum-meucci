use axum::{response::IntoResponse, Json};
use serde::Serialize;
use sysinfo::System;
use tokio::time::sleep;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct MemInfo {
    total: String,
    free: String,
    used: String,
}

#[derive(Serialize, ToSchema)]
pub struct SwapInfo {
    total: String,
    free: String,
    used: String,
}

#[derive(Serialize, ToSchema)]
pub struct CpuInfo {
    usage: f32,
    name: String,
    vendor_id: String,
    brand: String,
    frequency: String,
}

#[derive(Serialize, ToSchema)]
pub struct BasicSystemInfo {
    system_name: String,
    system_kernel_version: String,
    system_os_version: String,
    system_host_name: String,
}

#[derive(Serialize, ToSchema)]
pub struct SystemInfoResponse {
    cpu_info: CpuInfo,
    memory: MemInfo,
    swap: SwapInfo,
    basic: BasicSystemInfo,
}

#[utoipa::path(
    get,
    path = "/sys_info",
    responses(
            (status = 200, description = "System info", body = SystemInfoResponse),
    )
)]
pub async fn sys_info() -> impl IntoResponse {
    let mut s = System::new_all();

    // Wait a bit because CPU usage is based on diff.
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    // Refresh CPUs again.
    s.refresh_cpu_all();

    let res = SystemInfoResponse {
        cpu_info: get_cpu_info(&s),
        memory: get_mem_info(&s),
        swap: get_swap_info(&s),
        basic: get_basic_info(),
    };

    Json(res)
}

fn get_cpu_info(s: &System) -> CpuInfo {
    let cpu = s.cpus().first().unwrap();

    CpuInfo {
        usage: cpu.cpu_usage(),
        name: cpu.name().to_string(),
        vendor_id: cpu.vendor_id().to_string(),
        brand: cpu.brand().to_string(),
        frequency: fmt_frequency(cpu.frequency()),
    }
}

fn get_mem_info(s: &System) -> MemInfo {
    MemInfo {
        total: fmt_bytes(s.total_memory()),
        free: fmt_bytes(s.free_memory()),
        used: fmt_bytes(s.used_memory()),
    }
}

fn get_swap_info(s: &System) -> SwapInfo {
    SwapInfo {
        total: fmt_bytes(s.total_swap()),
        free: fmt_bytes(s.free_swap()),
        used: fmt_bytes(s.used_swap()),
    }
}

fn get_basic_info() -> BasicSystemInfo {
    BasicSystemInfo {
        system_name: System::name().unwrap_or("Unknown".to_string()),
        system_kernel_version: System::kernel_version().unwrap_or("Unknown".to_string()),
        system_os_version: System::os_version().unwrap_or("Unknown".to_string()),
        system_host_name: System::host_name().unwrap_or("Unknown".to_string()),
    }
}

fn fmt_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / 1024.0 / 1024.0 / 1024.0;

    format!("{:.3} GiB", gb)
}

fn fmt_frequency(mhz: u64) -> String {
    let ghz = mhz as f64 / 1000.0;

    format!("{:.3} GHz", ghz)
}
