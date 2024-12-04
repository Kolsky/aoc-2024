fn lattice<const N: usize>(mut f: impl FnMut([&[u8; N]; N], usize, usize)) {
    let (mut lines, pn) = ([&[][..]; N], N - 1);
    for (i, line) in aoc_2024::input!(4).lines().enumerate() {
        lines[i % N] = line.as_bytes();
        for j in N..=lines[pn].len() {
            let a = std::array::from_fn(|k| lines[(i - pn + k) % N][j - N..j].try_into().unwrap());
            f(a, i - pn, j - N);
        }
    }
}

fn main() {
    let mut ans1 = 0;
    lattice::<4>(|a, i, j| {
        let h = |b: &[u8; 4]| u32::from(b == b"XMAS") + u32::from(b == b"SAMX");
        let v = |f: fn(_) -> _| h(&[a[0][f(0)], a[1][f(1)], a[2][f(2)], a[3][f(3)]]);
        (i == 0).then(|| ans1 += h(a[0]) + h(a[1]) + h(a[2]));
        (j == 0).then(|| ans1 += v(|_| 0) + v(|_| 1) + v(|_| 2));
        ans1 += h(a[3]) + v(|_| 3) + v(|i| i) + v(|i| 3 - i);
    });

    let mut ans2 = 0;
    lattice::<3>(|arr, _, _| {
        let m = &[arr[0][0], arr[0][2], arr[2][2], arr[2][0]];
        let m = [b"MMSS", b"MSSM", b"SSMM", b"SMMS"].contains(&m);
        ans2 += u32::from(arr[1][1] == b'A' && m);
    });

    println!("{ans1}\n{ans2}");
}
