use std::fs::read_to_string;

fn main() {
    // Read the file input.txt
    let read_lines = read_lines("src/input.txt");

    // Split the first and the second line
    let mut columns1 = Vec::new();
    let mut columns2 = Vec::new();

    for line in read_lines {
        let splited_cols: Vec<&str> = line.split_whitespace().collect();

        if splited_cols.len() == 2 {
            let col1: i32 = splited_cols[0].parse().expect("Failed to parse col1");
            let col2: i32 = splited_cols[1].parse().expect("Failed to parse col2");

            columns1.push(col1);
            columns2.push(col2);
        }
    }

    // sort the first and the second line 
    columns1.sort();
    columns2.sort();
    
    // subtract the first line from the second line
    let mut difference = Vec::new();
    for i in 0..columns1.len() {
        difference.push((columns1[i] - columns2[i]).abs());
    }

    // adding the difference
    let difference_sum: i32 = difference.iter().sum();

    // print the result
    println!("{:?} ", difference_sum)
}

// Read file 
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
