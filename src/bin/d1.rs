fn main() -> eyre::Result<()> {
    let [mut va, mut vb] = [Vec::with_capacity(1000), Vec::with_capacity(1000)];
    for line in aoc_2024::input!(1).lines() {
        for num in line.split_ascii_whitespace() {
            va.push(num.parse::<u32>()?);
            (va, vb) = (vb, va);
        }
    }

    va.sort_unstable();
    vb.sort_unstable();

    let ans1 = std::iter::zip(&va, &vb)
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u32>();

    let mut vb = &vb[..];
    let mut ans2 = 0;
    for a in va {
        let i = vb.partition_point(|&b| a >= b);
        let j = vb.partition_point(|&b| a > b);
        vb = &vb[i..];
        ans2 += a * (i - j) as u32;
    }
    
    println!("{ans1}\n{ans2}");

    Ok(())
}
