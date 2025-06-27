//////////////////////////////////////////////////////////////////////
// For the non-mutable matrix's 
//////////////////////////////////////////////////////////////////////

trait Matrix {
    fn matrix_min(self) -> i32;
    fn matrix_max(self) -> i32;
    fn matrix_median(self) -> i32;
    fn matrix_print(self);
    fn matrix_avg(self) -> f64;
    fn matrix_scale(self , scalar : i32);
}

impl Matrix for &[&[i32]] {
    /// Returns the min value of the matrix
    fn matrix_min(self) -> i32 {
        let mut curr_min = self[0][0];
        for row in self {
            for val in *row {
                if *val < curr_min {
                    curr_min = *val
                }
            }
        }
        return curr_min;
    }

    /// Returns the max value of the matrix
    fn matrix_max(self) -> i32 {
        let mut curr_max = self[0][0];
        for row in self {
            for val in *row {
                if *val > curr_max {
                    curr_max = *val
                }
            }
        }
        return curr_max;
    }

    ///returns the averge of the matrix
    fn matrix_avg(self) -> f64 {
        let mut sum = 0;
        let mut size = 0;
        for row in self {
            for val in *row {
                sum += *val;
                size += 1;
            }
        }
        return sum as f64 / size as f64;
    }

    ///returns the sum of the matrix
    fn matrix_sum(self) -> i32{
        let mut sum : i32 = 0;
        for row in self {
            for val in *row {
                sum += *val;
            }
        }
        return sum;
    }

    //fn matrix_median(matrix)
    fn matrix_median(self) -> i32 {
        //flatten the matrix
        let mut flat : Vec<i32> = Vec::new();
        for row in self {
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

    ///prints the matrix
    fn matrix_print(self) {
        for row in self {
            print!("|");
            for val in *row {
                print!(" {} ",*val)
            }
            println!("|");
        }
    }

    ///returns a new scaled matrix
    fn matrix_scale(self , scalar : i32) {

    }
}

/////////////////////////////////////////////////////////////////////
// Mutable Matrix's 
/////////////////////////////////////////////////////////////////////

impl Matrix for &mut[&mut[i32]] {
    //applies a scalar to the matrix
    fn matrix_scale(self , scalar : i32) {
        for row in self {
            for val in &mut **row {
                *val = *val * scalar
            }
        }
    }

    fn matrix_print(self) {
        for row in self {
            print!("|");
            for val in &mut **row {
                print!(" {} ",*val)
            }
            println!("|");
        }
    }
}
