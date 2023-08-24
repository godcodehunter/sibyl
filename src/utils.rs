use std::{alloc::Allocator, io};

/// Represents the tag for a process, indicating whether it's a parent or child process.
pub enum ProcessTag {
    /// Code running in a parent process, where `pid` is the process ID of the child process.
    ParentProcess(libc::pid_t),
    /// Code running in a child process
    ChildProcess,
}

/// Forks the current process into a parent and child process.
///
/// # Returns
/// - `Ok(ProcessTag::Child)` in the child process.
/// - `Ok(ProcessTag::Parent(pid))` in the parent process, where `pid` is the process ID of the child process.
/// - `Err(code)` if an error occurs during forking, where `code` is a specific error code.
pub fn fork() -> Result<ProcessTag, impl std::error::Error> {
    let res = unsafe { libc::fork() };
    match res {
        -1 => Err(io::Error::last_os_error()),
        0 => Ok(ProcessTag::ChildProcess),
        res => Ok(ProcessTag::ParentProcess(res)),
    }
}

pub struct SharedMemoryAllocator;

unsafe impl Allocator for SharedMemoryAllocator {
    fn allocate(&self, layout: std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        unsafe {
            let res = libc::mmap(
                std::ptr::null_mut(),
                layout.size(),
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
    
            match res {
                // TODO: io::Error::last_os_error()
                libc::MAP_FAILED => Err(std::alloc::AllocError),
                _ => {
                    let ptr = res as *mut u8;
                    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, layout.size()) };
                    let non_nill = std::ptr::NonNull::new_unchecked(slice);
                    Ok(non_nill)
                }
            }
        }
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
        todo!()
    }
}

// Shared memory region
//
// There is no guarantee that the regions will be placed at the same base address.
// For example, one process might have the shared region starting at address 0x60000 while the other process uses 0x70000. It is critical to understand that these two addresses refer to the exact same piece of data.
// So storing the number 1 in the first process's address 0x60000 means the second process has the value of 1 at 0x70000. The two different addresses refer to the exact same location.

