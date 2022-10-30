
struct Dog {
    name: String,
    old: u32
}
fn main() {
    let mut alaska = Dog{name: String::from("Alaska"), old: 10 };
    let mut nakita = alaska;
    nakita.name = "Nakita".to_string();
    alaska.name = String::from("Alaska Junior");
    println!("a: {}, b: {}", nakita.name, nakita.old);
}