static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn main() {
    add_to_counter(3);

    unsafe {
        println!("COUNTER: {COUNTER}")
    }
}
