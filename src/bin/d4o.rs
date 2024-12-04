use std::arch::x86_64::*;

const fn u64(bytes: [u8; 8]) -> u64 {
    u64::from_le_bytes(bytes)
}

const fn ub(byte: u8) -> u64 {
    u64::from_ne_bytes([byte; 8])
}

const fn ib(byte: u8) -> i64 {
    i64::from_ne_bytes([byte; 8])
}

pub fn run(input: &[u8]) -> u32 {
    let input = unsafe { input.align_to::<[u8; 141]>() }.1;
    assert_eq!(input.len(), 140);
    let mut sum = 0;
    for lanes in input.windows(3) {
        for i in 0..23 {
            let t = <[_; 8]>::try_from(&lanes[0][i * 6..i * 6 + 8]).unwrap();
            let c = <[_; 8]>::try_from(&lanes[1][i * 6..i * 6 + 8]).unwrap();
            let b = <[_; 8]>::try_from(&lanes[2][i * 6..i * 6 + 8]).unwrap();

            let tb = (u64(t) & ub(!89)) + (3 * (u64(b) & ub(!89)));
            let tb = (tb << 7) | (tb >> 5);
            let ex = unsafe { _mm256_set_epi64x(ib(0x55), ib(0x77), ib(0x48), ib(0x84)) };
            let act = unsafe { _mm256_set1_epi64x(tb as _) };
            let tbf = unsafe { _mm256_cmpeq_epi8(ex, act) };

            let c = (!(u64(c) | (u64(c) << 1)) >> 4) & ub(1);
            let cf = unsafe { _mm256_set1_epi64x(c as _) };

            let f = unsafe { _mm256_and_si256(tbf, cf) };
            let psum: [u64; 4] = unsafe { std::mem::transmute(f) };
            let psum = (psum[0] + psum[1] + psum[2] + psum[3]).to_le_bytes();
            for p in psum {
                sum += p as u32;
            }
        }
    }
    sum
}

fn main() {
    let input = aoc_2024::input!(4).as_bytes();
    let sum = run(input);
    println!("{sum}");
}
