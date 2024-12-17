use std::fs;
use std::str::Chars;

fn main() {
    let input: String = fs::read_to_string("input/input.txt")
        .expect("Error reading input.txt");
    let input_chars = input.chars();

    // Part1
    let floor: i32 = find_final_position(input_chars);
    println!("Part1 {}", floor);


    // Part 2

}

//For Part1
fn find_final_position(input_chars: Chars) -> i32{
    let mut floor: i32 = 0;
    for ch in input_chars {
        if ch == '(' {
            floor += 1;
        }
        if ch == ')' {
            floor -= 1;
        }
    }
    floor
}

//For Part2
fn find_first_basement_position(input_chars: Chars) -> i32 {
    let mut floor: i32 = 0;
    for (index ,ch) in input_chars.into_iter() { //TODO RESUME HERE
        if ch == '(' {
            floor += 1;
        }
        if ch == ')' {
            floor -= 1;
        }
    }
    floor
}
