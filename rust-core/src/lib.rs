use std::collections::HashSet;
use std::ffi::CString;
use std::os::raw::c_char;
use sysinfo::{System};

/// Collects a list of running processes with their PIDs, ensuring each entry is unique.
///
/// # Safety
/// The returned pointer must be passed to `free_cstring` to prevent a memory leak.
/// The memory is owned by Rust until `free_cstring` is called.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn collect_unique_processes_list() -> *mut c_char {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut unique_processes = HashSet::new();

    for (pid, process) in sys.processes() {
        let process_name = process.name().to_string_lossy();
        let process_entry = format!("{}:{}", pid.as_u32(), process_name);
        unique_processes.insert(process_entry);
    }

    let results: Vec<String> = unique_processes.into_iter().collect();
    let joined = results.join("\n");

    // This allocates memory that Rust manages. `into_raw` gives up ownership.
    CString::new(joined).unwrap_or_default().into_raw()
}

/// Frees the memory for a C-style string that was allocated by Rust.
///
/// # Safety
/// This function must be called with a pointer that was obtained from a Rust
/// function that returned a `CString::into_raw` pointer. Calling it with any
/// other pointer will lead to undefined behavior.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_cstring(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // `CString::from_raw` takes ownership of the pointer and the memory.
    // When this CString goes out of scope at the end of the function,
    // Rust's memory manager will automatically deallocate it.
    unsafe{
        let _ = CString::from_raw(ptr);
    }
}