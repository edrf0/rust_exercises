use std::mem::{self, ManuallyDrop};

/// 1. Take a value out of a mutable Option, leaving None behind via std::mem::take.
pub fn take_and_reset(val: &mut Option<String>) -> String {
    mem::take(val).unwrap_or_default()
}

/// 2. Swap two elements in a mutable slice at indices `i` and `j` using std::mem::swap.
pub fn swap_elements(slice: &mut [i32], i: usize, j: usize) {
    // unsafe {
    //     let ptr_i = slice.as_mut_ptr().add(i);
    //     let ptr_j = slice.as_mut_ptr().add(j);
    //     mem::swap(&mut *ptr_i, &mut *ptr_j);
    // }
    slice.swap(i, j);
}

/// 3. Replace an old configuration string with a new one, returning the old config.
pub fn replace_config(current: &mut String, new_config: String) -> String {
    mem::replace(current, new_config)
}

/// 4. Return the size of type T in bytes using std::mem::size_of.
pub fn check_type_size<T>() -> usize {
    size_of::<T>()
}

/// 1. Explicitly drop an owned value early using std::mem::drop.
pub fn force_early_drop<T>(item: T) {
    drop(item);
}

/// 2. Extract the inner value from a ManuallyDrop wrapper.
pub fn unwrap_manual_drop(wrapped: ManuallyDrop<String>) -> String {
    ManuallyDrop::into_inner(wrapped)
}

/// 3. Reinterpret 4 bytes as an u32 using std::mem::transmute.
pub unsafe fn bytes_to_u32(bytes: [u8; 4]) -> u32 {
    // unsafe { mem::transmute::<[u8; 4], u32>(bytes) }
    u32::from_ne_bytes(bytes)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_utilities() {
        let mut opt = Some("Rust".to_string());
        let res = take_and_reset(&mut opt);
        assert_eq!(res, "Rust");
        assert_eq!(opt, None);

        let mut data = vec![1, 2, 3];
        swap_elements(&mut data, 0, 2);
        assert_eq!(data, vec![3, 2, 1]);

        let mut cfg = "v1".to_string();
        let old = replace_config(&mut cfg, "v2".to_string());
        assert_eq!(old, "v1");
        assert_eq!(cfg, "v2");

        assert_eq!(check_type_size::<u32>(), 4);
    }

    #[test]
    fn test_advanced_mem() {
        let s = "early_drop".to_string();
        force_early_drop(s);

        let wrapped = ManuallyDrop::new("manual".to_string());
        let unwrapped = unwrap_manual_drop(wrapped);
        assert_eq!(unwrapped, "manual");

        let target: u32 = 42;
        let encoded = target.to_ne_bytes();
        let decoded = unsafe { bytes_to_u32(encoded) };
        assert_eq!(decoded, 42);
    }
}