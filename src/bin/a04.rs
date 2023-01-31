use proconio::input;

fn main() {
    input! {
        mut n: u32
    };
    
    const DIGIT: u32 = 10;

    for i in (0..DIGIT).rev() {

        let nn = n / 2_u32.pow(i);

        print!("{}", nn % 2);
    }
    println!();
}
