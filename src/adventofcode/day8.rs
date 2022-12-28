use adventofcode::*;
use serde_json::to_string;

fn main() {
    println!("hello");
    let file_str = read_file_as_string("src/adventofcode/data/input_day8.txt".to_string());
}

fn process(s: String) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let lines = s.lines();
    
    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        line.as_bytes()
            .iter()
            .for_each(|c| row.push(c.clone() as u32));
        matrix.push(row);
    }

    return matrix;
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(0, 0)
    }
}
