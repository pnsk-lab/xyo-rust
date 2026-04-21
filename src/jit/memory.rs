use inkwell::memory_manager::McjitMemoryManager;

#[cfg(unix)]
use std::ptr::NonNull;

#[derive(Debug)]
pub struct SectionMemoryManager {
    #[cfg(unix)]
    allocations: Vec<Allocation>,
    #[cfg(unix)]
    page_size: usize,
}

impl SectionMemoryManager {
    pub fn new() -> Self {
        Self {
            #[cfg(unix)]
            allocations: Vec::new(),
            #[cfg(unix)]
            page_size: page_size(),
        }
    }
}

impl Default for SectionMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy)]
enum AllocationKind {
    Code,
    ReadOnlyData,
    ReadWriteData,
}

#[cfg(unix)]
#[derive(Debug)]
struct Allocation {
    base_ptr: NonNull<u8>,
    map_len: usize,
    kind: AllocationKind,
}

#[cfg(unix)]
impl SectionMemoryManager {
    fn allocate_section(
        &mut self,
        size: libc::uintptr_t,
        alignment: libc::c_uint,
        kind: AllocationKind,
    ) -> *mut u8 {
        let requested = usize::max(size, 1);
        let requested_alignment = usize::max(alignment as usize, 1);
        let reserve_size = round_up_to_page(
            requested.saturating_add(requested_alignment),
            self.page_size,
        );

        let base_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                reserve_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        if base_ptr == libc::MAP_FAILED {
            return std::ptr::null_mut();
        }

        let base_addr = base_ptr as usize;
        let aligned_addr = align_up(base_addr, requested_alignment);
        let base_ptr = NonNull::new(base_ptr.cast::<u8>()).expect("mmap returned null pointer");

        self.allocations.push(Allocation {
            base_ptr,
            map_len: reserve_size,
            kind,
        });

        aligned_addr as *mut u8
    }
}

impl McjitMemoryManager for SectionMemoryManager {
    fn allocate_code_section(
        &mut self,
        size: libc::uintptr_t,
        alignment: libc::c_uint,
        _section_id: libc::c_uint,
        _section_name: &str,
    ) -> *mut u8 {
        #[cfg(unix)]
        {
            self.allocate_section(size, alignment, AllocationKind::Code)
        }

        #[cfg(not(unix))]
        {
            let _ = (size, alignment);
            panic!("MCJIT section allocation is only implemented on unix targets");
        }
    }

    fn allocate_data_section(
        &mut self,
        size: libc::uintptr_t,
        alignment: libc::c_uint,
        _section_id: libc::c_uint,
        _section_name: &str,
        is_read_only: bool,
    ) -> *mut u8 {
        #[cfg(unix)]
        {
            let kind = if is_read_only {
                AllocationKind::ReadOnlyData
            } else {
                AllocationKind::ReadWriteData
            };
            self.allocate_section(size, alignment, kind)
        }

        #[cfg(not(unix))]
        {
            let _ = (size, alignment, is_read_only);
            panic!("MCJIT section allocation is only implemented on unix targets");
        }
    }

    fn finalize_memory(&mut self) -> Result<(), String> {
        #[cfg(unix)]
        {
            for allocation in &self.allocations {
                let protection = match allocation.kind {
                    AllocationKind::Code => libc::PROT_READ | libc::PROT_EXEC,
                    AllocationKind::ReadOnlyData => libc::PROT_READ,
                    AllocationKind::ReadWriteData => libc::PROT_READ | libc::PROT_WRITE,
                };
                let result = unsafe {
                    libc::mprotect(
                        allocation.base_ptr.as_ptr().cast::<libc::c_void>(),
                        allocation.map_len,
                        protection,
                    )
                };
                if result != 0 {
                    return Err(format!(
                        "mprotect failed while finalizing MCJIT memory: {}",
                        std::io::Error::last_os_error()
                    ));
                }
            }
            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err("MCJIT memory finalization is only implemented on unix targets".to_string())
        }
    }

    fn destroy(&mut self) {
        #[cfg(unix)]
        {
            for allocation in self.allocations.drain(..) {
                let _ = unsafe {
                    libc::munmap(
                        allocation.base_ptr.as_ptr().cast::<libc::c_void>(),
                        allocation.map_len,
                    )
                };
            }
        }
    }
}

#[cfg(unix)]
fn page_size() -> usize {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size <= 0 {
        4096
    } else {
        page_size as usize
    }
}

#[cfg(unix)]
fn round_up_to_page(value: usize, page_size: usize) -> usize {
    value.div_ceil(page_size) * page_size
}

#[cfg(unix)]
fn align_up(value: usize, alignment: usize) -> usize {
    if alignment <= 1 {
        value
    } else {
        value.div_ceil(alignment) * alignment
    }
}
