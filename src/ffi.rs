use libc::{c_int, c_uint, c_ulonglong, c_char, pid_t, pthread_t};
use num::{ToPrimitive, FromPrimitive};
use topology_object::TopologyObject;
use bitmap::IntHwlocBitmap;
use std::cmp::{PartialOrd, Ordering};
use support::TopologySupport;

pub enum HwlocTopology {}

/// Represents the type of a topology object.
///
/// Note that (partial) ordering for object types is implemented as a call
/// into the `hwloc` library which defines ordering as follows:
///
/// - A == B if `ObjectType::A` and `ObjectType::B` are the same.
/// - A < B if `ObjectType::A` includes objects of type `ObjectType::B`.
/// - A > B if objects of `ObjectType::A` are included in type `ObjectType::B`.
///
/// It can also help to think of it as comparing the relative depths of each type, so
/// a `ObjectType::System` will be smaller than a `ObjectType::PU` since the system
/// contains processing units.
#[repr(u32)]
#[derive(Debug,Clone)]
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
    ///
    /// Objects which do not fit in the above but are detected by hwloc and
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

impl PartialOrd for ObjectType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let compared = unsafe { hwloc_compare_types(self.clone(), other.clone()) };
        match compared {
            c if c < 0 => Some(Ordering::Less),
            c if c == 0 => Some(Ordering::Equal),
            c if c > 0 => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl PartialEq for ObjectType {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum TypeDepthError {
    /// No object of given type exists in the topology.
    TypeDepthUnknown = -1,
    /// Objects of given type exist at different depth in the topology.
    TypeDepthMultiple = -2,
    /// Virtual depth for bridge object level.
    TypeDepthBridge = -3,
    /// Virtual depth for PCI device object level.
    TypeDepthPCIDevice = -4,
    /// Virtual depth for software device object level.
    TypeDepthOSDevice = -5,
    /// HWLOC returned a depth error which is not known to the rust binding.
    Unkown = -99,
}

const TOPOLOGY_FLAG_WHOLE_SYSTEM: i64 = 1;
const TOPOLOGY_FLAG_IS_THIS_SYSTEM: i64 = 2;
const TOPOLOGY_FLAG_IO_DEVICES: i64 = 4;
const TOPOLOGY_FLAG_IO_BRIDGES: i64 = 8;
const TOPOLOGY_FLAG_WHOLE_IO: i64 = 16;
const TOPOLOGY_FLAG_I_CACHES: i64 = 32;

#[derive(Debug,PartialEq)]
pub enum TopologyFlag {
    WholeSystem = TOPOLOGY_FLAG_WHOLE_SYSTEM as isize,
    IsThisSystem = TOPOLOGY_FLAG_IS_THIS_SYSTEM as isize,
    IoDevices = TOPOLOGY_FLAG_IO_DEVICES as isize,
    IoBridges = TOPOLOGY_FLAG_IO_BRIDGES as isize,
    WholeIo = TOPOLOGY_FLAG_WHOLE_IO as isize,
    ICaches = TOPOLOGY_FLAG_I_CACHES as isize,
}

impl ToPrimitive for TopologyFlag {
    fn to_i64(&self) -> Option<i64> {
        match *self {
            TopologyFlag::WholeSystem => Some(TopologyFlag::WholeSystem as i64),
            TopologyFlag::IsThisSystem => Some(TopologyFlag::IsThisSystem as i64),
            TopologyFlag::IoDevices => Some(TopologyFlag::IoDevices as i64),
            TopologyFlag::IoBridges => Some(TopologyFlag::IoBridges as i64),
            TopologyFlag::WholeIo => Some(TopologyFlag::WholeIo as i64),
            TopologyFlag::ICaches => Some(TopologyFlag::ICaches as i64),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().and_then(|x| x.to_u64())
    }
}

impl FromPrimitive for TopologyFlag {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            TOPOLOGY_FLAG_WHOLE_SYSTEM => Some(TopologyFlag::WholeSystem),
            TOPOLOGY_FLAG_IS_THIS_SYSTEM => Some(TopologyFlag::IsThisSystem),
            TOPOLOGY_FLAG_IO_DEVICES => Some(TopologyFlag::IoDevices),
            TOPOLOGY_FLAG_IO_BRIDGES => Some(TopologyFlag::IoBridges),
            TOPOLOGY_FLAG_WHOLE_IO => Some(TopologyFlag::WholeIo),
            TOPOLOGY_FLAG_I_CACHES => Some(TopologyFlag::ICaches),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }
}

#[cfg(target_os = "windows")]
#[link(name = "libhwloc")]
extern "C" {

    // === Topology Creation and Destruction ===

    pub fn hwloc_topology_init(topology: *mut *mut HwlocTopology) -> c_int;
    pub fn hwloc_topology_load(topology: *mut HwlocTopology) -> c_int;
    pub fn hwloc_topology_destroy(topology: *mut HwlocTopology);

    // === Topology Detection Configuration and Query ===

    pub fn hwloc_topology_set_flags(topology: *mut HwlocTopology, flags: c_ulonglong) -> c_int;
    pub fn hwloc_topology_get_flags(topology: *mut HwlocTopology) -> c_ulonglong;
    pub fn hwloc_topology_get_support(topology: *mut HwlocTopology) -> *const TopologySupport;

    // === Object levels, depths and types ===

    pub fn hwloc_topology_get_depth(topology: *mut HwlocTopology) -> c_uint;
    pub fn hwloc_get_type_depth(topology: *mut HwlocTopology, object_type: ObjectType) -> c_int;
    pub fn hwloc_get_depth_type(topology: *mut HwlocTopology, depth: c_uint) -> ObjectType;
    pub fn hwloc_get_nbobjs_by_depth(topology: *mut HwlocTopology, depth: c_uint) -> c_uint;


    pub fn hwloc_get_obj_by_depth(topology: *mut HwlocTopology,
                                  depth: c_uint,
                                  idx: c_uint)
                                  -> *mut TopologyObject;

    // === CPU Binding ===
    pub fn hwloc_set_cpubind(topology: *mut HwlocTopology,
                             set: *const IntHwlocBitmap,
                             flags: c_int)
                             -> c_int;
    pub fn hwloc_get_cpubind(topology: *mut HwlocTopology,
                             set: *mut IntHwlocBitmap,
                             flags: c_int)
                             -> c_int;
    pub fn hwloc_get_last_cpu_location(topology: *mut HwlocTopology,
                                       set: *mut IntHwlocBitmap,
                                       flags: c_int)
                                       -> c_int;
    pub fn hwloc_get_proc_last_cpu_location(topology: *mut HwlocTopology,
                                            pid: pid_t,
                                            set: *mut IntHwlocBitmap,
                                            flags: c_int)
                                            -> c_int;
    pub fn hwloc_set_proc_cpubind(topology: *mut HwlocTopology,
                                  pid: pid_t,
                                  set: *const IntHwlocBitmap,
                                  flags: c_int)
                                  -> c_int;
    pub fn hwloc_get_proc_cpubind(topology: *mut HwlocTopology,
                                  pid: pid_t,
                                  set: *mut IntHwlocBitmap,
                                  flags: c_int)
                                  -> c_int;
    pub fn hwloc_set_thread_cpubind(topology: *mut HwlocTopology,
                                    thread: pthread_t,
                                    set: *const IntHwlocBitmap,
                                    flags: c_int)
                                    -> c_int;
    pub fn hwloc_get_thread_cpubind(topology: *mut HwlocTopology,
                                    pid: pthread_t,
                                    set: *mut IntHwlocBitmap,
                                    flags: c_int)
                                    -> c_int;

    // === Memory Binding ===


    // === Bitmap Methods ===
    pub fn hwloc_bitmap_alloc() -> *mut IntHwlocBitmap;
    pub fn hwloc_bitmap_alloc_full() -> *mut IntHwlocBitmap;
    pub fn hwloc_bitmap_free(bitmap: *mut IntHwlocBitmap);
    pub fn hwloc_bitmap_list_asprintf(strp: *mut *mut c_char,
                                      bitmap: *const IntHwlocBitmap)
                                      -> c_int;
    pub fn hwloc_bitmap_set(bitmap: *mut IntHwlocBitmap, id: c_uint);
    pub fn hwloc_bitmap_set_range(bitmap: *mut IntHwlocBitmap, begin: c_uint, end: c_int);
    pub fn hwloc_bitmap_clr(bitmap: *mut IntHwlocBitmap, id: c_uint);
    pub fn hwloc_bitmap_clr_range(bitmap: *mut IntHwlocBitmap, begin: c_uint, end: c_int);
    pub fn hwloc_bitmap_weight(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_zero(bitmap: *mut IntHwlocBitmap);
    pub fn hwloc_bitmap_iszero(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_isset(bitmap: *const IntHwlocBitmap, id: c_uint) -> c_int;
    pub fn hwloc_bitmap_singlify(bitmap: *mut IntHwlocBitmap);
    pub fn hwloc_bitmap_not(result: *mut IntHwlocBitmap, bitmap: *const IntHwlocBitmap);
    pub fn hwloc_bitmap_first(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_last(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_dup(src: *const IntHwlocBitmap) -> *mut IntHwlocBitmap;
    pub fn hwloc_bitmap_compare(left: *const IntHwlocBitmap,
                                right: *const IntHwlocBitmap)
                                -> c_int;
    pub fn hwloc_bitmap_isequal(left: *const IntHwlocBitmap,
                                right: *const IntHwlocBitmap)
                                -> c_int;
    pub fn hwloc_bitmap_isfull(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_next(bitmap: *const IntHwlocBitmap, prev: c_int) -> c_int;

    pub fn hwloc_obj_type_snprintf(into: *mut c_char,
                                   size: c_int,
                                   object: *const TopologyObject,
                                   verbose: bool)
                                   -> c_int;
    pub fn hwloc_obj_attr_snprintf(into: *mut c_char,
                                   size: c_int,
                                   object: *const TopologyObject,
                                   separator: *const c_char,
                                   verbose: bool)
                                   -> c_int;

    pub fn hwloc_compare_types(type1: ObjectType, type2: ObjectType) -> c_int;
}

#[cfg(not(target_os = "windows"))]
#[link(name = "hwloc")]
extern "C" {

    // === Topology Creation and Destruction ===

    pub fn hwloc_topology_init(topology: *mut *mut HwlocTopology) -> c_int;
    pub fn hwloc_topology_load(topology: *mut HwlocTopology) -> c_int;
    pub fn hwloc_topology_destroy(topology: *mut HwlocTopology);

    // === Topology Detection Configuration and Query ===

    pub fn hwloc_topology_set_flags(topology: *mut HwlocTopology, flags: c_ulonglong) -> c_int;
    pub fn hwloc_topology_get_flags(topology: *mut HwlocTopology) -> c_ulonglong;
    pub fn hwloc_topology_get_support(topology: *mut HwlocTopology) -> *const TopologySupport;

    // === Object levels, depths and types ===

    pub fn hwloc_topology_get_depth(topology: *mut HwlocTopology) -> c_uint;
    pub fn hwloc_get_type_depth(topology: *mut HwlocTopology, object_type: ObjectType) -> c_int;
    pub fn hwloc_get_depth_type(topology: *mut HwlocTopology, depth: c_uint) -> ObjectType;
    pub fn hwloc_get_nbobjs_by_depth(topology: *mut HwlocTopology, depth: c_uint) -> c_uint;


    pub fn hwloc_get_obj_by_depth(topology: *mut HwlocTopology,
                                  depth: c_uint,
                                  idx: c_uint)
                                  -> *mut TopologyObject;

    // === CPU Binding ===
    pub fn hwloc_set_cpubind(topology: *mut HwlocTopology,
                             set: *const IntHwlocBitmap,
                             flags: c_int)
                             -> c_int;
    pub fn hwloc_get_cpubind(topology: *mut HwlocTopology,
                             set: *mut IntHwlocBitmap,
                             flags: c_int)
                             -> c_int;
    pub fn hwloc_get_last_cpu_location(topology: *mut HwlocTopology,
                                       set: *mut IntHwlocBitmap,
                                       flags: c_int)
                                       -> c_int;
    pub fn hwloc_get_proc_last_cpu_location(topology: *mut HwlocTopology,
                                            pid: pid_t,
                                            set: *mut IntHwlocBitmap,
                                            flags: c_int)
                                            -> c_int;
    pub fn hwloc_set_proc_cpubind(topology: *mut HwlocTopology,
                                  pid: pid_t,
                                  set: *const IntHwlocBitmap,
                                  flags: c_int)
                                  -> c_int;
    pub fn hwloc_get_proc_cpubind(topology: *mut HwlocTopology,
                                  pid: pid_t,
                                  set: *mut IntHwlocBitmap,
                                  flags: c_int)
                                  -> c_int;
    pub fn hwloc_set_thread_cpubind(topology: *mut HwlocTopology,
                                    thread: pthread_t,
                                    set: *const IntHwlocBitmap,
                                    flags: c_int)
                                    -> c_int;
    pub fn hwloc_get_thread_cpubind(topology: *mut HwlocTopology,
                                    pid: pthread_t,
                                    set: *mut IntHwlocBitmap,
                                    flags: c_int)
                                    -> c_int;

    // === Memory Binding ===


    // === Bitmap Methods ===
    pub fn hwloc_bitmap_alloc() -> *mut IntHwlocBitmap;
    pub fn hwloc_bitmap_alloc_full() -> *mut IntHwlocBitmap;
    pub fn hwloc_bitmap_free(bitmap: *mut IntHwlocBitmap);
    pub fn hwloc_bitmap_list_asprintf(strp: *mut *mut c_char,
                                      bitmap: *const IntHwlocBitmap)
                                      -> c_int;
    pub fn hwloc_bitmap_set(bitmap: *mut IntHwlocBitmap, id: c_uint);
    pub fn hwloc_bitmap_set_range(bitmap: *mut IntHwlocBitmap, begin: c_uint, end: c_int);
    pub fn hwloc_bitmap_clr(bitmap: *mut IntHwlocBitmap, id: c_uint);
    pub fn hwloc_bitmap_clr_range(bitmap: *mut IntHwlocBitmap, begin: c_uint, end: c_int);
    pub fn hwloc_bitmap_weight(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_zero(bitmap: *mut IntHwlocBitmap);
    pub fn hwloc_bitmap_iszero(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_isset(bitmap: *const IntHwlocBitmap, id: c_uint) -> c_int;
    pub fn hwloc_bitmap_singlify(bitmap: *mut IntHwlocBitmap);
    pub fn hwloc_bitmap_not(result: *mut IntHwlocBitmap, bitmap: *const IntHwlocBitmap);
    pub fn hwloc_bitmap_first(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_last(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_dup(src: *const IntHwlocBitmap) -> *mut IntHwlocBitmap;
    pub fn hwloc_bitmap_compare(left: *const IntHwlocBitmap,
                                right: *const IntHwlocBitmap)
                                -> c_int;
    pub fn hwloc_bitmap_isequal(left: *const IntHwlocBitmap,
                                right: *const IntHwlocBitmap)
                                -> c_int;
    pub fn hwloc_bitmap_isfull(bitmap: *const IntHwlocBitmap) -> c_int;
    pub fn hwloc_bitmap_next(bitmap: *const IntHwlocBitmap, prev: c_int) -> c_int;

    pub fn hwloc_obj_type_snprintf(into: *mut c_char,
                                   size: c_int,
                                   object: *const TopologyObject,
                                   verbose: bool)
                                   -> c_int;
    pub fn hwloc_obj_attr_snprintf(into: *mut c_char,
                                   size: c_int,
                                   object: *const TopologyObject,
                                   separator: *const c_char,
                                   verbose: bool)
                                   -> c_int;

    pub fn hwloc_compare_types(type1: ObjectType, type2: ObjectType) -> c_int;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_convert_flag_to_primitive() {
        assert_eq!(1, TopologyFlag::WholeSystem as u64);
        assert_eq!(16, TopologyFlag::WholeIo as u64);
    }

    #[test]
    fn should_compare_object_types() {
        assert!(ObjectType::Machine == ObjectType::Machine);
        assert!(ObjectType::PU == ObjectType::PU);

        assert!(ObjectType::Machine < ObjectType::PU);
        assert!(ObjectType::PU > ObjectType::Cache);
    }

}
