use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs;

fn sum_of_divisors(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..=n / 2 {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

fn classify_numbers(numbers: Vec<usize>) -> String {
    let mut result = String::new();
    
    for number in numbers {
        let sum = sum_of_divisors(number);
        if sum < number || number == 1 {
            result.push('D'); // Недостаточное
        } else if sum > number {
            result.push('A'); // Избыточное
        } else {
            result.push('P'); // Совершенное
        }
    }
    
    result
}

fn read_my_file() -> Vec<usize>{
    let reader = BufReader::new(File::open("file.txt").expect("Cannot open file.txt"));
    let mut v: Vec<usize> = Vec::new();
    for line in reader.lines() {
        match line {
            Err(_e) => panic!("Reading error"),
            Ok(f) => v.push(f.parse::<usize>().unwrap()),
        }
    }
    if v[0] < 2 || v[0] > 1000000{
        panic!("Incorrect file data");
    }
    return v;
}

fn write_data_in_file(answer: String){
    let _ = fs::write("answer.txt", answer);
}

fn main() {
    let numbers = read_my_file();
    let classification = classify_numbers(numbers);
    println!("{}", classification);
    write_data_in_file(classification);
}
