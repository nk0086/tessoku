#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    // input a, b
    input! {
        a: i32, b: i32,
    }

    if divide(a, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn divisor(a: i32, b: i32) -> bool {
    for i in a..=b {
        if i % 100 == 0 {
            return true;
        }
    }

    return false;
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisor_test() {
        assert_eq!(divisor(101, 200), true);
    }

    #[test]
    fn divisor_test2() {
        assert_eq!(divisor(101, 199), false);
    }
}
