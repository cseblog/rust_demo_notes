
struct A {
    id: u32,
    name: String
}

struct B {
    id: u32,
    name: String
}

fn transfer(a_vec: &Vec<A>) -> Vec<B> {
    let mut ret_vec = vec![];
    for item in a_vec.into_iter() {
        let t = B{
            id: item.id,
            name: item.name.clone()
        };
        ret_vec.push(t);
    }
    return ret_vec;
}
fn main() {
    println!("Unit test demo")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transfer_test() {
        let mut a_vec = vec![];
        let a = A {
            id: 10,
            name: "Trung".to_string()
        };
        a_vec.push(a);
        let b_vec = transfer(&a_vec);
        assert_eq!(b_vec[0].id, 10);
        assert_eq!(b_vec[0].name, "Trung".to_string());
    }
}