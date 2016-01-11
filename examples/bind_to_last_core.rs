extern crate hwloc;

use hwloc::{Topology, TopologyObject, ObjectType, CPUBIND_PROCESS};

/// Bind to only one thread of the last core of the machine.
///
/// First find out where cores are, or else smaller sets of CPUs if
/// the OS doesn't have the notion of a "core".
///
/// Example Output with 2 cores (no HT) on linux:
///
/// ```
/// Cpu Binding before explicit bind: Some(0-1)
/// Cpu Location before explicit bind: Some(0)
/// Correctly bound to last core
/// Cpu Binding after explicit bind: Some(1)
/// Cpu Location after explicit bind: Some(1)
/// ```
fn main() {
    let mut topo = Topology::new();

    // Grab last core and exctract its CpuSet
    let mut cpuset = last_core(&mut topo).cpuset().unwrap();

    //  Get only one logical processor (in case the core is SMT/hyper-threaded).
    cpuset.singlify();

    // Print the current cpu binding before explicit setting
    println!("Cpu Binding before explicit bind: {:?}", topo.get_cpubind(CPUBIND_PROCESS));
    println!("Cpu Location before explicit bind: {:?}", topo.get_cpu_location(CPUBIND_PROCESS));

    // Try to bind all threads of the current (possibly multithreaded) process.
    match topo.set_cpubind(cpuset, CPUBIND_PROCESS) {
        Ok(_) => println!("Correctly bound to last core"),
        Err(e) => println!("Failed to bind: {:?}", e)
    }

    // Print the current cpu binding after explicit setting
    println!("Cpu Binding after explicit bind: {:?}", topo.get_cpubind(CPUBIND_PROCESS));
    println!("Cpu Location after explicit bind: {:?}", topo.get_cpu_location(CPUBIND_PROCESS));
}

/// Find the last core
fn last_core(topo: &mut Topology) -> &TopologyObject {
    let core_depth = topo.depth_or_below_for_type(&ObjectType::Core).unwrap();
    let all_cores = topo.objects_at_depth(core_depth);
    all_cores.last().unwrap()
}
