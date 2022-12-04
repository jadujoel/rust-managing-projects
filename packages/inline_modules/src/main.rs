fn main() {
    println!("Hello, world!");
}

mod hello {
    pub fn english() {
        println!("Hello, world!");
    }
    pub fn swedish() {
        println!("Dra åt helvete din gamle skojare");
    }
    mod casual {
        pub fn english() {
            println!("what's up?");
        }
        pub fn swedish() {
            println!("ah tja");
        }
    }
}
