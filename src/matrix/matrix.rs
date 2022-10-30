fn update2_2d<V: AsMut<[Row]>, Row: AsMut<[i32]>>(matrix: &mut V) {
    for row in matrix.as_mut() {
        for cell in row.as_mut() {
            *cell = 100;
        }
    }
}

fn update_2d<V: AsMut<[i32]>>(matrix: &mut [V]) {
    for i in 00..matrix.len() {
        let k = matrix[i].as_mut();
        for j in 00..k.len() {
            matrix[i].as_mut()[j] = 100;
        }
    }
}

// fn function_3d<V: AsMut<[i32]>, P: AsMut<[V]>>(array: &mut [P]) {
//     array[1].as_mut()[2].as_mut()[0] = 1;
// }

fn update_arr(arr: &mut [i32; 4]) {
    arr[0] = 100;
}

fn main() {
    let mut zbuffer = [[0; 20]; 20];

    println!("before: {:?}", zbuffer);
    update_2d(&mut zbuffer);
    println!("After: {:?}", zbuffer);

    let mut arr = [3, 4, 4, 10];
    println!("1d array: {:?}", arr);
    update_arr(&mut arr);
    println!("1d array: {:?}", arr);
}