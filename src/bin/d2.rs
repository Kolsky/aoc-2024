fn main() -> eyre::Result<()> {
    let (mut ans1, mut ans2) = (0, 0);
    for line in aoc_2024::input!(2).lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        let crit = |f: fn(_, _) -> bool| {
            let Some(p) = nums.windows(2).position(|w| !f(w[0], w[1])) else {
                return 2;
            };
            (nums[p + 2..].windows(2).all(|w| f(w[0], w[1]))
                && ((p == 0 || f(nums[p - 1], nums[p + 1]))
                    && (p == nums.len() - 2 || f(nums[p + 1], nums[p + 2]))
                    || (p == nums.len() - 2 || f(nums[p], nums[p + 2])))) as u32
        };
        let k = crit(|x, y| x < y && y - x <= 3).max(crit(|y, x| x < y && y - x <= 3));
        ans1 += u32::from(k > 1);
        ans2 += k.min(1);
    }

    println!("{ans1}\n{ans2}");

    Ok(())
}
