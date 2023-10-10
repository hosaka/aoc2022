use itertools::Itertools;

fn part_one() -> Option<u64> {
    return include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            return sum;
        })
        .max();
}

fn part_two() -> u64 {
    return include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let partone = part_one();
    println!("{partone:?}");

    let parttwo = part_two();
    println!("{parttwo:?}");

    Ok(())
}
