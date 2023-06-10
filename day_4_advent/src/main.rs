use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_range_a_in_range_b(range_a: &str, range_b: &str) -> bool {
    let (start_a, end_a): (i32, i32) = {
        let parts: Vec<i32> = range_a.split('-').map(|x| x.parse().unwrap()).collect();
        (parts[0], parts[1])
    };
    
    let (start_b, end_b): (i32, i32) = {
        let parts: Vec<i32> = range_b.split('-').map(|x| x.parse().unwrap()).collect();
        (parts[0], parts[1])
    };
    
    start_b <= start_a && end_a <= end_b
}

fn main() {
    let file = File::open("/home/oem/Desktop/AdventOfCode/AdventOfCode2022/day_4_advent/src/input.txt").expect("Failed to open input.txt");
    let reader = BufReader::new(file);
    //printea el archivo

    
    let assignment_pairs: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|entry| entry.trim().to_string())
        .collect();

    //println!("{:?}", assignment_pairs);
    
    let mut number_of_contains = 0;
    
    for assignment_pair in assignment_pairs {
        let mut ranges = assignment_pair.split(',');
        // println!("{:?}", ranges);
        let first_range = ranges.next().expect("Invalid input");
        let second_range = ranges.next().expect("Invalid input");
        
        if is_range_a_in_range_b(&first_range, &second_range) || is_range_a_in_range_b(&second_range, &first_range) {
            number_of_contains += 1;
        }
    }
    
    println!("{}", number_of_contains);
}
