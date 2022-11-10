fn update_2_2d<V: AsMut<[Row]>, Row: AsMut<[i32]>>(matrix: &mut V) {
    for row in matrix.as_mut() {
        for cell in row.as_mut() {
            *cell = 100; //unsafe action - no recommendation
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

//Using Vector
fn update_2d_by_vec(array: &mut Vec<Vec<u32>>) {
    for i in 0..array.len() {
        for j in 0..array[i].len() {
            array[i][j] = 200;
        }
    }
}

//Using Constant Generic
fn update_const_generic<const N: usize>(mat: &mut [[i32; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            mat[i][j] += 10;
        }
    }
}

fn main() {
    let mut zbuffer = [[0; 20]; 20];
    println!("before: {:?}", zbuffer);
    
    update_2d(&mut zbuffer);
    println!("After: {:?}", zbuffer);

    let mut array_2d_as_vec = vec![vec![0;20];20];
    update_2d_by_vec(&mut array_2d_as_vec);
    println!("{:?}", array_2d_as_vec);

    update_const_generic(&mut zbuffer); 
    println!("{:?}", zbuffer);

}