extern crate hwloc;
extern crate libc;

use hwloc::{Topology, CPUBIND_PROCESS, TopologyObject, ObjectType};

/// Example which binds an arbitrary process (in this example this very same one) to
/// the last core.
fn main() {
    let mut topo = Topology::new();

    // load the current pid through libc
    let pid = unsafe { libc::getpid() };

    println!("Binding Process with PID {:?}", pid);

    // Grab last core and exctract its CpuSet
    let mut cpuset = last_core(&mut topo).cpuset().unwrap();

    // Get only one logical processor (in case the core is SMT/hyper-threaded).
    cpuset.singlify();

    println!("Before Bind: {:?}", topo.get_cpubinding_for_pid(pid, CPUBIND_PROCESS).unwrap());

    // Last CPU Location for this PID
    println!(
        "Last Known CPU Location: {:?}",
        topo.get_last_cpu_location_for_pid(pid, CPUBIND_PROCESS).unwrap()
    );

    // Bind to one core.
    topo.set_cpubinding_for_pid(pid, cpuset, CPUBIND_PROCESS).unwrap();

    println!("After Bind: {:?}", topo.get_cpubinding_for_pid(pid, CPUBIND_PROCESS).unwrap());

    // Last CPU Location for this PID
    println!(
        "Last Known CPU Location: {:?}",
        topo.get_last_cpu_location_for_pid(pid, CPUBIND_PROCESS).unwrap()
    );
}

/// Find the last core
fn last_core(topo: &mut Topology) -> &TopologyObject {
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    all_cores.last().unwrap()
}
