fn main() -> eyre::Result<()> {
    let (mut ans1, mut ans2) = (0, 0);
    for line in aoc_2024::input!(2).lines() {
        let nums = line.split(' ').map(|num| num.parse::<u32>());
        let nums = nums.collect::<Result<Vec<_>, _>>()?;
        let crit = |f: fn(_, _) -> bool| {
            let p = nums.windows(2).position(|w| !f(w[0], w[1]))?;
            let q = nums.windows(2).rposition(|w| !f(w[0], w[1]))?;
            let b = p == q && (p == 0 || f(nums[p - 1], nums[p + 1]))
                || p + 1 >= q && (p + 2 == nums.len() || f(nums[p], nums[p + 2]));
            Some(b)
        };
        let k = crit(|x, y| x < y && y - x <= 3).zip(crit(|y, x| x < y && y - x <= 3));
        ans1 += k.map_or(1, |_| 0);
        ans2 += k.map_or(1, |(x, y)| u32::from(x | y));
    }

    println!("{ans1}\n{ans2}");

    Ok(())
}
