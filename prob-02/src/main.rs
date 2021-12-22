// use std::env;
use std::fs;

enum Direction{
    Up,
    Down,
    Forward,
    Unknown
}

struct Position{
    depth : i32,
    horizontal : i32,
    aim : i32
}

fn parse_direction(i : &str) ->  Direction{
    match i{
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
           _ => Direction::Unknown
    }

}

fn parse_command(i : &str) -> Option<(Direction, i32)>{
    let mut x = i.split(' ');
    let fst = match x.next(){
        Some(val) => parse_direction(val),
        None => {println!("Parse error! (wrong direction)"); return None;}
    };

    let snd = match x.next(){
        Some(val) => match val.parse(){
            Ok(res) => res,
            Err(e) => {println!("Parse error! {}", e); return None;}
        },
        None => {println!("Warning (parsing): missing number in inuput: \"{}\"", i); return None;}
    };
    Some((fst,snd))
}

fn parse(i : String) -> Vec<(Direction, i32)> {
    //let collected : Vec<( Direction, i32)> = i.split('\n').map(|x| parse_command(x)).collect();
    let parsed = i.split('\n').map(|x| parse_command(x));
    let mut res : Vec<(Direction,i32)> = Vec::new();
    for i in parsed{
        match i{
            Some(x) => {res.push(x);},
            None => {}
        }
    }
    res
}


fn update_position_part1(position : &mut Position, update : (Direction, i32)) ->  () {
    match update{
        (Direction::Forward,x) => {position.horizontal += x;},
        (Direction::Up,x) => {position.depth -= x;},
        (Direction::Down,x) => {position.depth += x;},
        _ => {println!("not updating");}
    }
}

fn update_position_part2(position : &mut Position, update : (Direction, i32)) ->  () {
    match update{
        (Direction::Forward,x) => {position.horizontal += x; position.depth += position.aim*x;},
        (Direction::Up,x) => {position.aim -= x;},
        (Direction::Down,x) => {position.aim += x;},
        _ => {println!("not updating");}
    }
}


fn main() {
    let filename = "input";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vec_raw = parse(contents);
    let mut position = Position{ horizontal : 0, depth : 0, aim : 0};
    for update in vec_raw{
        //update_position_part1(&mut position,update);
        update_position_part2(&mut position,update);
    }
    println!("hor: {}, depth: {}, prod: {}", position.horizontal, position.depth, position.horizontal * position.depth);

}
