use std::fs;


fn main() {
    let input: String = fs::read_to_string("input/input.txt")
        .expect("Error reading input.txt");

    let input_chars = input.chars();
    let mut floor: i32 = 0;

    for ch in input_chars {
        if ch == '(' {
            floor += 1;
        }
        if ch == ')' {
            floor -= 1;
        }
    }
    println!("Part1 {}", floor);
}
