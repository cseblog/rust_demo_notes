use adventofcode::*;
use itertools::Itertools;
use substring::Substring;

fn main() {
    println!("hello");
    let s = read_file_as_string("src/adventofcode/data/input_day3.txt".to_string()).unwrap();
    let res = process_part_1(&s).unwrap();
    println!("Result 1: {}", res);

    let res_2 = process_part_2(&s).unwrap();
    println!("Result 2: {}", res_2);
}

fn process_part_2(s: &String) -> Result<u32, String> {
    let mut group_list = Vec::new();
    let groups = s.lines().chunks(3);
    for chunk in groups.into_iter() {
        let st: Vec<String> = chunk.map(|item| item.to_string()).collect();
        let badge = get_common_badge(st)?;
        group_list.push(badge);
    }
    let total = get_total(group_list);
    return Ok(total);
}

fn get_common_badge(s: Vec<String>) -> Result<u8, String> {
    if s.len() != 3 {
        return Err("Invalid".to_string());
    }
    let b = get_common_3parts(&s[0], &s[1], &s[2])?;
    println!("Common badge: {}", b as char);
    return Ok(b);
}

fn process_part_1(s: &String) -> Result<u32, String> {
    let mut char_list = Vec::new();
    for line in s.lines() {
        let part_1 = line.substring(0, line.len() / 2);
        let part_2 = line.substring(line.len() / 2, line.len());

        let result = get_common_char(part_1, part_2)?;

        char_list.push(result);
    }
    let priority_total = get_total(char_list);
    Ok(priority_total)
}

fn get_common_3parts(s1: &str, s2: &str, s3: &str) -> Result<u8, String> {
    let part_a = s1.to_string();
    let part_b = s2.to_string();
    for c in part_a.as_bytes() {
        for k in part_b.as_bytes() {
            if c == k {
                for t in s3.to_string().as_bytes() {
                    if c == t {
                        return Ok(*c);
                    }
                }
            }
        }
    }
    return Err("Failed to get common char".to_string());
}

fn get_common_char(s1: &str, s2: &str) -> Result<u8, String> {
    let part_a = s1.to_string();
    let part_b = s2.to_string();
    for c in part_a.as_bytes() {
        for k in part_b.as_bytes() {
            if c == k {
                return Ok(*c);
            }
        }
    }
    return Err("Failed to get common char".to_string());
}

#[warn(non_snake_case)]
fn get_total(list: Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    let a = 'a' as u32;
    let A = 'A' as u32;
    for c in list {
        if c >= a as u8 {
            sum += (c as u32 - a) + 1;
            println!("{}, sum: {}", c as char, sum);
        } else {
            sum += (c as u32 - A) + 27;
            println!("{}, sum: {}", c as char, sum);
        }
    }
    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_total() {
        let list = vec!['a' as u8];
        assert_eq!(1, get_total(list));
    }

    #[test]
    fn test_get_total_2() {
        let list = vec!['a' as u8, 'z' as u8];
        assert_eq!(27, get_total(list));
    }

    #[test]
    fn test_get_total_3() {
        let list = vec!['A' as u8, 'z' as u8];
        assert_eq!(53, get_total(list));
    }

    #[test]
    fn test_common_badge() {
        let vec = Vec::from([
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ]);
        let r = get_common_badge(vec).unwrap();

        println!("{}", r as char);
        assert_eq!('r', r as char);
    }

    #[test]
    fn test_common_badge_2() {
        let vec = Vec::from([
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ]);
        let r = get_common_badge(vec).unwrap();

        println!("{}", r as char);
        assert_eq!('Z', r as char);
    }

    #[test]
    fn test_common_badge_3() {
        let vec = Vec::from([
            "mjpsHcssDzLTzMsz".to_string(),
            "tFhbtClRVtbhRCGBFntNTrLhqrwqWMDMTWTqMq".to_string(),
            "LltbngLGRSBgSgGRCJdSdQHvdfmQccmjSQ".to_string(),
        ]);
        let r = get_common_badge(vec).unwrap();

        println!("{}", r as char);
        assert_eq!('L', r as char);
    }
}
