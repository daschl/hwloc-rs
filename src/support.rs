use libc::c_uchar;
use std::fmt;

#[repr(C)]
pub struct TopologySupport {
    discovery: *const TopologyDiscoverySupport,
    cpubind: *const TopologyCpuBindSupport,
    membind: *const TopologyMemBindSupport,
}

impl fmt::Debug for TopologySupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f,
                   "{:?}, {:?}, {:?}",
                   *self.discovery,
                   *self.cpubind,
                   *self.membind)
        }
    }
}

impl TopologySupport {
    pub fn discovery(&self) -> &TopologyDiscoverySupport {
        unsafe { &*self.discovery }
    }

    pub fn cpu(&self) -> &TopologyCpuBindSupport {
        unsafe { &*self.cpubind }
    }

    pub fn memory(&self) -> &TopologyMemBindSupport {
        unsafe { &*self.membind }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TopologyDiscoverySupport {
    pu: c_uchar,
}

impl TopologyDiscoverySupport {
    /// Detecting the number of PU objects is supported.
    pub fn pu(&self) -> bool {
        self.pu == 1
    }
}

/// Flags describing actual PU binding support for this topology.
#[repr(C)]
#[derive(Debug)]
pub struct TopologyCpuBindSupport {
    set_thisproc_cpubind: c_uchar,
    get_thisproc_cpubind: c_uchar,
    set_proc_cpubind: c_uchar,
    get_proc_cpubind: c_uchar,
    set_thisthread_cpubind: c_uchar,
    get_thisthread_cpubind: c_uchar,
    set_thread_cpubind: c_uchar,
    get_thread_cpubind: c_uchar,
    get_thisproc_last_cpu_location: c_uchar,
    get_proc_last_cpu_location: c_uchar,
    get_thisthread_last_cpu_location: c_uchar,
}

impl TopologyCpuBindSupport {
    /// Binding the whole current process is supported.
    pub fn set_current_process(&self) -> bool {
        self.set_thisproc_cpubind == 1
    }

    /// Getting the binding of the whole current process is supported.
    pub fn get_current_process(&self) -> bool {
        self.get_thisproc_cpubind == 1
    }

    /// Binding a whole given process is supported.
    pub fn set_process(&self) -> bool {
        self.set_proc_cpubind == 1
    }

    /// Getting the binding of a whole given process is supported.
    pub fn get_process(&self) -> bool {
        self.get_proc_cpubind == 1
    }

    /// Binding the current thread only is supported.
    pub fn set_current_thread(&self) -> bool {
        self.set_thisthread_cpubind == 1
    }

    /// Getting the binding of the current thread only is supported.
    pub fn get_current_thread(&self) -> bool {
        self.get_thisthread_cpubind == 1
    }

    /// Binding a given thread only is supported.
    pub fn set_thread(&self) -> bool {
        self.set_thread_cpubind == 1
    }

    /// Getting the binding of a given thread only is supported.
    pub fn get_thread(&self) -> bool {
        self.get_thread_cpubind == 1
    }

    /// Getting the last processors where the whole current process ran is supported.
    pub fn get_current_process_last_cpu_location(&self) -> bool {
        self.get_thisproc_last_cpu_location == 1
    }

    /// Getting the last processors where a whole process ran is supported.
    pub fn get_process_last_cpu_location(&self) -> bool {
        self.get_proc_last_cpu_location == 1
    }

    /// Getting the last processors where the current thread ran is supported.
    pub fn get_current_thread_last_cpu_location(&self) -> bool {
        self.get_thisthread_last_cpu_location == 1
    }
}

/// Flags describing actual memory binding support for this topology.
#[repr(C)]
#[derive(Debug)]
pub struct TopologyMemBindSupport {
    set_thisproc: c_uchar,
    get_thisproc: c_uchar,
    set_proc: c_uchar,
    get_proc: c_uchar,
    set_thisthread: c_uchar,
    get_thisthread: c_uchar,
    set_area: c_uchar,
    get_area: c_uchar,
    alloc: c_uchar,
    firsttouch: c_uchar,
    bind: c_uchar,
    interleave: c_uchar,
    replicate: c_uchar,
    nexttouch: c_uchar,
    migrate: c_uchar,
}

impl TopologyMemBindSupport {
    /// Binding the whole current process is supported.
    pub fn set_current_process(&self) -> bool {
        self.set_thisproc == 1
    }

    /// Getting the binding of the whole current process is supported.
    pub fn get_current_process(&self) -> bool {
        self.get_thisproc == 1
    }

    /// Binding a whole given process is supported.
    pub fn set_process(&self) -> bool {
        self.set_proc == 1
    }

    /// Getting the binding of a whole given process is supported.
    pub fn get_process(&self) -> bool {
        self.get_proc == 1
    }

    /// Binding the current thread only is supported.
    pub fn set_current_thread(&self) -> bool {
        self.set_thisthread == 1
    }

    /// Getting the binding of the current thread only is supported.
    pub fn get_current_thread(&self) -> bool {
        self.get_thisthread == 1
    }

    /// Binding a given memory area is supported.
    pub fn set_area(&self) -> bool {
        self.set_area == 1
    }

    /// Getting the binding of a given memory area is supported.
    pub fn get_area(&self) -> bool {
        self.get_area == 1
    }

    /// Allocating a bound memory area is supported.
    pub fn alloc(&self) -> bool {
        self.alloc == 1
    }

    /// First-touch policy is supported.
    pub fn first_touch(&self) -> bool {
        self.firsttouch == 1
    }

    /// Bind policy is supported.
    pub fn bind(&self) -> bool {
        self.bind == 1
    }

    /// Interleave policy is supported.
    pub fn interleave(&self) -> bool {
        self.interleave == 1
    }

    /// Replication policy is supported.
    pub fn replicate(&self) -> bool {
        self.replicate == 1
    }

    /// Next-touch migration policy is supported.
    pub fn next_touch(&self) -> bool {
        self.nexttouch == 1
    }

    /// Migration flags is supported.
    pub fn migrate(&self) -> bool {
        self.migrate == 1
    }
}
