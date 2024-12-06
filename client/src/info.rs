use sysinfo::System;

pub fn get_total_mem_swap() -> (u64, u64) {
    let sys = System::new_all();
    (sys.total_memory(), sys.total_swap())
}
