#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars;h],
    }
    let mut ans = 0;
    for maskrow in 0..(1 << h) {
        for maskcol in 0..(1 << w) {
            let mut black = 0;
            for i in 0..h {
                for j in 0..w {
                    if ((maskrow >> i) & 1) == 0 && ((maskcol >> j) & 1) == 0 && c[i][j] == '#' {
                        black += 1;
                    }
                }
            }
            if black == k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
