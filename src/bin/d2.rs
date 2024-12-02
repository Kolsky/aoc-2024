fn test<'a>(into_iter: impl IntoIterator<Item = &'a u32>, n: usize) -> bool {
    let mut iter = into_iter.into_iter().enumerate();
    let (_, mut a) = iter.nth(if n == 0 { 1 } else { 0 }).unwrap();
    for (i, b) in iter {
        if i != n && !(a < b && b - a <= 3) {
            return true;
        }
        if i != n {
            a = b;
        }
    }
    false
}

fn main() -> eyre::Result<()> {
    let (mut ans1, mut ans2) = (0, 0);
    for line in aoc_2024::input!(2).lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        let nums = || nums.iter();
        let n = nums().len();
        let inc = (0..=n).take_while(|i| test(nums(), n - i)).count();
        let dec = (0..=n).take_while(|i| test(nums().rev(), n - i)).count();
        if inc == 0 || dec == 0 {
            ans1 += 1;
        }
        if inc <= n || dec <= n {
            ans2 += 1;
        }
    }

    println!("{ans1}\n{ans2}");

    Ok(())
}
