use proconio::input;

fn main() {

    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        range: [[usize; 2]; d]
    }

    let mut cumulative_max_from_left: Vec<usize> = vec![0; n];
    let mut cumulative_max_from_right: Vec<usize> = vec![0; n];

    for i in 0..n {

        if i == 0 {
            cumulative_max_from_left[0] = a[0];
            continue;
        }

        cumulative_max_from_left[i] = std::cmp::max(a[i], cumulative_max_from_left[i - 1]);
    }


    for i in (0..n).rev() {

        if i == (n - 1) {
            cumulative_max_from_right[i] = a[i];
            continue;
        }

        cumulative_max_from_right[i] = std::cmp::max(a[i], cumulative_max_from_right[i + 1]);
    }

    for i in range.iter() {

        let left = cumulative_max_from_left[0..i[0] - 1].iter().max().unwrap();
        let right = cumulative_max_from_right[i[1]..n].iter().max().unwrap();

        println!("{}", std::cmp::max(left, right))
    }


}
