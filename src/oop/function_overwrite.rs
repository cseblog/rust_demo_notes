struct Character {

}

trait Human {
    fn learn(&self) -> String;
    fn fight(&self) -> u32;
}


trait Orc {
    fn fight(&self) ->f32 {
        return 12.33;
    }    
}

// trait HalfHumanHalfOrc {
//     fn learn(&self) -> String {
//         return String::from("Human")
//     }

//     fn fight(&self) ->f32 {
//         return 12.33;
//     }   
// }

impl Orc for Character {
    fn fight(&self) ->f32 {
        println!("Orc can fight!");
        return 12.33;
    }   
}

impl Human for Character {
    fn learn(&self) -> String {
        println!("Human can learn!");
        return String::from("Human")
    }

    fn fight(&self) -> u32 {
        println!("Human can fight!");
        return 122;
    }
}

fn main() {
    let orc = Character{};
    Human::fight(&orc);
    Orc::fight(&orc);
    orc.learn();
}