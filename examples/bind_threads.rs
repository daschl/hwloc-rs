extern crate hwloc;
extern crate libc;

use hwloc::{Topology, ObjectType, CPUBIND_THREAD, CpuSet};
use std::thread;
use std::sync::{Arc,Mutex};

/// Example which spawns one thread per core and then assigns it to each.
///
/// Example Output with 2 cores (no HT) on linux:
///
/// ```
/// Found 2 cores.
/// Thread 0: Before Some(0-1), After Some(0)
/// Thread 1: Before Some(0-1), After Some(1)
/// ```
fn main() {
    let topo = Arc::new(Mutex::new(Topology::new()));

    // Grab the number of cores in a block so that the lock is removed once
    // the end of the block is reached.
    let num_cores = {
        let topo_rc = topo.clone();
        let topo_locked = topo_rc.lock().unwrap();
        (*topo_locked).objects_with_type(&ObjectType::Core).unwrap().len()
    };
    println!("Found {} cores.", num_cores);

    // Spawn one thread for each and pass the topology down into scope.
    let handles: Vec<_> = (0..num_cores).map(|i| {
            let child_topo = topo.clone();
            thread::spawn(move || {
                // Get the current thread id and lock the topology to use.
                let tid = get_thread_id();
                let mut locked_topo = child_topo.lock().unwrap();

                // Thread binding before explicit set.
                let before = locked_topo.get_cpubinding_for_thread(tid, CPUBIND_THREAD);

                // Create a CPU binding for the current thread.
                let mut bind_to = CpuSet::new();
                bind_to.set((i as u32));

                // Set the binding.
                locked_topo.set_cpubinding_for_thread(tid, bind_to, CPUBIND_THREAD).unwrap();

                // Thread binding after explicit set.
                let after = locked_topo.get_cpubinding_for_thread(tid, CPUBIND_THREAD);
                println!("Thread {}: Before {:?}, After {:?}", i, before, after);
            })
        }).collect();

        // Wait for all threads to complete before ending the program.
        for h in handles {
            h.join().unwrap();
        }
}

/// Helper method to get the thread id through libc, with current rust stable (1.5.0) its not
/// possible otherwise I think.
fn get_thread_id() -> libc::pthread_t {
    unsafe { libc::pthread_self() }
}
