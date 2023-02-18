/*!
- <https://github.com/envoyproxy/envoy/blob/main/api/envoy/admin/v3/memory.proto>
- <https://www.envoyproxy.io/docs/envoy/latest/api-v3/admin/v3/memory.proto>
*/

/**
Proto representation of the internal memory consumption of an Envoy instance. These represent values extracted from an internal TCMalloc instance. For more information, see the section of the docs entitled ['Generic Tcmalloc Status'](https://gperftools.github.io/gperftools/tcmalloc.html).
*/
pub struct Memory {
    /// The number of bytes allocated by the heap for Envoy. This is an alias for `generic.current_allocated_bytes`.
    allocated: u64,

    /// The number of bytes reserved by the heap but not necessarily allocated. This is an alias for `generic.heap_size`.
    heap_size: u64,

    /// The number of bytes in free, unmapped pages in the page heap. These bytes always count towards virtual memory usage, and depending on the OS, typically do not count towards physical memory usage. This is an alias for `tcmalloc.pageheap_unmapped_bytes`.
    pageheap_unmapped: u64,

    /// The number of bytes in free, mapped pages in the page heap. These bytes always count towards virtual memory usage, and unless the underlying memory is swapped out by the OS, they also count towards physical memory usage. This is an alias for `tcmalloc.pageheap_free_bytes`.
    pageheap_free: u64,

    /// The amount of memory used by the TCMalloc thread caches (for small objects). This is an alias for `tcmalloc.current_total_thread_cache_bytes`.
    total_thread_cache: u64,

    /// The number of bytes of the physical memory usage by the allocator. This is an alias for `generic.total_physical_bytes`.
    total_physical_bytes: u64,
}
