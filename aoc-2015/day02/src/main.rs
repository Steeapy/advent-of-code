use std::fs;

struct Box{
    l: u32,
    h: u32,
    w: u32,
}

fn main() {
    let input: String = fs::read_to_string("input/input.txt")
        .expect("Error reading input.txt");

    let dimensions: Vec<Box> = parse_input(input);

    println!("Sum Paper: {}", get_total_paper(dimensions))
}

fn get_total_paper(dimensions: Vec<Box>) -> u32{
    let mut sum_paper: u32 = 0;
    
    for present in dimensions{
        let areas: Vec<u32> = vec![
            (2 * present.l * present.w),
            (2 * present.w * present.h),
            (2 * present.h * present.l),
        ];
        let area_present: u32 = areas.iter().sum();
        let smallest_area: u32 = areas.into_iter().min().unwrap()/2;
        
        sum_paper += area_present + smallest_area;
    }
    sum_paper
}

fn parse_input(input: String) -> Vec<Box> {
    let mut dimensions: Vec<Box> = Vec::new();

    for line in input.lines().into_iter() {
        let values: Vec<&str> = line
            .split('x')
            .collect();

        dimensions.push(Box{
            l: values[0]
                .parse()
                .unwrap(),
            h: values[1]
                .parse()
                .unwrap(),
            w: values[2]
                .parse()
                .unwrap(),
        });
    }

    dimensions
}