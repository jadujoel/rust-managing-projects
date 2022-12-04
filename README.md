# rust-managing-projects
Manage rust projects course

Rust Module System Elements
* Modules
* Paths
* Crates
* Packages

cargo new my-package
* generates binary crate
* generates main.rs
cargo new my-package --lib
* generates library crate
* generates lib.rs

bin dir in src
* expects rust expects main function in those .rs files


my-project
    * Cargo.toml
    * src/
        * lib.rs // root library
        * main.rs // first executable
        * bin/
            * second_binary.rs // second executable
            * third_binary/
                * main.rs // third executable
                * another_module.rs
    * benches/ // hold benchmarking code
    * examples/ // hold example code
    * tests/ // hold integration tests


```bash
cargo build --workspace
```

Inline Modules

Absolute path
crate::hello::english

Relative path
self::english
super::hello::english

When enum is made public
all it's member are also made public


## Tests
#[test]
#[should_panic] // expect the test to panic
#[ignore] // dont run this test
#[cfg(test)] // will only compile when you run the test command
assert_eq!
assert_ne!

```bash
cargo test -- --show-output --test-threads=1
```

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_add_two() {
        assert_eq!(add_two(1,2), 3);
    }
}

```
