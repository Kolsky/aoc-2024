fn main() -> eyre::Result<()> {
    let mut lines = aoc_2024::input!(5).lines();
    let mut rules = hashbrown::HashSet::new();
    for line in &mut lines {
        let Some((l, r)) = line.split_once('|') else {
            break;
        };
        rules.insert((l.parse()?, r.parse()?));
    }

    let (mut ans1, mut ans2) = (0, 0);
    for line in lines {
        let pages = line.split(',').map(|num| num.parse::<u32>());
        let pages = pages.collect::<Result<Vec<_>, _>>()?;
        let mut prio = vec![0; pages.len()];
        for i in 0..pages.len() {
            for j in i + 1..pages.len() {
                if rules.contains(&(pages[i], pages[j])) {
                    prio[j] += 1;
                } else if rules.contains(&(pages[j], pages[i])) {
                    prio[i] += 1;
                }
            }
        }

        if prio.is_sorted() {
            ans1 += pages[pages.len() / 2];
        } else {
            let Some(midpoint) = prio.iter().position(|&x| x == pages.len() / 2) else {
                eyre::bail!("priority list is unusual");
            };
            ans2 += pages[midpoint];
        }
    }

    println!("{ans1}\n{ans2}");

    Ok(())
}
