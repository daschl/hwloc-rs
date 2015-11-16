use libc::{c_int, c_uint, c_ulonglong, c_char, c_void, c_float, c_ushort, c_uchar};
use ffi::{ObjectType, CpuSet, NodeSet};

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObject {
    pub _type: ObjectType,
    pub os_index: c_uint,
    pub name: *mut c_char,
    pub memory: TopologyObjectMemory,
    pub attr: *mut TopologyObjectAttributes,
    pub depth: c_uint,
    pub logical_index: c_uint,
    pub next_cousin: *mut TopologyObject,
    pub prev_cousin: *mut TopologyObject,
    pub parent: *mut TopologyObject,
    pub sibling_rank: c_uint,
    pub next_sibling: *mut TopologyObject,
    pub prev_sibling: *mut TopologyObject,
    pub arity: c_uint,
    pub children: *mut *mut TopologyObject,
    pub first_child: *mut TopologyObject,
    pub last_child: *mut TopologyObject,
    pub symmetric_subtree: c_int,
    pub io_arity: c_uint,
    pub io_first_child: *mut TopologyObject,
    pub misc_arity: c_uint,
    pub misc_first_child: *mut TopologyObject,
    pub cpuset: *mut CpuSet,
    pub complete_cpuset: *mut CpuSet,
    pub allowed_cpuset: *mut CpuSet,
    pub nodeset: *mut NodeSet,
    pub complete_nodeset: *mut NodeSet,
    pub allowed_nodeset: *mut NodeSet,
    pub distances: *mut *mut TopologyObjectDistances,
    pub distances_count: c_uint,
    pub infos: *mut TopologyObjectInfo,
    pub infos_count: c_uint,
    pub userdata: *mut c_void,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectMemory {
    pub total_memory: c_ulonglong,
    pub local_memory: c_ulonglong,
    pub page_types_len: c_uint,
    pub page_types: *mut TopologyObjectMemoryPageType,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectMemoryPageType {
    pub size: c_ulonglong,
    pub count: c_ulonglong,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectInfo {
    pub name: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectDistances {
    pub relative_depth: c_uint,
    pub nbobjs: c_uint,
    pub latency: *mut c_float,
    pub latency_max: c_float,
    pub latency_base: c_float,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectAttributes {
    pub _bindgen_data_: [u64; 5usize],
}

impl TopologyObjectAttributes {
    pub unsafe fn cache(&mut self) -> *mut TopologyObjectCacheAttributes {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn group(&mut self) -> *mut TopologyObjectGroupAttributes {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn pcidev(&mut self) -> *mut TopologyObjectPCIDevAttributes {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn bridge(&mut self) -> *mut TopologyObjectBridgeAttributes {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn osdev(&mut self) -> *mut TopologyObjectOSDevAttributes {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectCacheAttributes {
    pub size: c_ulonglong,
    pub depth: c_uint,
    pub linesize: c_uint,
    pub associativity: c_int,
    pub _type: TopologyObjectCacheType,
}

#[derive(Debug,PartialEq)]
pub enum TopologyObjectCacheType {
    HWLOC_OBJ_CACHE_UNIFIED = 0,
    HWLOC_OBJ_CACHE_DATA = 1,
    HWLOC_OBJ_CACHE_INSTRUCTION = 2,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectGroupAttributes {
    pub depth: c_uint,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectPCIDevAttributes {
    pub domain: c_ushort,
    pub bus: c_uchar,
    pub dev: c_uchar,
    pub func: c_uchar,
    pub class_id: c_ushort,
    pub vendor_id: c_ushort,
    pub device_id: c_ushort,
    pub subvendor_id: c_ushort,
    pub subdevice_id: c_ushort,
    pub revision: c_uchar,
    pub linkspeed: c_float,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectBridgeAttributes {
    //pub upstream: Union_Unnamed4,
    pub upstream_type: TopologyObjectBridgeType,
    //pub downstream: Union_Unnamed5,
    pub downstream_type: TopologyObjectBridgeType,
    pub depth: c_uint,
}

#[derive(Debug,PartialEq)]
pub enum TopologyObjectBridgeType {
    HWLOC_OBJ_BRIDGE_HOST = 0,
    HWLOC_OBJ_BRIDGE_PCI = 1,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectOSDevAttributes {
    pub _type: TopologyObjectOSDevType,
}

#[derive(Debug,PartialEq)]
pub enum TopologyObjectOSDevType {
    HWLOC_OBJ_OSDEV_BLOCK = 0,
    HWLOC_OBJ_OSDEV_GPU = 1,
    HWLOC_OBJ_OSDEV_NETWORK = 2,
    HWLOC_OBJ_OSDEV_OPENFABRICS = 3,
    HWLOC_OBJ_OSDEV_DMA = 4,
    HWLOC_OBJ_OSDEV_COPROC = 5,
}