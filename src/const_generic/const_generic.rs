use std::fmt::Debug;
#[derive(Debug)]
struct Array<T, const N: usize> {
    data : [T; N]
}

fn update<T: Debug, const N: usize>(arr: Array<T, N>) {
    println!("{:?}", arr)
}


fn main() {
    let array_1 = Array{
            data: [1, 2, 3],
    };

    update(array_1);
    let array_2 = Array{
            data: [1, 2, 3, 4],
    };
    

    update(array_2);
    let array_3 = Array{
            data: [1.3, 1.2, 3.0, 4.0],
    };

    update(array_3);

}