use adventofcode::*;

fn main() {
    println!("Hello");
    let file_content_str = read_file_as_string("src/adventofcode/data/input_day6.txt".to_string());

    let pos = get_marker_char_position(file_content_str.clone().unwrap(), 4);
    println!("Position: {}", pos);

    let pos_2 = get_marker_char_position(file_content_str.unwrap(), 14);
    println!("Position: {}", pos_2);
}

fn get_marker_char_position(s: String, limit: usize) -> u32 {
    let mut count = 0;
    let mut byte_arr: Vec<u8> = Vec::new();

    for b in s.as_bytes() {
        count += 1;
        if !byte_arr.contains(b) {
            byte_arr.push(b.clone());
            if byte_arr.len() == limit {
                return count;
            }
        } else {
            let slice_index_element = byte_arr.iter().position(|&x| x == *b).unwrap();
            byte_arr.drain(0..=slice_index_element);
            byte_arr.push(b.clone());
            if byte_arr.len() == limit {
                return count;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use crate::get_marker_char_position;

    #[test]
    fn test() {
        let test_str = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(get_marker_char_position(test_str, 4), 5);
    }

    #[test]
    fn test_2() {
        let test_str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(get_marker_char_position(test_str, 4), 10);
    }

    #[test]
    fn test_3() {
        let test_str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(get_marker_char_position(test_str, 4), 11);
    }

    #[test]
    fn test_4() {
        let test_str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(get_marker_char_position(test_str, 14), 29);
    }
}
