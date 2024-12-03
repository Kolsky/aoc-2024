fn main() {
    let (mut ans1, mut ans2) = (0, 0);
    let mut incl = true;
    for line in aoc_2024::input!(3).split("do") {
        incl = (incl || line.starts_with("()")) && !line.starts_with("n't()");
        for r in line.split("mul(") {
            let prod = (|| {
                let (l, r) = r.split_once(')')?.0.split_once(',')?;
                Some(l.parse::<u32>().ok()? * r.parse::<u32>().ok()?)
            })();
            ans1 += prod.unwrap_or(0);
            ans2 += prod.unwrap_or(0) * u32::from(incl);
        }
    }
    println!("{ans1}\n{ans2}");
}
