
/// Returns the min value of the matrix
pub fn matrix_min(matrix : &[&[i32]]) -> i32 {
    let mut curr_min = matrix[0][0];
    for row in matrix {
        for val in *row {
            if *val < curr_min {
                curr_min = *val
            }
        }
    }
    return curr_min;
}

/// Returns the max value of the matrix
pub fn matrix_max(matrix : &[&[i32]]) -> i32 {
    let mut curr_max = matrix[0][0];
    for row in matrix {
        for val in *row {
            if *val > curr_max {
                curr_max = *val
            }
        }
    }
    return curr_max;
}

///returns the averge of the matrix
pub fn matrix_avg(matrix : &[&[i32]]) -> f64 {
    let mut sum = 0;
    let mut size = 0;
    for row in matrix {
        for val in *row {
            sum += *val;
            size += 1;
        }
    }
    return sum as f64 / size as f64;
}

///returns the sum of the matrix
pub fn matrix_sum(matrix: &[&[i32]]) -> i32{
    let mut sum : i32 = 0;
    for row in matrix {
        for val in *row {
            sum += *val;
        }
    }
    return sum;
}

//pub fn matrix_median(matrix)
pub fn matrix_median(matrix: &[&[i32]]) -> i32 {
    //flatten the matrix
    let mut flat : Vec<i32> = Vec::new();
    for row in matrix {
        for val in *row {
            flat.push(*val);
        }
    }

    //sort the flattened matrix
    flat.sort();

    //return the median
    let result = flat.get(flat.len() / 2);
    match result {
        Some(result) => return *result,
        None => panic!("Unexpected error"),
    }
}

//applies a scalar to the matrix
pub fn matrix_scale(matrix: &mut[&mut[i32]] , scalar : i32) {
    for row in matrix {
        for val in &mut **row {
            *val = *val * scalar
        }
    }
}

