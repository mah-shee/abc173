#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,

    }
    println!("{}", (1000 - (n % 1000)) % 1000);
}
