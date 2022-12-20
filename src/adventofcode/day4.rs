use adventofcode::*;

fn main() {
    println!("hello");
    let s = read_file_as_string("src/adventofcode/data/input_day4.txt".to_string()).unwrap();
    let res = process(&s);
    println!("Result: {}", res.unwrap());

    let res2 = process_part_2(&s);
    println!("Result: {}", res2.unwrap());


}

#[derive(Default)]
struct Range {
    l: u32,
    r: u32
}


impl Range {
    fn is_full_contain(&self, other: &Range) -> bool {
        if self.l <= other.l && self.r >= other.r {
            return true;
        }
        return false;
    }
    
    fn is_overlap(&self, other: &Range) -> bool {
        if other.l <= self.r && other.r >= self.r {
            return true;
        } else {
            if other.l <= self.l && other.r >= self.l {
                return true;
            } else {
                return false;
            }
        }
    }
}

fn process_part_2(s: &String) -> Result<u32, String> {

    let mut count = 0;
    for line in s.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let mut range_list = Vec::new();
        for r in ranges {
            let pair: Vec<&str> = r.split("-").collect();
            range_list.push(Range{l: pair[0].parse::<u32>().unwrap_or(0), r: pair[1].parse::<u32>().unwrap_or(0)});
        }
        if range_list[0].is_overlap(&range_list[1]) {
            // println!("r1: {}", range_list);
            count += 1;
        } else {
            if range_list[1].is_overlap(&range_list[0]) {
                count += 1;
            }
        }

   }
    
    return Ok(count);
}

fn process(s: &String) -> Result<u32, String> {

    let mut count = 0;
    for line in s.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let mut range_list = Vec::new();
        for r in ranges {
            let pair: Vec<&str> = r.split("-").collect();
            range_list.push(Range{l: pair[0].parse::<u32>().unwrap_or(0), r: pair[1].parse::<u32>().unwrap_or(0)});
        }
        if range_list[0].is_full_contain(&range_list[1]) {
            // println!("r1: {}", range_list);
            count += 1;
        } else {
            if range_list[1].is_full_contain(&range_list[0]) {
                count += 1;
            }
        }

   }
    
    return Ok(count);
}
