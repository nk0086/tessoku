#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    // input n, output n + n
    input! {
        n: i32,
    }
    println!("{}", add(n));
}

// function name is add and return type is i32
fn add(n: i32) -> i32 {
    n + n
}

//test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(1), 2);
    }

    #[test]
    fn add_test2() {
        assert_eq!(add(5), 10);
    }
}
