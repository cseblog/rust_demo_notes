use std::{fs::File, io::Read, path::Path};
use adventofcode::read_file_as_string;

fn main() {
    println!("Hello day 1");
    let s = read_file_as_string("src/adventofcode/data/input_day1.txt".to_string()).unwrap();
    get_max_calorine(s);
}

fn get_max_calorine(s: String) -> i32 {
    let mut calories = Vec::new();
    let mut sum = 0;

    for i in s.lines() {
        if i.trim().is_empty() {
            calories.push(sum);
            calories.sort_by(|a, b| b.cmp(a));
            sum = 0;
        } else {
            sum += i.to_string().trim().parse::<i32>().unwrap_or(0);
        }
    }

    calories.truncate(3);
    let total = calories.iter().sum();
    println!("Max calories: {}", calories[0]);
    println!("Total colorine: {}", total);
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let input = "10
        1
        2

        9
        5

        20
        
        2
        
        1
        1

        5
        5"
        .to_string();
        assert_eq!(47, get_max_calorine(input));
    }
}
