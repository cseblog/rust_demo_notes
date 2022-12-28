use adventofcode::*;
use substring::Substring;
use text_io::scan;

fn main() {
    println!("Hello");
    let s = read_file_as_string("src/adventofcode/data/input_day5.txt".to_string()).unwrap();

    let (mut part1, part2) = split(s);
    part1.remove(part1.len() - 1);

    let mut stacks = parse_1(&part1);
    let steps = get_steps(part2);

    // PART 1
    for step in steps.iter() {
        let sta_index = step.source - 1;
        let end_index = step.destination - 1;

        (0..step.count).for_each(|_| {
            let size = stacks[sta_index].len();
            let item = stacks[sta_index][size - 1].clone();
            stacks[sta_index].pop();
            stacks[end_index].push(item);
        });
    }

    // PART 2
    let mut stacks_2 = parse_1(&part1);
    for step in steps.iter() {
        let sta_index = step.source - 1;
        let end_index = step.destination - 1;

        let mut temp_vec = Vec::new();
        (0..step.count).for_each(|_| {
            let size = stacks_2[sta_index].len();
            let item = stacks_2[sta_index][size - 1].clone();
            stacks_2[sta_index].pop();
            temp_vec.push(item);
        });
        temp_vec.reverse();
        stacks_2[end_index].append(&mut temp_vec);
    }

    println!("After: ");
    stack_print(&stacks);

    println!("Top list");
    top_stack_print(&stacks);
    top_stack_print(&stacks_2);
}

#[derive(Debug)]
struct Step {
    count: u32,
    source: usize,
    destination: usize,
}

fn top_stack_print(stacks: &Vec<Vec<String>>) {
    let mut final_str = String::new();
    for s in stacks {
        if s.len() > 0 {
            let item = s[s.len() - 1].clone().substring(1, 2).to_string();
            final_str.push_str(item.as_str());
        }
    }
    println!("{}", final_str);
}

fn parse_1(s: &Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    (0..9).for_each(|_| {
        let vec: Vec<String> = Vec::new();
        result.push(vec);
    });

    for line in s {
        for i in 0..9 {
            let start_index = i * 3 + i;
            let end_index = start_index + 3;
            let st = line.substring(start_index, end_index);

            if !st.to_string().trim().is_empty() {
                result[i].push(st.to_string());
            }
        }
    }

    let mut result2: Vec<Vec<String>> = Vec::new();

    for s in result {
        let mut temp_s = s.clone();
        temp_s.reverse();
        result2.push(temp_s);
    }
    stack_print(&result2);
    return result2;
}

fn stack_print(stacks: &Vec<Vec<String>>) {
    for s in stacks {
        println!("{:?}", s);
    }
}

fn get_steps(list: Vec<String>) -> Vec<Step> {
    let mut steps = Vec::new();
    for step in list {
        let count: u32;
        let source: usize;
        let destination: usize;
        let s = step.as_str();
        scan!(s.bytes() =>"move {} from {} to {}", count, source, destination);
        steps.push(Step {
            count,
            source,
            destination,
        });
    }

    return steps;
}

fn split(s: String) -> (Vec<String>, Vec<String>) {
    let mut part1: Vec<String> = Vec::new();
    let mut part2: Vec<String> = Vec::new();

    for line in s.lines() {
        if line.is_empty() {
            part1 = part2.clone();
            part2.clear();
        } else {
            part2.push(line.to_string());
        }
    }
    return (part1, part2);
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}
