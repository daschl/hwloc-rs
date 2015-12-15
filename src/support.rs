use libc::c_uchar;

#[repr(C)]
pub struct TopologySupport {
    discovery: *const TopologyDiscoverySupport,
    cpubind: *const TopologyCpuBindSupport,
    membind: *const TopologyMemBindSupport,
}

#[repr(C)]
pub struct TopologyDiscoverySupport {
    /// Detecting the number of PU objects is supported.
    pu: c_uchar,
}

/// Flags describing actual PU binding support for this topology.
#[repr(C)]
pub struct TopologyCpuBindSupport {
    /// Binding the whole current process is supported.
    set_thisproc_cpubind: c_uchar,
    /// Getting the binding of the whole current process is supported.
    get_thisproc_cpubind: c_uchar,
    /// Binding a whole given process is supported.
    set_proc_cpubind: c_uchar,
    /// Getting the binding of a whole given process is supported.
    get_proc_cpubind: c_uchar,
    /// Binding the current thread only is supported.
    set_thisthread_cpubind: c_uchar,
    /// Getting the binding of the current thread only is supported.
    get_thisthread_cpubind: c_uchar,
    /// Binding a given thread only is supported.
    set_thread_cpubind: c_uchar,
    /// Getting the binding of a given thread only is supported.
    get_thread_cpubind: c_uchar,
    /// Getting the last processors where the whole current process ran is supported.
    get_thisproc_last_cpu_location: c_uchar,
    /// Getting the last processors where a whole process ran is supported.
    get_proc_last_cpu_location: c_uchar,
    /// Getting the last processors where the current thread ran is supported.
    get_thisthread_last_cpu_location: c_uchar,
}

/// Flags describing actual memory binding support for this topology.
#[repr(C)]
pub struct TopologyMemBindSupport {
    /// Binding the whole current process is supported.
    set_thisproc: c_uchar,
    /// Getting the binding of the whole current process is supported.
    get_thisproc: c_uchar,
    /// Binding a whole given process is supported.
    set_proc: c_uchar,
    /// Getting the binding of a whole given process is supported.
    get_proc: c_uchar,
    /// Binding the current thread only is supported.
    set_thisthread: c_uchar,
    /// Getting the binding of the current thread only is supported.
    get_thisthread: c_uchar,
    /// Binding a given memory area is supported.
    set_area: c_uchar,
    /// Getting the binding of a given memory area is supported.
    get_area: c_uchar,
    /// Allocating a bound memory area is supported.
    alloc: c_uchar,
    /// First-touch policy is supported.
    firsttouch: c_uchar,
    /// Bind policy is supported.
    bind: c_uchar,
    /// Interleave policy is supported.
    interleave: c_uchar,
    /// Replication policy is supported.
    replicate: c_uchar,
    /// Next-touch migration policy is supported.
    nexttouch: c_uchar,
    /// Migration flags is supported.
    migrate: c_uchar,
}
