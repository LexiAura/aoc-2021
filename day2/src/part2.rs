use std::fs;

fn main() {
    let input_string = fs::read_to_string("puzzle_input.txt").expect("Error reading file");
    
    let split_input = input_string.split("\n") // split into lines
        .collect::<Vec<&str>>()
        .iter()
        .map(|x: &&str| -> Vec<&str> {(**x).split(" ").collect::<Vec<&str>>()}) // split each line into parts
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .fold(AimCoords{horizontal: 0, depth: 0, aim: 0}, |accum: AimCoords, next: &Vec<&str>| -> AimCoords {accum.folder(next)});
    
    println!("Horizontal = {}", split_input.horizontal);
    println!("Depth = {}", split_input.depth);
    println!("Product = {}", split_input.horizontal * split_input.depth);
    
}

struct AimCoords {
    horizontal: i32,
    depth:      i32,
    aim:        i32,
}

impl AimCoords {
    fn folder(&self, command: &Vec<&str>) -> AimCoords {
        let ret: AimCoords;
        let cmd_arg = command[1].parse::<i32>().unwrap();
        if command[0] == "forward" {
            ret = AimCoords {
                horizontal: self.horizontal + cmd_arg,
                depth: self.depth + (cmd_arg * self.aim),
                aim: self.aim,
            };
        } else if command[0] == "down" {
            ret = AimCoords {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + cmd_arg,
            };
        } else if command[0] == "up" {
            ret = AimCoords {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - cmd_arg,
            };
        } else {
            ret = AimCoords {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim,
            };
        }

        ret
    }
}
