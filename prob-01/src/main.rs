use std::env;
use std::fs;

fn parse(i : String) -> Vec<i32> {
    let res: Vec<i32> = i.split('\n')
        .map(|x|  x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect();
    res
}


const window_size: usize = 3; // = 1 for first star

fn sliding_window_sum(i : Vec<i32>) -> Vec<i32> {
  let mut window : [i32; window_size] = [0; window_size];
  let mut pos : usize = 0;
  let mut res : Vec<i32> = Vec::new();
  let mut window_full = false;
    for x in i {
        window[pos] = x;
        pos = (pos + 1) % window_size;
        if pos == 0 {  window_full = true }
        if window_full { res.push(window.iter().sum())}
    }
    res
}

fn main() {
    let filename = "input";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec_raw = parse(contents);
    let vec = sliding_window_sum(vec_raw);
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
