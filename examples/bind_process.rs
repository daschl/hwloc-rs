extern crate hwloc;
extern crate libc;
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;

use hwloc::{Topology, CPUBIND_PROCESS, TopologyObject, ObjectType};

/// Example which binds an arbitrary process (in this example this very same one) to
/// the last core.
fn main() {
    let mut topo = Topology::new();

    // load the current pid through libc
    let pid = get_pid();

    println!("Binding Process with PID {:?}", pid);

    // Grab last core and exctract its CpuSet
    let mut cpuset = last_core(&mut topo).cpuset().unwrap();

    // Get only one logical processor (in case the core is SMT/hyper-threaded).
    cpuset.singlify();

    println!("Before Bind: {:?}",
             topo.get_cpubind_for_process(pid, CPUBIND_PROCESS)
                 .unwrap());

    // Last CPU Location for this PID (not implemented on all systems)
    if let Some(l) = topo.get_cpu_location_for_process(pid, CPUBIND_PROCESS) {
        println!("Last Known CPU Location: {:?}", l);
    }

    // Bind to one core.
    topo.set_cpubind_for_process(pid, cpuset, CPUBIND_PROCESS)
        .unwrap();

    println!("After Bind: {:?}",
             topo.get_cpubind_for_process(pid, CPUBIND_PROCESS)
                 .unwrap());

    // Last CPU Location for this PID (not implemented on all systems)
    if let Some(l) = topo.get_cpu_location_for_process(pid, CPUBIND_PROCESS) {
        println!("Last Known CPU Location: {:?}", l);
    }
}

/// Find the last core
fn last_core(topo: &mut Topology) -> &TopologyObject {
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    all_cores.last().unwrap()
}

#[cfg(target_os = "windows")]
fn get_pid() -> winapi::minwindef::DWORD {
    unsafe { kernel32::GetCurrentProcessId() }
}

#[cfg(not(target_os = "windows"))]
fn get_pid() -> libc::pid_t {
    unsafe { libc::getpid() }
}
