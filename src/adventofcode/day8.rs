use adventofcode::*;

fn main() {
    let file_str = read_file_as_string("src/adventofcode/data/input_day8.txt".to_string());
    let matrix = process(file_str.unwrap());

    let size = matrix.len();
    println!("size: {}", size);

    //1827
    let (count, total_view_count) = get_visible_tree_count(&matrix);
    println!("total visible tree: {}, -- {}", count, total_view_count);
}

fn get_visible_tree_count(matrix: &Vec<Vec<u32>>) -> (u32, u32) {
    let size = matrix.len();
    let mut count = 0;
    let mut total_view_count: u32 = 0;
    for i in 0..size {
        for j in 0..size {
            let (is_invisible, view_score) = is_invisible(&matrix, i, j);

            if view_score > total_view_count {
                total_view_count = view_score;
            }

            if !is_invisible {
                println!("visible: {}", matrix[i][j]);
                count += 1;
            }
        }
    }
    return (count, total_view_count);
}

fn get_col(matrix: &Vec<Vec<u32>>, j: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for i in 0..matrix.len() {
        v.push(matrix[i][j]);
    }
    return v;
}

fn get_up_visible(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> (bool, u32) {
    let mut is_up_invisible = false;
    let mut view_count = 0;

    let trees_col = get_col(matrix, j);
    for k in (0..i).rev() {
        view_count += 1;
        if trees_col[k] >= matrix[i][j] {
            is_up_invisible = true;
            break;
        }
    }
    return (is_up_invisible, view_count);
}

fn get_down_visible(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> (bool, u32) {
    let mut is_up_invisible = false;
    let mut view_count = 0;

    let trees_col = get_col(matrix, j);

    for k in i + 1..trees_col.len() {
        view_count += 1;
        if trees_col[k] >= matrix[i][j] {
            is_up_invisible = true;
            break;
        }
    }

    return (is_up_invisible, view_count);
}

fn get_left_visible(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> (bool, u32) {
    let mut is_left_invisible = false;
    let mut view_count = 0;
    let trees_row = matrix[i].clone();
    for k in (0..j).rev() {
        view_count += 1;
        if trees_row[k] >= matrix[i][j] {
            is_left_invisible = true;
            break;
        }
    }

    return (is_left_invisible, view_count);
}

fn get_right_visible(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> (bool, u32) {
    let mut is_right_invisible = false;
    let mut view_count = 0;

    let trees_row = matrix[i].clone();
    for k in j + 1..trees_row.len() {
        view_count += 1;
        if trees_row[k] >= matrix[i][j] {
            is_right_invisible = true;
            break;
        }
    }
    return (is_right_invisible, view_count);
}

fn is_invisible(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> (bool, u32) {
    let (up_invisible, up_count) = get_up_visible(&matrix, i, j);
    let (down_invisible, down_count) = get_down_visible(&matrix, i, j);

    let (right_invisible, right_count) = get_right_visible(&matrix, i, j);
    let (left_invisible, left_count) = get_left_visible(&matrix, i, j);

    let is_invisible = down_invisible && up_invisible && left_invisible && right_invisible;
    let count = up_count * down_count * right_count * left_count;
    return (is_invisible, count);
}

fn process(s: String) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let lines = s.lines();

    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        line.trim()
            .chars()
            .into_iter()
            .for_each(|c| row.push(c.to_digit(10).unwrap()));
        matrix.push(row);
    }

    return matrix;
}

#[cfg(test)]
mod test {
    use crate::{
        get_down_visible, get_left_visible, get_right_visible, get_up_visible,
        get_visible_tree_count, process,
    };

    #[test]
    fn test_visible() {
        let s = "30373
        25512
        65332
        33549
        35390";

        let res = process(s.to_string());
        let (count, view_score) = get_visible_tree_count(&res);
        assert_eq!(21, count);
        assert_eq!(8, view_score);
    }

    #[test]
    fn test() {
        let s = "30373
        25512
        65332
        33549
        35390";
        let res = process(s.to_string());

        let (invisible, view_count) = get_up_visible(&res, 2, 3);
        assert_eq!(true, invisible);
        assert_eq!(2, view_count);

        let (invisible, view_count) = get_down_visible(&res, 2, 3);
        assert_eq!(true, invisible);
        assert_eq!(1, view_count);

        let (invisible, view_count) = get_left_visible(&res, 2, 3);
        assert_eq!(true, invisible);
        assert_eq!(1, view_count);

        let (invisible, view_count) = get_right_visible(&res, 2, 3);
        assert_eq!(false, invisible);
        assert_eq!(1, view_count);
    }

    #[test]
    fn test_corner_point() {
        let s = "30373
        25512
        65332
        33549
        35390";

        let res = process(s.to_string());

        let (invisible, view_count) = get_up_visible(&res, 0, 0);
        assert_eq!(false, invisible);
        assert_eq!(0, view_count);

        let (invisible, view_count) = get_down_visible(&res, 0, 0);
        assert_eq!(true, invisible);
        assert_eq!(2, view_count);

        let (invisible, view_count) = get_left_visible(&res, 0, 0);
        assert_eq!(false, invisible);
        assert_eq!(0, view_count);

        let (invisible, view_count) = get_right_visible(&res, 0, 0);
        assert_eq!(true, invisible);
        assert_eq!(2, view_count);
    }
}
