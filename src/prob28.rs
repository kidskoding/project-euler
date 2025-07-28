pub fn prob28() -> i32 {
    let spiral = generate_center_outward_spiral(1001);
    let (mut x, mut y) = (0i32, 1000i32);
    let mut sum = 0;
    let mut subtract = false;

    for row in spiral {
        if x != y {
            sum += row[x as usize] + row[y as usize];
        } else {
            sum += row[x as usize];
            subtract = false;
        }

        if subtract {
            x -= 1;
            y += 1;
        } else {
            x += 1;
            y -= 1;
        }
    }

    sum as i32
}

fn generate_center_outward_spiral(n: usize) -> Vec<Vec<usize>> {
    let mut spiral = vec![vec![0; n]; n];
    let (mut x, mut y) = (n / 2, n / 2);
    let mut num: usize = 1;
    let mut step = 1;

    spiral[y][x] = num;
    num += 1;

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while num <= n * n {
        for (dx, dy) in directions.iter().cycle() {
            for _ in 0..step {
                if num > n * n {
                    break;
                }
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;
                spiral[y][x] = num;
                num += 1;
            }

            if dx == &0 {
                step += 1;
            }
            if num > n * n {
                break;
            }
        }
    }

    spiral
}
