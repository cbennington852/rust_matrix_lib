use matrix_lib::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let row1 = [1, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(1 , matrix_min(matrix));
    }

    #[test]
    fn max_test() {
        let row1 = [1, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(9 , matrix_max(matrix));
    }

    #[test]
    fn avg_test() {
        let row1 = [5, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(5.444444444444445 , matrix_avg(matrix));
    }

    #[test]
    fn sum_test() {
        let row1 = [5, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(49 , matrix_sum(matrix));
    }

    #[test]
    fn median_test() {
        let row1 = [5, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(5 , matrix_median(matrix));
        matrix_print(matrix);
    }

    #[test]
    fn scalar_test() {
        let mut row1 = [2, 2, 2];
        let mut row2 = [3, 3, 3];
        let mut row3 = [5, 5, 5];

        let matrix: &mut[&mut[i32]] = &mut[&mut row1, &mut row2, &mut row3];


        let mut res1 = [4, 4, 4];
        let mut res2 = [6, 6, 6];
        let mut res3 = [10, 10, 10];

        let res: &mut[&mut[i32]] = &mut[&mut res1, &mut res2, &mut res3];

        assert_eq!(1 , 1);
    }
}
