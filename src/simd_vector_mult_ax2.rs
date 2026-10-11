#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Multiplies two slices element-wise (`dest = a * b`), using AVX2 acceleration
/// when available, with a scalar fallback for remaining elements or non-x86 architectures.
pub fn multiply_arrays_simd(a: &[f32], b: &[f32], dest: &mut [f32]) {
    if a.len() != b.len() || a.len() != dest.len() {
        return;
    }

    let len = a.len();
    let mut i = 0;

    // 1. Runtime Feature Detection for x86_64
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // Safety: We verified at runtime that AVX2 is supported.
            unsafe {
                i = multiply_avx2_kernel(a, b, dest);
            }
        }
    }

    // 2. Scalar Fallback: Handle leftover elements or unsupported architectures
    while i < len {
        dest[i] = a[i] * b[i];
        i += 1;
    }
}

/// AVX2 Kernel: Processes 8 floats per iteration.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn multiply_avx2_kernel(a: &[f32], b: &[f32], dest: &mut [f32]) -> usize {
    let chunks = a.len() / 8;

    for c in 0..chunks {
        let idx = c * 8;

        // 1. Load 8 unaligned floats from `a` and `b` into SIMD registers
        let va = unsafe { _mm256_loadu_ps(a.as_ptr().add(idx)) };
        let vb = unsafe { _mm256_loadu_ps(b.as_ptr().add(idx)) };

        // 2. Multiply the two vectors in parallel
        let vres = _mm256_mul_ps(va, vb);

        // 3. Store the result vector back into `dest`
        unsafe { _mm256_storeu_ps(dest.as_mut_ptr().add(idx), vres); }
    }

    chunks * 8 // Processed elements
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiply_arrays_simd() {
        let n = 100;
        let a: Vec<f32> = (0..n).map(|x| x as f32).collect();
        let b: Vec<f32> = a.clone();
        let mut dest = vec![0.0; n];

        multiply_arrays_simd(&a, &b, &mut dest);

        for i in 0..n {
            assert_eq!(dest[i], a[i] * b[i]);
        }
    }
}