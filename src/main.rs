use std::env;
use std::fs;

fn main() -> () {
    let mut input: String;

    // Day 1
    input = get_input("1.txt");
    println!("#####\nDay 1\n#####");
    one(input);

    // Day 2
    println!("#####\nDay 2\n#####");
    input = get_input("2.txt");
    two(input);
}

fn two(input: String) -> () {
    let lines = input.clone();
    let first: usize = lines
        .split("\n")
        .map(|line| {
            let mut splitline = line.split(" ").into_iter();
            let (play, response) = (splitline.next().unwrap(), splitline.next().unwrap());
            let mut score: usize = match response {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("Invalid input play"),
            };
            score += match (play, response) {
                ("B", "X") | ("C", "Y") | ("A", "Z") => 0,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                ("C", "X") | ("A", "Y") | ("B", "Z") => 6,
                _ => panic!("Invalid input line"),
            };
            score
        })
        .sum();
    let second: usize = input
        .split("\n")
        .map(|line| {
            let mut splitline = line.split(" ").into_iter();
            let (play, response) = (splitline.next().unwrap(), splitline.next().unwrap());
            let mut score: usize = match response {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("Invalid input response"),
            };
            score += match (play, response) {
                ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
                ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
                _ => panic!("Invalid input line"),
            };
            score
        })
        .sum();
    println!("Problem 1:\n{}\n", first);
    println!("Problem 2:\n{}\n", second);
}

fn one(input: String) -> () {
    let snackbags = input
        .trim()
        .split("\n\n")
        .map(|snackbag| snackbag.split("\n").map(|cal| cal.parse::<i32>().unwrap()))
        .map(|s| s.fold(0, |acc, x| acc + x))
        .collect::<Vec<_>>();

    println! {"Problem 1:\n{}\n", snackbags.iter().max().unwrap()};

    let mut iter = snackbags.into_iter();
    let mut top_three: usize = 0;
    for _ in 0..3 {
        top_three += iter.next().unwrap() as usize;
    }
    println! {"Problem 2:\n{}\n", top_three};
}

fn get_input(file: &str) -> String {
    let mut path = env::current_dir().expect("Cannot read current directory");
    path.push("data");
    path.push(file);
    fs::read_to_string(path).expect("Unable to read file")
}
