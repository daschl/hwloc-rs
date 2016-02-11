extern crate hwloc;

use hwloc::Topology;

/// Example on how to check for specific topology support of a feature.
fn main() {
    let topo = Topology::new();

    // Check if Process Binding for CPUs is supported
    println!("CPU Binding (current process) supported: {}", topo.support().cpu().set_current_process());
    println!("CPU Binding (any process) supported: {}", topo.support().cpu().set_process());

    // Check if Thread Binding for CPUs is supported
    println!("CPU Binding (current thread) supported: {}", topo.support().cpu().set_current_thread());
    println!("CPU Binding (any thread) supported: {}", topo.support().cpu().set_thread());

    // Check if Memory Binding is supported
    println!("Memory Binding supported: {}", topo.support().memory().set_current_process());

    // Debug Print all the Support Flags
    println!("All Flags:\n{:?}", topo.support());
}
