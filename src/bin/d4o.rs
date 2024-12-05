#![feature(portable_simd)]

use std::simd::*;
use cmp::SimdPartialEq;

fn bits(t: &[u8; 141], c: &[u8; 141], b: &[u8; 141], offset: usize) -> u32 {
    let w = u8x16::from_slice(&t[offset..]);
    let x = u8x16::from_slice(&t[offset + 2..]);
    let a = u8x16::from_slice(&c[offset + 1..]);
    let y = u8x16::from_slice(&b[offset..]);
    let z = u8x16::from_slice(&b[offset + 2..]);

    let wz = (w ^ z).simd_eq(u8x16::splat(b'M' ^ b'S'));
    let xy = (x ^ y).simd_eq(u8x16::splat(b'M' ^ b'S'));
    let a = a.simd_eq(u8x16::splat(b'A'));

    (wz & a & xy).to_bitmask() as u32
}

pub fn run(input: &[u8]) -> u32 {
    let input = unsafe { input.align_to::<[u8; 141]>() }.1;
    assert_eq!(input.len(), 140);
    let mut sum = 0;
    for lanes in input.windows(3) {
        let (t, c, b) = (&lanes[0], &lanes[1], &lanes[2]);
        for i in 0..8 {
            sum += bits(t, c, b, 16 * i).count_ones();
        }
        sum += (bits(t, c, b, 16 * 8 - 6) & (!63u32)).count_ones();
    }
    sum
}

fn main() {
    let input = aoc_2024::input!(4).as_bytes();
    let sum = run(input);
    println!("{sum}");
}
