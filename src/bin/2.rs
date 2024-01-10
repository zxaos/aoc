use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = Path::new("inputs/input.2.txt");
    let file = File::open(&path).expect("Failed to open input");

    let reader = io::BufReader::new(file);

    let mut paper_sum: u32 = 0;
    let mut ribbon_length: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let dims = split_dimensions(line);
        paper_sum += calculate_paper_area(dims);
        ribbon_length += calculate_ribbon_length(dims);
    }

    println!("1: Total paper: {paper_sum}");
    println!("2: Total ribbon length: {ribbon_length}");
}

fn split_dimensions(input_dims: String) -> (u8, u8, u8) {
    let dims: Vec<&str> = input_dims.split('x').collect();
    if dims.len() != 3 {
        panic!("Invalid size '{input_dims}'");
    }
    let dims: Vec<u8> = dims
        .into_iter()
        .map(|d| d.parse::<u8>().expect("Invalid Size"))
        .collect();

    (dims[0], dims[1], dims[2])
}

fn calculate_paper_area(input: (u8, u8, u8)) -> u32 {
    let l = u32::from(input.0);
    let w = u32::from(input.1);
    let h = u32::from(input.2);
    let lw = l * w;
    let lh = l * h;
    let wh = w * h;
    let min_area = lw.min(lh.min(wh));
    (2 * lw) + (2 * lh) + (2 * wh) + min_area
}

fn calculate_ribbon_length(input: (u8, u8, u8)) -> u32 {
    let l = u32::from(input.0);
    let w = u32::from(input.1);
    let h = u32::from(input.2);
    let p1 = l + w;
    let p2 = l + h;
    let p3 = w + h;
    let selected_p = p1.min(p2.min(p3));
    (2 * selected_p) + (l * w * h)
}
