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
    for line in input.lines().into_iter() {
        let values: Vec<&str> = line
            .split('x').collect();

        dimensions.push(Box{
            length: values[0].parse().unwrap(),
            height: values[1].parse().unwrap(),
            width: values[2].parse().unwrap(),
        });
    }

    let mut sum_sq_ft: u32 = 0;
    let mut sum_slack: u32 = 0;
    for cube in dimensions{
        let side1: u32 = 2*cube.length*cube.width;
        let side2: u32 = 2*cube.width*cube.height;
        let side3: u32 = 2*cube.height*cube.length;
        let smallest_side: u32 = (side1.min(side2).min(side3)) / 2;
        sum_slack += smallest_side;

        print!("Side1:{side1}\tSide2:{side2}\tSide3:{side3}\tsmallest:{smallest_side}");
        let length: u32 = cube.length;
        let height: u32 = cube.height;
        let width: u32 = cube.width;

        let total_box: u32 = side1 + side2 + side3 + smallest_side;
        println!("\tLength:{length}\tHeight:{height}\tWidth:{width}\tTotalBox:{total_box}");


        sum_sq_ft += side1 + side2 + side3 + smallest_side;
    }

    println!("Part 1    : {}", sum_sq_ft);
    println!("Part 1Slack: {}", sum_slack);
}