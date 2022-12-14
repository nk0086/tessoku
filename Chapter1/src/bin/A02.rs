#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, x: usize,
        a: [usize; n],
    }

    if find(a, x) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn find(a: Vec<usize>, x: usize) -> bool {
    for i in 0..a.len() {
        if a[i] == x {
            return true;
        }
    }
    return false;
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_test() {
        assert_eq!(find(vec![1, 2, 3], 1), true);
    }

    #[test]
    fn find_test2() {
        assert_eq!(find(vec![1, 2, 3], 5), false);
    }
}
