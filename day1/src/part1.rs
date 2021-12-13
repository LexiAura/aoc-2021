use std::fs;

fn main() {
    let input_string = fs::read_to_string("puzzle_input.txt").expect("Error reading file");

    let int_list = input_string.split("\n")
                               .collect::<Vec<&str>>()
                               .iter()
                               .map(|i: &&str| -> i32 {(**i).parse::<i32>().unwrap()})
                               .collect::<Vec<i32>>();
                                        
    let num_of_increases = int_list.iter()
        .fold((0, 100_000), |(accum, last): (i32, i32), next: &i32| -> (i32, i32) {(accum + ((*next > last) as i32), *next)}).0;
    
    println!("Number of times depth increased: {}", num_of_increases);

}