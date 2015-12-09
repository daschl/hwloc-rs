extern crate hwloc;

use hwloc::{Topology, TopologyObject};

/// Walk the topology in a tree-style and print it.
fn main() {
	let topo = Topology::new();

	println!("*** Printing overall tree");
	print_children(&topo, topo.object_at_root(), 0);
}

fn print_children(topo: &Topology, obj: &TopologyObject, depth: usize) {
	let padding = std::iter::repeat(" ").take(depth).collect::<String>();
	println!("{}{}: #{}", padding, obj, obj.os_index());

	for i in 0..obj.arity() {
		print_children(topo, obj.children()[i as usize], depth + 1);
	}
}