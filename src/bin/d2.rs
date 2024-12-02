fn test<'a>(
    into_iter: impl IntoIterator<Item = &'a u32, IntoIter: ExactSizeIterator>,
    n: usize,
) -> usize {
    let mut iter = into_iter.into_iter().enumerate();
    let len = iter.len();
    let (_, mut a) = iter.nth(if n == 0 { 1 } else { 0 }).unwrap();
    for (i, b) in iter {
        if i != n && !(a < b && b - a <= 3) {
            return i;
        }
        if i != n {
            a = b;
        }
    }
    len
}

fn main() -> eyre::Result<()> {
    let (mut ans1, mut ans2) = (0, 0);
    for line in aoc_2024::input!(2).lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        let ie = test(nums.iter(), nums.len());
        let de = test(nums.iter().rev(), nums.len());
        if [ie, de].contains(&nums.len()) {
            ans1 += 1;
            ans2 += 1;
        } else {
            let w = test(nums.iter(), ie - 1);
            let x = test(nums.iter(), ie);
            let y = test(nums.iter().rev(), de - 1);
            let z = test(nums.iter().rev(), de);
            if [w, x, y, z].contains(&nums.len()) {
                ans2 += 1;
            }
        }
    }

    println!("{ans1}\n{ans2}");

    Ok(())
}
