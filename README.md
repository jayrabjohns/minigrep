# Minigrep
Building a watered down version of grep with the goal of learning more about rust along the way.

Based on chapter 12 of [the rust book](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

# Building localy
```bash
cargo build
```

# Running localy
```
Usage: minigrep [OPTIONS] <PATTERN> <FILE_PATH>

Arguments:
  <PATTERN>    
  <FILE_PATH>  

Options:
  -i, --ignore-case  Ignore case
  -h, --help         Print help
```