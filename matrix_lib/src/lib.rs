use noise::{NoiseFn, Perlin, Seedable};

//////////////////////////////////////////////////////////////////////
// For the non-mutable matrix's 
//////////////////////////////////////////////////////////////////////
pub trait Matrix {
    fn matrix_min(&self) -> i32;
    fn matrix_max(&self) -> i32;
    fn matrix_median(&self) -> i32;
    fn matrix_sum(&self) -> i32;
    fn matrix_print(&self);
    fn matrix_avg(&self) -> f64;
    fn matrix_scale(&mut self , scalar : i32);
    fn matrix_multiply(&self, matrix2: Vec<Vec<i32>>);
    fn matrix_reduce(&self);
    fn matrix_equal(&self, matrix2: Vec<Vec<i32>>) -> bool;
    fn matrix_noise(seed: i32) -> Vec<Vec<i32>>;
    fn matrix_init(cols : i32 , rows: i32, starting_val : i32) -> Vec<Vec<i32>>;
}


impl Matrix for Vec<Vec<i32>> {
    fn matrix_min(&self) -> i32 {
        let mut min : i32 = self[0][0];
        for row in self {
            for &val in row {
                if val < min {
                    min = val;
                }
            }
        }
        min
    }

    fn matrix_max(&self) -> i32 {
        let mut max : i32 = self[0][0];
        for row in self {
            for &val in row {
                if val > max {
                    max = val;
                }
            }
        }
        max
    }

    fn matrix_median(&self) -> i32 {
        //flatten 
        let mut lst : Vec<i32> = vec![];
        for row in self {
            for &val in row {
                lst.push(val);
            }
        }

        //sort
        lst.sort();

        //get median
        return lst[lst.len()/2];
    }

    fn matrix_sum(&self) -> i32 {
        let mut sum : i32 = self[0][0];
        for row in self {
            for &val in row {
                sum += val;
            }
        }
        sum
    }

    fn matrix_print(&self) {
        for row in self {
            print!("|");
            for &val in row {
                print!(" {} " , val);
            }
            println!("|");
        }
    }

    fn matrix_avg(&self) -> f64 {
        let mut sum = 0;
        let mut count = 0;
        for row in self {
            for &val in row {
                count += 1;
                sum += val;
            }
        }
        return (sum / count).into();
    }

    /// Checks tp make sure that two matrix's are equal to each other. 
    fn matrix_equal(&self, matrix2: Vec<Vec<i32>>) -> bool {
        if self.len() != matrix2.len() {
            //println!("ISSUE: not rows {} != {}", self.len(), matrix2.len());
            return false;
        }
        for x in 0..self.len() {
            if self.get(x).unwrap().len() != matrix2.get(x).unwrap().len() {
                //println!("ISSUE: not rows {} != {}", self.len(), matrix2.get(x).unwrap().len());
                return false
            }
            for y in 0..self.get(x).unwrap().len() {
                //println!("{x} , {y}");
                if self.get(x).unwrap().get(y) != matrix2.get(x).unwrap().get(y) {
                    //println!("ISSUE: not equal ");
                    return false
                }
            }
        }
        return true
    }

    ///Multiplies each and every element in a matrix by a scalar. 
    fn matrix_scale(&mut self , scalar : i32) {
        for row in self {
            for mut val in row {
                *val = *val * scalar;
            }
        }
    }

    ///
    fn matrix_multiply(&self, matrix2: Vec<Vec<i32>>) {
        todo!()
    }
    
    fn matrix_reduce(&self) {
        todo!()
    }
    
    fn matrix_noise(seed: i32) -> Vec<Vec<i32>> {
        todo!()
    }
    
    ///makes a new matrix with specified cols and rows 
    fn matrix_init(cols : i32 , rows: i32, starting_val : i32) -> Vec<Vec<i32>> {
        if cols <= 0 || rows <= 0 {
            panic!("{cols} and {rows} is not a valid matrix size");
        }
        let mut matrix : Vec<Vec<i32>> = vec![];
        for x in 1..cols {
            let mut row : Vec<i32> = vec![starting_val; rows.try_into().unwrap()];
            matrix.push(row);
        }
        return matrix
    }

    
}


/* 
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
    fn matrix_scale(self , _scalar : i32) {
        panic!("Matrix immutable ")
    }

    ///multiple two matrix's against each other...
    fn matrix_multiply(self, matrix2 : &[&[i32]]) {
        let fst_num_cols = self.len(); 
        let fst_num_rows = self[0].len();
        let snd_num_cols = matrix2.len();
        let snd_num_rows = matrix2[0].len();

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

    fn matrix_min(self) -> i32 {
        let mut curr_min = self[0][0];
        for row in self {
            for val in &mut **row {
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
            for val in &mut **row {
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
            for val in &mut **row {
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
            for val in &mut **row {
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
            for val in &mut **row {
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
    
    fn matrix_multiply(self, matrix2: &[&[i32]]) -> &[&[i32]] {
        todo!()
    }
    */
