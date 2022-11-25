use std::slice;

static HELLO_WORLD: &str = "Hello World";

#[test]
fn unsafe_test() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1:{}", *r1);
        println!("r2:{}", *r2);
    }
    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        dangerous();
    }
}
unsafe fn dangerous() {}

fn split_at_mut(slice:&mut [i32], mid:usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn extern_test() {
    unsafe {
        println!("Absolute value is is {}",abs(-3));
    }
}

// Function names are not changed at compile time
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER:u32 = 0;
fn add_to_count(inc:u32) {
    unsafe {
        COUNTER += inc;
    }
}


fn unsafe_test_two() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo{

}

unsafe impl Foo for i32 {

}













