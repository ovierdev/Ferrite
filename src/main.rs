use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=============================================");

    // Display system information
    println!("System name: {:?}", System::name().unwrap());
    println!(
        "System kernel version: {:?}",
        System::kernel_version().unwrap()
    );
    println!("System OS version: {:?}", System::os_version().unwrap());
    println!("System host name: {:?}", System::host_name().unwrap());

    println!("=============================================");
    // Display CPUs
    println!("NB CPUs: {}", sys.cpus().len());
    // Wait a bit because CPU usage is based on diff.

    sys.refresh_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()));
    // Refresh CPUs again to get actual value.
    println!("Used CPU: {:.2}%", sys.global_cpu_usage());
    println!("=============================================");
    // Display RAM memory\
    let memory = sys.total_memory() as f64 / 1048576.0;
    println!("Total memory:  {:.2} MiB", memory);
    let use_memory = sys.used_memory() as f64 / 1048576.0;
    println!("Used memory: {:.2}  MiB", use_memory);
    let total_swap = sys.total_swap() as f64 / 1048576.0;
    println!("Total swap: {:.2} MiB", total_swap);
    let used_swap = sys.used_swap() as f64 / 1048576.0;
    println!("Used swap {:.2} MiB", used_swap);
}
