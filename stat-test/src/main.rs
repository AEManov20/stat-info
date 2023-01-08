use stat_info::StatisticalInfo;
use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines();
    while let Some(Ok(inp)) = lines.next()
    {
        let mut inp: Vec<i32> = inp
            .trim()
            .split(" ")
            .map(|part| part.parse().expect("invalid number"))
            .collect();
        inp.sort();
        println!("{:?}\n{:?}\n", inp, StatisticalInfo::new(&inp));
    }
}