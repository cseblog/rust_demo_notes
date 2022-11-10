fn update_2d_array(arr: &mut [[u32; 10]; 10]) {
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j] = 10;
        }
    }
}

fn main() {
    let mut array_2d = [[0;10]; 10];
    println!("Before: {:?}", array_2d);

    update_2d_array(&mut array_2d);
    println!("After: {:?}", array_2d);

}