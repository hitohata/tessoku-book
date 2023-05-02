use proconio::input;

fn main() {

    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.insert(0, 0);

    let aa = Box::new(a.to_vec());

    let mut left = 0;
    let mut right = 1_000_000;

    while left < right {

        let center = (left + right) / 2;

        if check(&center, n, k, &aa) {
            right = center;
        } else {
            left = center + 1;
        }

    }

    println!("{}", left);

}

fn check(time: &usize, n: usize, target: usize, a: &Box<Vec<usize>>) ->  bool {

        let mut sum = 0;

        for i in 1_usize..=n {
            sum += time / a[i];
        }

        if sum >= target {
            true
        } else {
            false
        }

    }
