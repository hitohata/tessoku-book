use proconio::input;

fn main() {

    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [[usize; 4]; n]
    }

    let mut answer = vec![vec![0_isize; w + 1]; h + 1];


    for point in abcd.iter() {
        marking(&mut answer, point.to_vec());
    }

    cumulative_sum(&mut answer, h, w);

    print_answer(&answer, h, w);

}

fn cumulative_sum(grid: &mut Vec<Vec<isize>>, y_size: usize, x_size: usize) {

    for y in 0..y_size {
        for x in 0..x_size {
            if x == 0 {
                continue;
            }
            grid[y][x] += grid[y][x - 1]
        }
    }

    for y in 0..y_size {
        for x in 0..x_size {
            if y == 0 {
                continue;
            }
            grid[y][x] += grid[y - 1][x]
        }
    }

}

fn print_answer(grid: &Vec<Vec<isize>>, y_size: usize, x_size: usize) {
    for y in 0..y_size {
        for x in 0..x_size {
            print!("{} ", grid[y][x]);
            if x == x_size - 1 {
                print!("\n");
            }
        }
    }
}

fn marking(grid: &mut Vec<Vec<isize>>, point: Vec<usize>) {
    let min_y = point[0] - 1;
    let min_x = point[1] - 1;
    let max_y = point[2] - 1;
    let max_x = point[3] - 1;

    grid[min_y][min_x] += 1;
    grid[max_y + 1][max_x + 1] += 1;
    grid[min_y][max_x + 1] -= 1;
    grid[max_y + 1][min_x] -= 1;
}
