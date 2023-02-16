use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [[usize; 2]; n],
    }

    let mut su = vec![0_i32; d + 1];

    for i in 0..n {
        su[lr[i][0] - 1] += 1;
        su[lr[i][1]] -= 1;
    }


    let mut answer = vec![0_i32; d + 1];

    for i in 1..=d {
        answer[i] = answer[i - 1] + su[i - 1];
        println!("{}", answer[i]);
    };

}
