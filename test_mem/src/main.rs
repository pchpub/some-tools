use std::{alloc::{alloc, dealloc, Layout}, time::Duration};

fn main() {
    unsafe {
        let test_memory = 20.0;
        let mut num = (test_memory*1024.0*1024.0/4.0) as i64;
        while num >= 0{
            let layout = Layout::from_size_align(4096, 1024).unwrap();
            let _ptr = alloc(layout);  
            num -= 1;
        }
        std::thread::sleep(Duration::from_secs(1145141919810));
        // dealloc(ptr, layout);
    }
}
