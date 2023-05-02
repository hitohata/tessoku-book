use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [[usize; 4]; q]
    }


    let z: Vec<Vec<usize>> = two_dimension_sum(x, w, h);

    for i in abcd.iter() {
        let [a, b, c, d]: [usize; 4] = [i[0], i[1], i[2], i[3]];
        let answer = z[c][d] + z[a - 1][b - 1] - z[c][b - 1] - z[a - 1][d];
        println!("{}", answer);
    }


}

fn two_dimension_sum(two_dimension: Vec<Vec<usize>>, size_x: usize, size_y: usize) -> Vec<Vec<usize>> {
    let mut sum: Vec<Vec<usize>> = vec![vec![0; size_x + 1]; size_y + 1];

    for y in 1..=size_y {
        for x in 1..=size_x {

            if x == 0 {
                sum[y][x] = 0;
                continue;
            }

            sum[y][x] = sum[y][x - 1] + two_dimension[y - 1][x - 1];
        }
    }

    for y in 1..=size_y {
        for x in 1..=size_x {

            if x == 0 {
                continue;
            }

            sum[y][x] += sum[y - 1][x];

        }
    }

    sum
}
