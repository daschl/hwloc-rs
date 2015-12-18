# hwloc-rs
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/hwloc)](https://crates.io/crates/hwloc)

This project is a rust binding to the
[hwloc C library](http://www.open-mpi.org/projects/hwloc/), which provides a
portable abstraction of the hierarchical topology of modern architectures,
including NUMA memory nodes, sockets, shared caches, cores and simultaneous
multithreading.

## Prerequisites
Since this binding depends on the c library, you need to have it installed. The
easiest way is to install it system-wide.

Here is a table of the version compatibility that we try to test for:

| hwloc-rs | hwloc  |
|----------|--------|
| 0.1      | 1.11.1 |
| 0.2      | 1.11.1 |

### Install hwloc on OS X
The easiest way is to download, build and install the sources from the website.

 1. [Download](https://www.open-mpi.org/software/hwloc/v1.11/downloads/hwloc-1.11.2.tar.gz)
    the artifact.
 2. `tar -xvzpf hwloc-1.11.2.tar.gz`
 3. `cd hwloc-1.11.2`
 4. `./configure && make && sudo make install`

You can check if it works by trying the `lstopo` command. Here is the sample
output for a Mid 2012 MacBook Pro with 16GB of RAM:

```
~ $ lstopo
Machine (16GB total) + NUMANode L#0 (P#0 16GB) + L3 L#0 (6144KB)
  L2 L#0 (256KB) + L1d L#0 (32KB) + L1i L#0 (32KB) + Core L#0
    PU L#0 (P#0)
    PU L#1 (P#1)
  L2 L#1 (256KB) + L1d L#1 (32KB) + L1i L#1 (32KB) + Core L#1
    PU L#2 (P#2)
    PU L#3 (P#3)
  L2 L#2 (256KB) + L1d L#2 (32KB) + L1i L#2 (32KB) + Core L#2
    PU L#4 (P#4)
    PU L#5 (P#5)
  L2 L#3 (256KB) + L1d L#3 (32KB) + L1i L#3 (32KB) + Core L#3
    PU L#6 (P#6)
    PU L#7 (P#7)
```

### Install hwloc on Linux (Ubuntu 14.04)
On Ubuntu 14.04 installing it through `apt` is probably the easiest.

```
~ $ sudo apt-get install hwloc libhwloc-dev
```

You can again check with `lstopo`. The following is a 2-core virtual machine:

```
ubuntu-trusty-64:~$ lstopo
Machine (490MB)
  Socket L#0 + L2d L#0 (6144KB)
    L1d L#0 (32KB) + Core L#0 + PU L#0 (P#0)
    L1d L#1 (32KB) + Core L#1 + PU L#1 (P#1)
  HostBridge L#0
    PCI 80ee:beef
    PCI 8086:100e
      Net L#0 "eth0"
    PCI 8086:100e
      Net L#1 "eth1"
    PCI 8086:2829
      Block L#2 "sda"
```

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
hwloc = "0.1.0"
```

Next, add this to your crate root:

```rust
extern crate hwloc;
```

Here is a quick example which walks the `Topology` and prints it out:

```rust
extern crate hwloc;

use hwloc::Topology;

fn main() {
	let topo = Topology::new();

	for i in 0..topo.depth() {
		println!("*** Objects at level {}", i);

		for (idx, object) in topo.objects_at_depth(i).iter().enumerate() {
			println!("{}: {}", idx, object);
		}
	}
}
```

You can also [look at](https://github.com/daschl/hwloc-rs/tree/master/examples)
more examples, if you want to run them check out the next section below.

## Running Examples
The library ships with examples, and to run them you need to clone the repository
and then run them through `cargo run --example=`.

```
$ git clone https://github.com/daschl/hwloc-rs.git
$ cd hwloc-rs
```

To run an example (which will download the dependencies and build it) you can
use `cargo run -example=`:

```
$ cargo run --example=walk_tree
   Compiling libc v0.2.3
   ...
   Compiling hwloc v0.2.0 (file:///vagrant/hwloc-rs)
     Running `target/debug/examples/walk_tree`
*** Printing overall tree
Machine (490MB): #0
 Socket (): #0
  L2d (6144KB): #4294967295
   L1d (32KB): #4294967295
    Core (): #0
     PU (): #0
   L1d (32KB): #4294967295
    Core (): #1
     PU (): #1
```

## License
This project uses the MIT license, please see the
[LICENSE](https://github.com/daschl/hwloc-rs/blob/master/LICENSE) file for more
information.
