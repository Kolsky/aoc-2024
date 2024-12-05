#![feature(portable_simd)]

use cmp::SimdPartialEq;
use std::simd::*;

fn h(v: u8x64, hmask: u64) -> u32 {
    let x = v.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let m = v.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let a = v.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let s = v.simd_eq(u8x64::splat(b'S')).to_bitmask();

    let h = x & (m >> 1) & (a >> 2) & (s >> 3) & hmask;
    let rh = (x >> 3) & (m >> 2) & (a >> 1) & s & hmask;

    h.count_ones() + rh.count_ones()
}

fn xmas(input: &[u8], i: usize) -> u32 {
    let d = (61 * i).saturating_sub(141 * 137 - 64);
    let mask = (!0) << d;

    let w = u8x64::from_slice(&input[61 * i - d..]);
    let x = u8x64::from_slice(&input[141 + 61 * i - d..]);
    let y = u8x64::from_slice(&input[141 * 2 + 61 * i - d..]);
    let z = u8x64::from_slice(&input[141 * 3 + 61 * i - d..]);

    let xv = w.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mv = x.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let av = y.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let sv = z.simd_eq(u8x64::splat(b'S')).to_bitmask();

    let xrv = z.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mrv = y.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let arv = x.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let srv = w.simd_eq(u8x64::splat(b'S')).to_bitmask();

    let hmask = if i < 2 { mask } else { mask & !(1 << 48 - 1) };
    let vmask = if i == 0 { mask } else { mask & !7u64 };

    let h = if i < 3 {
        h(w, hmask) + h(x, hmask) + h(y, hmask) + h(z, hmask)
    } else {
        h(z, hmask)
    };
    let v = xv & mv & av & sv & vmask;
    let rv = xrv & mrv & arv & srv & vmask;
    let md = xv & (mv >> 1) & (av >> 2) & (sv >> 3) & mask;
    let rmd = xrv & (mrv >> 1) & (arv >> 2) & (srv >> 3) & mask;
    let ad = (xv >> 3) & (mv >> 2) & (av >> 1) & sv & mask;
    let rad = (xrv >> 3) & (mrv >> 2) & (arv >> 1) & srv & mask;
    h + (v.count_ones() + md.count_ones() + ad.count_ones())
        + (rv.count_ones() + rmd.count_ones() + rad.count_ones())
}

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
    if false {
        for i in 0..317 {
            sum += xmas(input, i);
        }
        return sum;
    }
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
