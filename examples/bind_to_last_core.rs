extern crate hwloc;

use hwloc::{Topology, ObjectType, CPUBIND_PROCESS};

/// Bind to only one thread of the last core of the machine.
///
/// First find out where cores are, or else smaller sets of CPUs if
/// the OS doesn't have the notion of a "core".
fn main() {
    let topo = Topology::new();

    // Find the last core
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    let last_core = all_cores.last().unwrap();

    // Grab its CpuSet
    let mut cpuset = last_core.cpuset().unwrap();

    //  Get only one logical processor (in case the core is SMT/hyper-threaded).
    cpuset.singlify();

    // Try to bind all threads of the current (possibly multithreaded) process.
    match topo.set_cpubinding(cpuset, CPUBIND_PROCESS) {
        Ok(_) => println!("Correctly bound to last core"),
        Err(e) => println!("Failed to bind: {:?}", e)
    }
}
