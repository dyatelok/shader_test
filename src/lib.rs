#![cfg_attr(
    target_arch = "spirv",
    feature(register_attr),
    register_attr(spirv),
    no_std
)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
//#![deny(warnings)]

extern crate spirv_std;

use glam::UVec3;
use spirv_std::glam;
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

// Adapted from the wgpu hello-compute example

fn fib(f : (u128, u128)) -> (u128, u128) {
    (f.1, f.0+f.1)
}

pub fn fibonachi(n: u128) -> Option<u128> {
    /*let mut f : (u128, u128) = (0,1);
    for _ in 0..n {
        f = fib(f);
        if f.1 >= 200000000000000000000000000000000000000 {
            return None;
        }
    }
    Some(f.1)*/
    let mut a = 1;
    for _ in 0..1000000 {
        a += 1;
    }
    if n > 20 {return None;}
    Some(n.pow(10))
}

pub fn collatz(mut n: u32) -> Option<u32> {
    let mut i = 0;
    if n == 0 {
        return None;
    }
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            // Overflow? (i.e. 3*n + 1 > 0xffff_ffff)
            if n >= 0x5555_5555 {
                return None;
            }
            // TODO: Use this instead when/if checked add/mul can work: n.checked_mul(3)?.checked_add(1)?
            3 * n + 1
        };
        i += 1;
    }
    Some(i)
}

// LocalSize/numthreads of (x = 64, y = 1, z = 1)
#[spirv(compute(threads(64)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] prime_indices: &mut [u32],
) {
    let index = id.x as usize;
    prime_indices[index] = collatz(prime_indices[index]).unwrap_or(u32::MAX);
}
