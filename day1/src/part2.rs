use std::fs;

fn main() {
    let input_string = fs::read_to_string("puzzle_input.txt").expect("Error reading file");

    let int_list = input_string.split("\n")
                               .collect::<Vec<&str>>()
                               .iter()
                               .map(|i: &&str| -> i32 {(**i).parse::<i32>().unwrap()})
                               .collect::<Vec<i32>>();
                                        
    let num_of_increases = int_list.iter()
        .fold((0, (100_000, 100_000, 100_000)), 
        |(accum, last_three): (i32, (i32, i32, i32)), next: &i32| 
        window_is_larger((accum, last_three), *next)).0;
    
    println!("Number of times depth increased: {}", num_of_increases);
}

/**
 * The fold function for line 12. This could surely be made much more concise.
 */
fn window_is_larger((accum, last_three): (i32, (i32, i32, i32)), next: i32) -> (i32, (i32, i32, i32)) {
    (accum + ((next + last_three.1 + last_three.2 > (last_three.0 + last_three.1 + last_three.2)) as i32), (last_three.1, last_three.2, next))

}
