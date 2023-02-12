fn main() {
    let mut v = update_vec();
    
    let item = v[0].get(v[0].len()-1).clone();
    v[0].pop();
    v[0].push("sample".to_string());
    println!("{:?}", v[0]);

    //
    (0..0).rev().for_each(|e| {
        println!("{}", e);
    })

}


fn update_vec() -> Vec<Vec<String>> {
    let mut vec: Vec<Vec<String>> = Vec::new();
    for i in 0..9 {
        let mut vec1: Vec<String> = Vec::new();
  vec.push(vec1);
    }

    vec[0].push("hello".to_string());
    vec[0].push("world".to_string());

    let item = vec[0].pop();
    return vec;
}