use adventofcode::*;
use std::collections::HashMap;

fn main() {
    println!("Hello day 2");
    let s = read_file_as_string("src/adventofcode/data/input_day2.txt".to_string()).unwrap();
    let res = process(s.clone());
    let res_2 = process_2(s);

    println!("Res: {}", res.unwrap());
    println!("Res_2: {}", res_2.unwrap());
}

fn process(s: String) -> Result<u64, String> {
    let mut sum = 0;

    for line in s.lines() {
        let words: Vec<_> = line.split(" ").collect();

        if words.len() != 2 {
            return Err(format!("Invalid input line: {}", line));
        }

        let s = resolve(words[1].to_string())?;
        let b = get_bonus(&s)?;
        let w = fight(words[0].to_string(), s)?;
        sum += b + w;
    }

    Ok(sum)
}

fn process_2(s: String) -> Result<u64, String> {
    let lines = s.lines();
    let mut sum = 0;

    for l in lines {
        let c: Vec<_> = l.split(" ").collect();
        println!("{} {}", c[0], c[1]);
        let s = resolve_2(c[1].to_string(), c[0].to_string());
        sum += s.unwrap();
    }
    return Ok(sum);
}

fn get_bonus(c: &str) -> Result<u64, String> {
    match c {
        "A" => Ok(1),
        "B" => Ok(2),
        "C" => Ok(3),
        _ => Err("Invalid input!".to_string()),
    }
}

fn fight(s1: String, s2: String) -> Result<u64, String> {
    let kv = [
        ("AB", 6),
        ("AC", 0),
        ("AA", 3),
        ("BA", 0),
        ("BC", 6),
        ("BB", 3),
        ("CA", 6),
        ("CB", 0),
        ("CC", 3),
    ];
    let map: HashMap<&str, u64> = HashMap::from(kv);
    let k = s1 + s2.as_str();

    match map.get(k.as_str()) {
        Some(x) => Ok(*x),
        None => Err("Invalid input!".to_string()),
    }
}

fn resolve_2(s1: String, s: String) -> Result<u64, String> {
    //lose
    let bonus_1 = match s.as_str() {
        "A" => get_bonus(&"C".to_string()),
        "B" => get_bonus(&"A".to_string()),
        "C" => get_bonus(&"B".to_string()),
        &_ => Err("invalid".to_string()),
    }?;

    //draw
    let bonus_2 = match s.as_str() {
        "A" => get_bonus(&"A".to_string()),
        "B" => get_bonus(&"B".to_string()),
        "C" => get_bonus(&"C".to_string()),
        &_ => return Err("invalid".to_string()),
    }?;

    //win
    let bonus_3 = match s.as_str() {
        "A" => get_bonus(&"B".to_string()),
        "B" => get_bonus(&"C".to_string()),
        "C" => get_bonus(&"A".to_string()),
        &_ => return Err("invalid".to_string()),
    }?;

    match s1.as_str() {
        "X" => Ok(0 + bonus_1),
        "Y" => Ok(3 + bonus_2),
        "Z" => Ok(6 + bonus_3),
        &_ => Err("Invalid".to_string()),
    }
}

fn resolve(s: String) -> Result<String, String> {
    match s.as_str() {
        "X" => Ok("A".to_string()),
        "Y" => Ok("B".to_string()),
        "Z" => Ok("C".to_string()),
        _ => Err("Invalid input!".to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::resolve;

    #[test]
    fn test() -> Result<(), String> {
        let _t = resolve("xx".to_string())?;
        return Ok(());
    }
}
