use stat_info::StatisticalInfo;
use std::{io::BufRead, time};


fn main() {
    
    let mut lines = std::io::stdin().lock().lines();
    while let Some(Ok(inp)) = lines.next()
    {
        let now = time::Instant::now();

        let mut inp: Vec<i32> = inp
            .trim()
            .split(" ")
            .map(|part| part.parse().expect("invalid number"))
            .collect();
        inp.sort();
        println!("{:?}\n{:?}", inp, StatisticalInfo::new(&inp));

        if cfg!(debug_assertions) {
            println!("{:?}\n", time::Instant::now().duration_since(now));
        } else { println!(); }
    }

}