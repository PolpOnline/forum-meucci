use axum::{response::IntoResponse, Json};
use serde::Serialize;
use sysinfo::System;
use tokio::time::sleep;
use utoipa::ToSchema;

use crate::app::openapi::SYSTEM_TAG;

#[derive(Serialize, ToSchema)]
pub struct MemInfo {
    total: String,
    free: String,
    used: String,
}

impl MemInfo {
    pub fn new(s: &System) -> Self {
        Self {
            total: fmt_bytes(s.total_memory()),
            free: fmt_bytes(s.free_memory()),
            used: fmt_bytes(s.used_memory()),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct SwapInfo {
    total: String,
    free: String,
    used: String,
}

impl SwapInfo {
    pub fn new(s: &System) -> Self {
        Self {
            total: fmt_bytes(s.total_swap()),
            free: fmt_bytes(s.free_swap()),
            used: fmt_bytes(s.used_swap()),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct CpuInfo {
    usage: f32,
    name: String,
    vendor_id: String,
    brand: String,
    frequency: String,
}

impl CpuInfo {
    pub fn new(s: &System) -> Self {
        let cpu = s.cpus().first().unwrap();

        Self {
            usage: cpu.cpu_usage(),
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: fmt_frequency(cpu.frequency()),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct BasicSystemInfo {
    system_name: String,
    system_kernel_version: String,
    system_os_version: String,
    system_host_name: String,
}

impl BasicSystemInfo {
    pub fn new() -> Self {
        Self {
            system_name: System::name().unwrap_or("Unknown".to_string()),
            system_kernel_version: System::kernel_version().unwrap_or("Unknown".to_string()),
            system_os_version: System::os_version().unwrap_or("Unknown".to_string()),
            system_host_name: System::host_name().unwrap_or("Unknown".to_string()),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct SystemInfoResponse {
    cpu_info: CpuInfo,
    memory: MemInfo,
    swap: SwapInfo,
    basic: BasicSystemInfo,
}

impl SystemInfoResponse {
    pub fn new(s: &System) -> Self {
        Self {
            cpu_info: CpuInfo::new(s),
            memory: MemInfo::new(s),
            swap: SwapInfo::new(s),
            basic: BasicSystemInfo::new(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/sys_info",
    summary = "System Info",
    responses(
            (status = 200, description = "System info", body = SystemInfoResponse),
    ),
    tag = SYSTEM_TAG
)]
pub async fn sys_info() -> impl IntoResponse {
    let mut s = System::new_all();

    // Wait a bit because CPU usage is based on diff.
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    // Refresh CPUs again.
    s.refresh_cpu_all();

    Json(SystemInfoResponse::new(&s))
}

fn fmt_bytes(bytes: u64) -> String {
    let gb = bytes as f64 / 1024.0 / 1024.0 / 1024.0;

    format!("{:.3} GiB", gb)
}

fn fmt_frequency(mhz: u64) -> String {
    let ghz = mhz as f64 / 1000.0;

    format!("{:.3} GHz", ghz)
}
