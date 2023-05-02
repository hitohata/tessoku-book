use proconio::input;

fn main() {

    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.insert(0, 0);

    let mut r = vec![0; n + 1];

    for i in 1..n {
        if i == 1 {
            r[i] = 1;
        } else {
            r[i] = r[i - 1];
        }

        while (r[i] < n) && (a[r[i] + 1] - a[i] <= k) {
            r[i] += 1;
        }
    }

    let mut answer = 0_usize;

    for i in 1..n {
        answer += r[i] - i
    }

    println!("{:?}", answer);

}
