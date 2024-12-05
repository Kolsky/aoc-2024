#![feature(portable_simd)]

use std::simd::*;
use cmp::SimdPartialEq;

fn bits(input: &[u8], i: usize) -> u64 {
    let w = u8x64::from_slice(&input[64 * i..]);
    let x = u8x64::from_slice(&input[64 * i + 2..]);
    let a = u8x64::from_slice(&input[141 + 64 * i + 1..]);
    let y = u8x64::from_slice(&input[141 * 2 + 64 * i..]);
    let z = u8x64::from_slice(&input[141 * 2 + 64 * i + 2..]);

    let wz = (w ^ z).simd_eq(u8x64::splat(b'M' ^ b'S'));
    let xy = (x ^ y).simd_eq(u8x64::splat(b'M' ^ b'S'));
    let a = a.simd_eq(u8x64::splat(b'A'));

    (wz & a & xy).to_bitmask()
}

pub fn run(input: &[u8]) -> u32 {
    assert_eq!(input.len(), 141 * 140);
    let mut sum = 0;
    for i in 0..304 {
        sum += bits(input, i).count_ones();
    }
    sum
}

fn main() {
    let input = aoc_2024::input!(4).as_bytes();
    let t = std::time::Instant::now();
    let sum = run(input);
    dbg!(t.elapsed());
    println!("{sum}");
}
