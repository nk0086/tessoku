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
    println!("{}", n * n);
}
