use libc::{c_int, c_uint, c_ulonglong, c_char, c_void, c_float, c_ushort, c_uchar};
use std::ffi::CString;
use std::str::from_utf8;

use ffi::{ObjectType, CpuSet, NodeSet};

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObject {
    object_type: ObjectType,
    os_index: c_uint,
    name: *mut c_char,
    memory: TopologyObjectMemory,
    attr: *mut TopologyObjectAttributes, // todo: getter
    depth: c_uint,
    logical_index: c_uint,
    next_cousin: *mut TopologyObject,
    prev_cousin: *mut TopologyObject,
    parent: *mut TopologyObject,
    sibling_rank: c_uint,
    next_sibling: *mut TopologyObject,
    prev_sibling: *mut TopologyObject,
    arity: c_uint,
    children: *mut *mut TopologyObject,
    first_child: *mut TopologyObject,
    last_child: *mut TopologyObject,
    symmetric_subtree: c_int, // todo: getter
    io_arity: c_uint, // todo: getter
    io_first_child: *mut TopologyObject, // todo: getter
    misc_arity: c_uint, // todo: getter
    misc_first_child: *mut TopologyObject, // todo: getter
    cpuset: *mut CpuSet, // todo: getter
    complete_cpuset: *mut CpuSet, // todo: getter
    allowed_cpuset: *mut CpuSet, // todo: getter
    nodeset: *mut NodeSet, // todo: getter
    complete_nodeset: *mut NodeSet, // todo: getter
    allowed_nodeset: *mut NodeSet, // todo: getter
    distances: *mut *mut TopologyObjectDistances, // todo: getter
    distances_count: c_uint, // todo: getter
    infos: *mut TopologyObjectInfo, // todo: getter
    infos_count: c_uint, // todo: getter
    userdata: *mut c_void, // todo: getter
}

impl TopologyObject {

    /// The type of the object.
    pub fn object_type(&self) -> ObjectType {
        self.object_type.clone()
    }

    /// The memory attributes of the object.
    pub fn memory(&self) -> &TopologyObjectMemory {
        &self.memory
    }

    /// The OS-provided physical index number.
    ///
    /// It is not guaranteed unique across the entire machine, 
    /// except for PUs and NUMA nodes.
    pub fn os_index(&self) -> u32 {
        self.os_index
    }

    /// The name of the object, if set.
    pub fn name(&self) -> String {
        let c_str = unsafe { CString::from_raw(self.name) };
        c_str.to_str().unwrap().to_string()
    }

    /// Vertical index in the hierarchy.
    ///
    /// If the topology is symmetric, this is equal to the parent 
    /// depth plus one, and also equal to the number of parent/child 
    /// links from the root object to here.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Horizontal index in the whole list of similar objects, hence guaranteed 
    /// unique across the entire machine.
    ///
    /// Could be a "cousin_rank" since it's the rank within the "cousin" list below.
    pub fn logical_index(&self) -> u32 {
        self.logical_index
    }

    /// This objects index in the parents children list.
    pub fn sibling_rank(&self) -> u32 {
        self.sibling_rank
    }

    /// The number of direct children.
    pub fn arity(&self) -> u32 {
        self.arity
    }

    /// All direct children of this object.
    pub fn children(&self) -> Vec<&TopologyObject> {
        (0..self.arity())
            .map(|i| unsafe { &**self.children.offset(i as isize) })
            .collect::<Vec<&TopologyObject>>()
    }

    /// Next object of same type and depth.
    pub fn next_cousin(&self) -> Option<&TopologyObject> {
        self.derefObj(&self.next_cousin)
    }

    /// Previous object of same type and depth.
    pub fn prev_cousin(&self) -> Option<&TopologyObject> {
        self.derefObj(&self.prev_cousin)
    }

    /// First child of the next depth.
    pub fn first_child(&self) -> Option<&TopologyObject> {
        self.derefObj(&self.first_child)
    }

    /// Last child of the next depth.
    pub fn last_child(&self) -> Option<&TopologyObject> {
        self.derefObj(&self.last_child)
    }

    /// Last child of the next depth.
    pub fn parent(&self) -> Option<&TopologyObject> {
       self.derefObj(&self.parent)
    }

    /// Previous object below the same parent.
    pub fn prev_sibling(&self) -> Option<&TopologyObject> {
        self.derefObj(&self.prev_sibling)
    }

    /// Next object below the same parent.
    pub fn next_sibling(&self) -> Option<&TopologyObject> {
        self.derefObj(&self.next_sibling)
    }

    fn derefObj(&self, p: &*mut TopologyObject) -> Option<&TopologyObject> {
        unsafe { if p.is_null() { None } else { Some(&**p) } }
    }
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectMemory {
    total_memory: c_ulonglong,
    local_memory: c_ulonglong,
    page_types_len: c_uint, // todo: getter
    page_types: *mut TopologyObjectMemoryPageType, // todo: getter
}

impl TopologyObjectMemory {
    /// The total memory (in bytes) in this object and its children.
    pub fn total_memory(&self) -> u64 {
        self.total_memory
    }

    /// The local memory (in bytes) in this object.
    pub fn local_memory(&self) -> u64 {
        self.local_memory
    }
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
    relative_depth: c_uint,
    nbobjs: c_uint,
    latency: *mut c_float, // TODO: getter (expose properly)
    latency_max: c_float,
    latency_base: c_float,
}

impl TopologyObjectDistances {

    /// Relative depth of the considered objects below the 
    /// object containing this distance information.
    pub fn relative_depth(&self) -> u32 {
        self.relative_depth
    }

    /// Number of objects considered in the matrix.
    ///
    /// It is the number of descendant objects at relative_depth below
    /// the containing object.
    pub fn number_of_objects(&self) -> u32 {
        self.nbobjs
    }

    /// The maximal value in the latency matrix.
    pub fn max_latency(&self) -> f32 {
        self.latency_max
    }

    /// The multiplier that should be applied to latency matrix to 
    /// retrieve the original OS-provided latencies.
    ///
    /// Usually 10 on Linux since ACPI SLIT uses 10 for local latency.
    pub fn base_latency(&self) -> f32 {
        self.latency_base
    }

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
    Unified= 0,
    Data = 1,
    Instruction = 2,
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
    Host = 0,
    PCI = 1,
}

#[repr(C)]
#[derive(Debug,PartialEq)]
pub struct TopologyObjectOSDevAttributes {
    pub _type: TopologyObjectOSDevType,
}

#[derive(Debug,PartialEq)]
pub enum TopologyObjectOSDevType {
    Block = 0,
    GPU = 1,
    Network = 2,
    OpenFabrics = 3,
    DMA = 4,
    COPROC = 5,
}