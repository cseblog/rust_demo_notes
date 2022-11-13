use std::sync::{Arc, RwLock};

#[derive(Default)]
struct Config {
    pub debug_mode: bool,
}

impl Config {
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}

fn main() {
    Config { debug_mode: true }.make_current();
    if Config::current().debug_mode {
        println!("Hello....")
    }

    let conf = Config{debug_mode: false};
    
    //
}