use proconio::input;

fn main() {

    input! {
        n: usize,
        q: u32,
        a: [u32; n],
        qe: [[usize; 2]; q]
    }

    let mut su = vec![0; n + 1];

    for (i, val) in a.iter().enumerate() {
        if i == 0 {
            su[i + 1] = val.to_owned();
        } else {
            su[i + 1] = su[i] + val.to_owned();
        }
    }

    for question in qe {
        let start_form = question[0];
        let end_to = question[1];
        println!("{}", (su[end_to] - su[start_form - 1]));
    }

}
