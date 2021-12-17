use std::cmp;

fn main() {
    println!("test");
    println!("max height: {}", max_height(20, 30, -10, -5));
    println!("real");
    // target area: x=269..292, y=-68..-44
    println!("max height: {}", max_height(269, 292, -68, -44));
}

fn max_height(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> isize {
    let mut max_height = 0;
    let mut candidates = 0;
    for init_x_velo in 1..(x_max+1) {
        for init_y_velo in y_min..1000 {
            let mut this_max_height = 0;
            let mut valid_parameters = false;

            let mut x_location = 0;
            let mut y_location = 0;

            let mut x_velo = init_x_velo;
            let mut y_velo = init_y_velo;

            while x_location <= x_max && y_location >= y_min {
                if x_min <= x_location && y_location <= y_max {
                    valid_parameters = true;
                }

                this_max_height = cmp::max(y_location, this_max_height);

                x_location += x_velo;
                y_location += y_velo;

                x_velo = cmp::max(0, x_velo - 1);
                y_velo -= 1;
            }

            if valid_parameters {
                max_height = cmp::max(max_height, this_max_height);
                candidates += 1;
            }
        }
    }
    println!("possible velos: {}", candidates);
    return max_height;
}
