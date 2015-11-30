#![allow(dead_code)]
extern crate libc;
extern crate num;

mod ffi;
mod topology_object;

pub use ffi::{ObjectType, TypeDepthError, TopologyFlag, CpuSet};
use num::{ToPrimitive, FromPrimitive};

pub use topology_object::{TopologyObject, TopologyObjectMemory};

pub struct Topology {
	topo: *mut ffi::HwlocTopology
}

impl Topology {

	/// Creates a new Topology.
	///
	/// If no further customization is needed on init, this method
	/// represents the main entry point. A topology is returned
	/// which contains the logical representation of the physical
	/// hardware.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::Topology;
	///
	/// let topology = Topology::new();
	/// ```
	///
	/// Note that the topology implements the Drop trait, so when
	/// it goes out of scope no further cleanup is necessary.
	pub fn new() -> Topology {
		let  mut topo: *mut ffi::HwlocTopology = std::ptr::null_mut();

		unsafe {
			ffi::hwloc_topology_init(&mut topo);
			ffi::hwloc_topology_load(topo);
		}

		Topology { topo: topo }
	}

	/// Creates a new Topology with custom flags.
	///
	/// This method works like `new`, but allows to provide a vector
	/// of flags which customize the topology discovery process.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::{Topology, TopologyFlag};
	///
	/// let topology = Topology::with_flags(vec![TopologyFlag::IoDevices]);
	/// ```
	///
	/// Note that the topology implements the Drop trait, so when
	/// it goes out of scope no further cleanup is necessary.
	pub fn with_flags(flags: Vec<TopologyFlag>) -> Topology {
		let  mut topo: *mut ffi::HwlocTopology = std::ptr::null_mut();

		let final_flag = flags
			.iter()
			.map(|f| f.to_u64().unwrap())
			.fold(0, |out, current| out | current);

		unsafe {
			ffi::hwloc_topology_init(&mut topo);
			ffi::hwloc_topology_set_flags(topo, final_flag);
			ffi::hwloc_topology_load(topo);
		}

		Topology { topo: topo }
	}

	/// Returns the flags currently set for this topology.
	///
	/// Note that the flags are only used during initialization, so this
	/// method can just be used for debugging purposes.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::{Topology,TopologyFlag};
	///
	/// let default_topology = Topology::new();
	/// assert_eq!(0, default_topology.flags().len());
	///
	/// let topology_with_flags = Topology::with_flags(vec![TopologyFlag::IoDevices]);
	/// assert_eq!(vec![TopologyFlag::IoDevices], topology_with_flags.flags());
	/// ```
	pub fn flags(&self) -> Vec<TopologyFlag> {
		let stored_flags = unsafe {
			ffi::hwloc_topology_get_flags(self.topo)
		};

		(0..64)
			.map(|x| (1 << x) & stored_flags)
			.filter(|&x| x > 0 )
			.map(|x| TopologyFlag::from_u64(x).unwrap())
			.collect::<Vec<TopologyFlag>>()
	}

	/// Returns the full depth of the topology.
	///
	/// In practice, the full depth of the topology equals the depth of the `ObjectType::PU`
	/// plus one.
	///
	/// The full topology depth is useful to know if one needs to manually traverse the
	/// complete topology.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::Topology;
	///
	/// let topology = Topology::new();
	/// assert!(topology.depth() > 0);
	/// ```
	pub fn depth(&self) -> u32 {
		unsafe {
			ffi::hwloc_topology_get_depth(self.topo)
		}
	}

	/// Returns the depth for the given `ObjectType`.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::{Topology,ObjectType};
	///
	/// let topology = Topology::new();
	///
	/// let machine_depth = topology.depth_for_type(ObjectType::Machine).unwrap();
	/// let pu_depth = topology.depth_for_type(ObjectType::PU).unwrap();
	/// assert!(machine_depth < pu_depth); 
	/// ```
	///
	/// # Failures
	///
	/// If hwloc can't find the depth for the given `ObjectType`, this method will
	/// return an error from the `TypeDepthError` enum. See this one for more info
	/// on each specific error.
	///
	/// Note that for `ObjectType::Bridge`, `ObjectType::PCIDevice` and `ObjectType::OSDevice`,
	/// always an error will be returned which signals their virtual depth.
	pub fn depth_for_type(&self, object_type: ObjectType) -> Result<u32, TypeDepthError> {
		let result = unsafe {
			ffi::hwloc_get_type_depth(self.topo, object_type)
		};

		match result {
			result if result >= 0 => Ok(result as u32),
			-1 => Err(TypeDepthError::TypeDepthUnknown),
			-2 => Err(TypeDepthError::TypeDepthMultiple),
			-3 => Err(TypeDepthError::TypeDepthBridge),
			-4 => Err(TypeDepthError::TypeDepthPCIDevice),
			-5 => Err(TypeDepthError::TypeDepthOSDevice),
			_ => Err(TypeDepthError::UnkownTypeDepthError)
		}
	}

	/// Returns the corresponding `ObjectType` for the given depth.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::{Topology,ObjectType};
	///
	/// let topology = Topology::new();
	///
	/// // Load depth for PU to assert against
	/// let pu_depth = topology.depth_for_type(ObjectType::PU).unwrap();
	/// // Retrieve the type for the given depth
	/// assert_eq!(ObjectType::PU, topology.type_at_depth(pu_depth));
	/// ```
	///
	/// # Panics
	///
	/// This method will panic if the given depth is larger than the full depth 
	/// minus one. It can't be negative since its an unsigned integer, but be
	/// careful with the depth provided in general.
	pub fn type_at_depth(&self, depth: u32) -> ObjectType {
		if depth > self.depth() - 1 {
			panic!("The provided depth {} is out of bounds.", depth);
		}

		unsafe {
			ffi::hwloc_get_depth_type(self.topo, depth)
		}
	}

	/// Returns the number of objects at the given depth.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::Topology;
	///
	/// let topology = Topology::new();
	///
	/// let topo_depth = topology.depth();
	/// assert!(topology.size_at_depth(topo_depth - 1) > 0);
	/// ```
	///
	/// # Panics
	///
	/// This method will panic if the given depth is larger than the full depth 
	/// minus one. It can't be negative since its an unsigned integer, but be
	/// careful with the depth provided in general.
	pub fn size_at_depth(&self, depth: u32) -> u32 {
		if depth > self.depth() - 1 {
			panic!("The provided depth {} is out of bounds.", depth);
		}

		unsafe {
			ffi::hwloc_get_nbobjs_by_depth(self.topo, depth)
		}
	}

	/// Returns the `TopologyObject` at the root of the topology.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::{Topology,TopologyObject};
	///
	/// let topology = Topology::new();
	///
	/// assert_eq!(topology.type_at_root(), topology.object_at_root().object_type());
	/// ```
	pub fn object_at_root(&self) -> &TopologyObject {
		self.objects_at_depth(0).first().unwrap()
	}

	/// Returns the `ObjectType` at the root of the topology.
	///
	/// This method is a convenient shorthand for `type_at_depth(0)`.
	///
	/// # Examples
	///
	/// ```
	/// use hwloc::{Topology,ObjectType};
	///
	/// let topology = Topology::new();
	///
	/// let root_type = topology.type_at_root();
	/// let depth_type = topology.type_at_depth(0);
	/// assert_eq!(root_type, depth_type);
	/// ```
	pub fn type_at_root(&self) -> ObjectType {
		self.type_at_depth(0)
	}

	/// Returns all `TopologyObjects` with the given `ObjectType`.
	pub fn objects_with_type(&self, object_type: ObjectType) -> Result<Vec<&TopologyObject>, TypeDepthError> {
		match self.depth_for_type(object_type) {
			Ok(depth) => Ok(self.objects_at_depth(depth)),
			Err(TypeDepthError::TypeDepthOSDevice) => Ok(self.objects_at_depth(-5)),
			Err(TypeDepthError::TypeDepthPCIDevice) => Ok(self.objects_at_depth(-4)),
			Err(TypeDepthError::TypeDepthBridge) => Ok(self.objects_at_depth(-3)),
			Err(e) => Err(e)
		}

		// TODO: Fix the -x stuff on the unsigned part in a sane way across the code
	}

	pub fn objects_at_depth(&self, depth: u32) -> Vec<&TopologyObject>  {
		let size = self.size_at_depth(depth);
		(0..size).map(|idx| {
			unsafe {
				&*ffi::hwloc_get_obj_by_depth(self.topo, depth, idx)
			}
		}).collect::<Vec<&TopologyObject>>()
	}

	//pub fn get_last_cpu_location(&self) {
	//	let res = unsafe {
	//		let set = std::ptr::null_mut();
	//		ffi::hwloc_get_last_cpu_location(self.topo, set, 0)
	//	};
	//
	//	panic!(format!("{:?}", res));
	//}

}

impl Drop for Topology {

	fn drop(&mut self) {
		unsafe {
			ffi::hwloc_topology_destroy(self.topo)
		}
	}

}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn should_set_and_get_flags() {
		let topo = Topology::with_flags(vec![TopologyFlag::WholeSystem, TopologyFlag::IoBridges]);
		assert_eq!(vec![TopologyFlag::WholeSystem, TopologyFlag::IoBridges], topo.flags());
	}

	#[test]
	fn should_get_topology_depth() {
		let topo = Topology::new();
		assert!(topo.depth() > 0);
	}

	#[test]
	fn should_match_types_and_their_depth() {
		let topo = Topology::new();

		let pu_depth = topo.depth_for_type(ObjectType::PU).ok().unwrap();
		assert!(pu_depth > 0);
		assert_eq!(ObjectType::PU, topo.type_at_depth(pu_depth));
	}

	#[test]
	fn should_get_nbobjs_by_depth() {
		let topo = Topology::new();
		assert!(topo.size_at_depth(1) > 0);
	}

	#[test]
	fn should_get_root_object() {
		let topo = Topology::new();

		let root_obj = topo.object_at_root();
		assert_eq!(ObjectType::Machine, root_obj.object_type());
		assert!(root_obj.memory().total_memory() > 0);
		assert_eq!(0, root_obj.memory().local_memory());
		assert_eq!(0, root_obj.depth());
		assert_eq!(0, root_obj.logical_index());
		assert_eq!(None, root_obj.next_cousin());
		assert_eq!(None, root_obj.prev_cousin());
		assert!(root_obj.first_child().is_some());
		assert!(root_obj.last_child().is_some());
		//panic!(format!("{:?}", root_obj.next_cousin()));
		//panic!(format!("{:?}", root_obj.first_child()));
	}

}