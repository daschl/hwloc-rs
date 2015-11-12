use libc::{c_int, c_uint, c_ulonglong};
use num::{ToPrimitive, FromPrimitive};

pub enum HwlocTopology {}

#[repr(u32)]
#[derive(Debug,PartialEq)]
pub enum ObjectType {
	/// The whole system that is accessible to hwloc. That may comprise several 
	/// machines in SSI systems like Kerrighed.
	System,
	/// The typical root object type. A set of processors and memory with cache 
	/// coherency.
	Machine,
	/// A set of processors around memory which the processors can directly 
	/// access.
	NUMANode,
	/// Physical package, what goes into a socket. In the physical meaning, 
	/// i.e. that you can add or remove physically.
	Package,
	/// The Cache. Can be L1i, L1d, L2, L3,...
	Cache,
	/// A computation unit (may be shared by several logical processors).
	Core,
	/// Processing Unit, or (Logical) Processor.
	///
	/// An execution unit (may share a core with some other logical 
	/// processors, e.g. in the case of an SMT core). Objects of this kind 
	/// are always reported and can thus be used as fallback when others are 
	/// not.
	PU,
	/// Group objects.
	///	Objects which do not fit in the above but are detected by hwloc and 
	/// are useful to take into account for affinity. For instance, some 
	/// operating systems expose their arbitrary processors aggregation this
	/// way. And hwloc may insert such objects to group NUMA nodes according 
	/// to their distances.
	///
	/// These objects are ignored when they do not bring any structure.
	Group,
	/// Miscellaneous objects.
	///
	/// Objects without particular meaning, that can e.g. be
	/// added by the application for its own use, or by hwloc
	/// for miscellaneous objects such as MemoryModule (DIMMs).
	Misc,
	/// Any bridge that connects the host or an I/O bus, to another I/O bus.
	///
	/// Bridge objects have neither CPU sets nor node sets.
	/// They are not added to the topology unless I/O discovery
	/// is enabled through the custom flags.
	Bridge,
	/// PCI device.
	///
	/// These objects have neither CPU sets nor node sets.
	/// They are not added to the topology unless I/O discovery
	/// is enabled through the custom flags.
	PCIDevice,
	/// Operating system device.
	///	
	/// These objects have neither CPU sets nor node sets. They are not 
	/// added to the topology unless I/O discovery is enabled 
	/// through the custom flags.
	OSDevice,
	/// An internal sentinel value.
	TypeMax,
}

#[derive(Debug,PartialEq)]
pub enum TypeDepthError {
	/// HWLOC returned a depth error which is not known to the rust binding.
	UnkownTypeDepthError,
	/// No object of given type exists in the topology.
	TypeDepthUnknown,
	/// Objects of given type exist at different depth in the topology.
	TypeDepthMultiple,
	/// Virtual depth for bridge object level.
	TypeDepthBridge,
	/// Virtual depth for PCI device object level.
	TypeDepthPCIDevice,
	/// Virtual depth for software device object level.
	TypeDepthOSDevice,
}

#[derive(Debug,PartialEq)]
pub enum TopologyFlag {
	WholeSystem,
	IsThisSystem,
	IoDevices,
	IoBridges,
	WholeIo,
	ICaches,
}

impl ToPrimitive for TopologyFlag {
	
	fn to_i64(&self) -> Option<i64> {
		match *self {
			TopologyFlag::WholeSystem => Some(1),
			TopologyFlag::IsThisSystem => Some(2),
			TopologyFlag::IoDevices => Some(4),
			TopologyFlag::IoBridges => Some(8),
			TopologyFlag::WholeIo => Some(16),
			TopologyFlag::ICaches => Some(32),
		}
	}

	fn to_u64(&self) -> Option<u64> {
		self.to_i64().and_then(|x| x.to_u64())
	}

}

impl FromPrimitive for TopologyFlag {

	fn from_i64(n: i64) -> Option<Self> {
		match n {
			1 => Some(TopologyFlag::WholeSystem),
			2 => Some(TopologyFlag::IsThisSystem),
			4 => Some(TopologyFlag::IoDevices),
			8 => Some(TopologyFlag::IoBridges),
			16 => Some(TopologyFlag::WholeIo),
			32 => Some(TopologyFlag::ICaches),
			_ => None,
		}
	}

    fn from_u64(n: u64) -> Option<Self> {
    	FromPrimitive::from_i64(n as i64)
    }
}

#[link(name = "hwloc")]
extern "C" {

	// === Topology Creation and Destruction ===

	pub fn hwloc_topology_init(topology: *mut *mut HwlocTopology) -> c_int;
	pub fn hwloc_topology_load(topology: *mut HwlocTopology) -> c_int;
	pub fn hwloc_topology_destroy(topology: *mut HwlocTopology);

	// === Topology Detection Configuration and Query ===

	pub fn hwloc_topology_set_flags(topology: *mut HwlocTopology, flags: c_ulonglong) -> c_int;
	pub fn hwloc_topology_get_flags(topology: *mut HwlocTopology) -> c_ulonglong;

	// === Object levels, depths and types ===

	pub fn hwloc_topology_get_depth(topology: *mut HwlocTopology) -> c_uint;
	pub fn hwloc_get_type_depth(topology: *mut HwlocTopology, object_type: ObjectType) -> c_int;
	pub fn hwloc_get_depth_type(topology: *mut HwlocTopology, depth: c_uint) -> ObjectType;
	pub fn hwloc_get_nbobjs_by_depth(topology: *mut HwlocTopology, depth: c_uint) -> c_uint;
	pub fn hwloc_get_obj_by_depth(topology: *mut HwlocTopology, depth: c_uint, idx: c_uint) -> ObjectType;
}