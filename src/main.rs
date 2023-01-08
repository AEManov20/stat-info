use stat_info::StatisticalInfo;

fn main() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("please input something");
    let mut inp: Vec<i32> = inp
        .trim()
        .split(" ")
        .map(|part| part.parse().unwrap())
        .collect();
    
    inp.sort();
    println!("{:?}\n{:#?}", inp, StatisticalInfo::new(&inp));
}
