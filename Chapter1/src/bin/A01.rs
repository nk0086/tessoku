#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
// input n output n * n
fn main() {
    input! {
        n: usize,
    }
    println!("{}", multipul(n));
}

fn multipul(n: usize) -> usize {
    n * n
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(multipul(1), 1);
    }

    #[test]
    fn simple2() {
        assert_eq!(multipul(5), 25);
    }
}
