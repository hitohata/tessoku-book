use proconio::input;

fn main() {

    input! {
        n: usize,
        x: usize,
        mut a: [usize; n]
    }

    a.insert(0, 0);

    let mut min = 1;
    let mut max = n;

    while min <= max {
        let center = (max + min) / 2;

        if a[center] == x {
            println!("{}", center);
            break;
        }

        if x < a[center] {
            max = center - 1;
        }

        if a[center] < x {
            min = center + 1;
        }

    }

}
