use std::fs;


struct Box{
    length: u32,
    height: u32,
    width: u32,
}

fn main() {
    let input: String = fs::read_to_string("input/input.txt")
        .expect("Error reading input.txt");
    let mut dimensions: Vec<Box> = Vec::new();
    let mut sum_sq_ft: u32 = 0;

    for line in input.lines().into_iter() {
        let values: Vec<&str> = line
            .split('x').collect();

        dimensions.push(Box{
            length: values[0].parse().unwrap(),
            height: values[1].parse().unwrap(),
            width: values[2].parse().unwrap(),
        });
    }

    for cube in dimensions{
        let side1: u32 = 2*cube.length + 2*cube.width;
        let side2: u32 = 2*cube.width + 2*cube.height;
        let side3: u32 = 2*cube.height + 2*cube.length;
        let smallest_side: u32 = (side1.min(side2).min(side3)) / 2;

        sum_sq_ft += side1 + side2 + side3 + smallest_side;
    }

    println!("Part 1: {}", sum_sq_ft);
}