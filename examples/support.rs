extern crate hwloc;

use hwloc::Topology;

/// Example on how to check for specific topology support of a feature.
fn main() {
    let topo = Topology::new();

    // Check if CPU Binding is supported
    println!("CPU Binding supported: {}", topo.support().cpu().set_process());

    // Check if Memory Binding is supported
    println!("Memory Binding supported: {}", topo.support().memory().set_process());

    // Debug Print all the Support Flags
    println!("All Flags:\n{:?}", topo.support());
}
