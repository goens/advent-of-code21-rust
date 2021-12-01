use std::env;
use std::fs;

fn parse(i : String) -> Vec<i32> {
    let res: Vec<i32> = i.split('\n')
        .map(|x|  x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect();
    res
}

fn main() {
    let filename = "input";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec = parse(contents);
    let mut last = vec[0];
    let mut total = 0;
    for x in vec {
        if x > last {
           total += 1;
        }
        last = x;
    }
    println!("Total increases:\n{}", total );
}
