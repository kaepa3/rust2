use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

static HELLO_WORLD: &str = "hello world";
static mut COUNTER: u32 = 0;
extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc:u32){
    unsafe {
        COUNTER += inc;
    }
}
unsafe trait Foo{

}
unsafe impl Foo for i32{

}

#[cfg(test)]
mod tests {
    use crate::COUNTER;
    use crate::abs;
    use crate::HELLO_WORLD;
    use core::slice;
    use crate::add_to_count;

    #[test]
    fn increment() {
        add_to_count(3);
        unsafe {
            println!("Counter: {}", COUNTER)
        }
    }
    #[test]
    fn hello() {
        println!("name is {}", HELLO_WORLD);
    }
    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    #[test]
    fn slicer() {
        let address = 0x012345usize;
        let r = address as *mut i32;

        let slice = unsafe { slice::from_raw_parts_mut(r, 1000) };
    }
    #[test]
    fn abser() {
        unsafe {
            println!("-3 abs to C:{}", abs(-3));
        }
    }
}
