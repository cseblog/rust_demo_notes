use regex::Regex; 
use substring::Substring;

fn add_str(str1: &str, str2: &str) {
    let str3 = str1.to_owned() + str2;
    println!("{}", str3)
}

fn main() {
    //&str: static by default, it is 'literal string'
    let str1 = "abc";
    let str2 = "yxz";
    let str3 = str1.to_owned() + str2;
    println!("{}", str3);

    //Convert literal string -> String
    let str4: String = str1.to_string();
    println!("{}", str4);

    //String box type
    let mut sbox = String::from("ABCD");
    let sbox2 = String::from("XYZ");
    // concate a &str string into String
    sbox.push_str("EFHG");
    println!("sbox: {}", sbox);

    //+ two String
    let sbox3 = sbox + &sbox2;
    println!("sbox3: {}", sbox3);
    add_str("trung", "andrew");

    //Sub string
    let str = String::from("Hello world!");
    println!("{}", str.substring(7, 12));

    // Format string
    println!("{}", format!("Hello, {}!", "world"));
    println!("{}", format!("{:?}", (3, 4)));

    //Regex
    let reg = Regex::new("You.* PayNow").unwrap();
    let test_string = String::from(
        "Dear Sir / Madam,<br /><br /><br /> You have received SGD 14.40 on 01 Jun 11:37 (SGT) from DINH QUANG HIEU to your account via PayNow. <br /><br /> Thank you for banking with us.<br"
    );
    let mat = reg.find(&*test_string).unwrap();
    println!("{}", mat.as_str())
}