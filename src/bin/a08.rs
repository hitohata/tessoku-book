use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[u32; w]; h],
        q: usize,
        abcd: [[usize; 4]; q]
    }


    let z: Vec<Vec<u32>> = two_dimension_sum(x, h, w);

    for i in abcd.iter() {
        let [a, b, c, d] = [i[0] - 1, i[1] - 1, i[2] - 1, i[3] - 1];
        let answer = z[c][d] + z[a - 1][b - 1] - z[a - 1][d] - z[c][b - 1];
        println!("{}", answer);
    }


}

fn two_dimension_sum(two_dimension: Vec<Vec<u32>>, size_x: usize, size_y: usize) -> Vec<Vec<u32>> {
    let mut sum: Vec<Vec<u32>> = vec![vec![0; size_x]; size_y];

    for y in 0..size_y {
        for x in 0..size_x {
            if x == 0 {
                sum[y][x] = two_dimension[y][x];
                continue;
            };
            sum[y][x] = sum[y][x - 1] + two_dimension[y][x];
        }
    };

    for x in 0..size_x {
        for y in 0..size_y {
            if y == 0 {
                continue;
            };
            sum[y][x] = sum[y - 1][x] + sum[y][x];
        };
    }

    sum
}
