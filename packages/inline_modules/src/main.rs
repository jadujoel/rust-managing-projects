fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod hello {
    #[test]
    pub fn english() {
        println!("Hello, world!");
    }
    #[test]
    pub fn swedish() {
        println!("Dra åt helvete din gamle skojare");
    }
    mod casual {
        #[test]
        pub fn english() {
            println!("what's up?");
        }
        #[test]
        pub fn swedish() {
            println!("ah tja");
        }
    }
}
