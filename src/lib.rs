extern crate libc;
extern crate num;

mod ffi;

pub use ffi::{ObjectType, TypeDepthError, TopologyFlag};
use num::{ToPrimitive, FromPrimitive};

pub struct Topology {
	topo: *mut ffi::HwlocTopology
}

impl Topology {

	pub fn new() -> Topology {
		let  mut topo: *mut ffi::HwlocTopology = std::ptr::null_mut();

		unsafe {
			ffi::hwloc_topology_init(&mut topo);
			ffi::hwloc_topology_load(topo);
		}

		Topology { topo: topo }
	}

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

	pub fn get_flags(&self) -> Vec<TopologyFlag> {
		let stored_flags = unsafe {
			ffi::hwloc_topology_get_flags(self.topo)
		};

		(0..64)
			.map(|x| (1 << x) & stored_flags)
			.filter(|&x| x > 0 )
			.map(|x| TopologyFlag::from_u64(x).unwrap())
			.collect::<Vec<TopologyFlag>>()
	}

	pub fn get_topology_depth(&self) -> u32 {
		unsafe {
			ffi::hwloc_topology_get_depth(self.topo)
		}
	}

	pub fn get_type_depth(&self, object_type: ObjectType) -> Result<u32, TypeDepthError> {
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

	pub fn get_depth_type(&self, depth: u32) -> ObjectType {
		unsafe {
			ffi::hwloc_get_depth_type(self.topo, depth)
		}
	}

	pub fn get_nbobjs_by_depth(&self, depth: u32) -> u32 {
		unsafe {
			ffi::hwloc_get_nbobjs_by_depth(self.topo, depth)
		}
	}

	pub fn get_obj_by_depth(&self, depth: u32, idx: u32) -> ObjectType {
		unsafe {
			ffi::hwloc_get_obj_by_depth(self.topo, depth, idx)
		}
	}

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
		assert_eq!(vec![TopologyFlag::WholeSystem, TopologyFlag::IoBridges], topo.get_flags());
	}

	#[test]
	fn should_get_topology_depth() {
		let topo = Topology::new();
		assert!(topo.get_topology_depth() > 0);
	}

	#[test]
	fn should_match_types_and_their_depth() {
		let topo = Topology::new();

		let pu_depth = topo.get_type_depth(ObjectType::PU).ok().unwrap();
		assert!(pu_depth > 0);
		assert_eq!(ObjectType::PU, topo.get_depth_type(pu_depth));
	}

	#[test]
	fn should_get_nbobjs_by_depth() {
		let topo = Topology::new();
		assert!(topo.get_nbobjs_by_depth(1) > 0);
	}

	#[test]
	fn should_get_object_at_depth_and_index() {
		let topo = Topology::new();

		let pu_depth = topo.get_type_depth(ObjectType::Machine).ok().unwrap();
		assert_eq!(ObjectType::System, topo.get_obj_by_depth(pu_depth, 1));
	}

}