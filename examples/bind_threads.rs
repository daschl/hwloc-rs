extern crate hwloc;
extern crate libc;

use hwloc::{Topology, ObjectType, CPUBIND_THREAD, CpuSet};
use std::thread;
use std::sync::Arc;

/// Example which spawns one thread per core and then assigns it to each.
fn main() {
    let topo = Arc::new(Topology::new());

    let cores = topo.objects_with_type(&ObjectType::Core).unwrap();
    let num_cores = cores.len();
    println!("Found {} cores.", num_cores);

    let handles: Vec<_> = (0..num_cores).map(|i| {
            let child_topo = topo.clone();
            thread::spawn(move || {
                let tid = get_thread_id();
                let local_topo = &child_topo;

                println!("Before T{}: {:?}", i, local_topo.get_cpubinding_for_thread(tid, CPUBIND_THREAD));

                let mut bind_to = CpuSet::new();
                bind_to.set((i as u32));

                local_topo.set_cpubinding_for_thread(tid, bind_to, CPUBIND_THREAD).unwrap();

                println!("After T{}: {:?}", i, local_topo.get_cpubinding_for_thread(tid, CPUBIND_THREAD));
            })
        }).collect();

        for h in handles {
            h.join().unwrap();
        }
}

fn get_thread_id() -> u64 {
    unsafe { libc::pthread_self() }
}
