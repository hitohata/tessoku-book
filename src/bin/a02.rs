use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        a: [u32; n]
    }

    for aa in a {

        if aa == x {
            println!("Yes");
            return;
        };

    }

    println!("No");
}
