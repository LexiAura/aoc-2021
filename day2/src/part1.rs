use std::fs;

fn main() {
    let input_string = fs::read_to_string("puzzle_input.txt").expect("Error reading file");
    
    let split_input = input_string.split("\n") // split into lines
        .collect::<Vec<&str>>()
        .iter()
        .map(|x: &&str| -> Vec<&str> {(**x).split(" ").collect::<Vec<&str>>()}) // split each line into parts
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|command| cmd_to_diff(command))
        .collect::<Vec<(i32, i32)>>().iter()
        .fold((0,0), |(in_a1, in_a2), (in_b1, in_b2)| (in_a1 + in_b1, in_a2 + in_b2));
    
    println!("Horizontal = {}", split_input.0);
    println!("Depth = {}", split_input.1);
    println!("Product = {}", split_input.1 * split_input.0);
    
}

fn cmd_to_diff(command: &Vec<&str>) -> (i32, i32) {
    let ret: (i32, i32);
    // let mut ret;
    if command[0] == "forward" {
        ret = (command[1].parse::<i32>().unwrap(), 0);
    } else if command[0] == "down" {
        ret = (0, command[1].parse::<i32>().unwrap());
    } else if command[0] == "up" {
        ret = (0, -1 * command[1].parse::<i32>().unwrap());
    } else {
        ret = (0, 0);
    }

    ret
}