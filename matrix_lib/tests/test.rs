use matrix_lib::*;


#[cfg(test)]
mod tests {
    use super::*;

    ///////////////////////////////////////////////////////////////////////////
    // TEST VECTORS
    //////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_min() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(1 , matrix.matrix_min());
    }

    #[test]
    fn test_max() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(9 , matrix.matrix_max());
    }

    #[test]
    fn test_print() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        matrix.matrix_print();
        assert_eq!(true, true);
    }

    #[test]
    fn test_sum() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(5.0 , matrix.matrix_avg());
    }

    #[test]
    fn test_equal() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(true , matrix.matrix_equal(matrix2));
    }

    #[test]
    fn test_equal2() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 4, 6],
            vec![7, 8, 9],
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(false , matrix.matrix_equal(matrix2));
    }

    #[test]
    fn test_equal3() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 4, 6],
            vec![7, 8, 9],
            vec![5, 5, 6],
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(false , matrix.matrix_equal(matrix2));
    }
    #[test]
    fn test_equal4() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 4, 6 , 7 , 8],
            vec![5, 5, 6],
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(false , matrix.matrix_equal(matrix2));
    }

   

    #[test]
    fn test_equal5() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1]
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![1]
        ];
        assert_eq!(true , matrix.matrix_equal(matrix2));
    }

    #[test]
    fn test_equal6() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![]
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![]
        ];
        assert_eq!(true , matrix.matrix_equal(matrix2));
    }

     #[test]
    fn test_equal7() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 4, 6],
            vec![5, 5, 6],
            vec![7, 8, 9],
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 4, 6],
            vec![5, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(true , matrix.matrix_equal(matrix2));
    }

    
    #[test]
    fn test_scale() {
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![1, 2, 2],
            vec![1, 2, 2],
            vec![1, 2, 2],
        ];
         let matrix2: Vec<Vec<i32>> = vec![
            vec![2, 4, 4],
            vec![2, 4, 4],
            vec![2, 4, 4],
        ];
        matrix.matrix_scale(2);
        assert_eq!(true , matrix.matrix_equal(matrix2));
    }
    

    /* 

    #[test]
    fn it_works() {
        let row1 = [1, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(1 , matrix.matrix_min());
    }

    
    #[test]
    fn max_test() {
        let row1 = [1, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(9 , matrix.matrix_max());
    }

    #[test]
    fn avg_test() {
        let row1 = [5, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(5.444444444444445 , matrix.matrix_avg());
    }

    #[test]
    fn sum_test() {
        let row1 = [5, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(49 , matrix.matrix_sum());
    }

    #[test]
    fn median_test() {
        let row1 = [5, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];

        let matrix: &[&[i32]] = &[&row1, &row2, &row3];
        assert_eq!(5 , matrix.matrix_median());
        matrix.matrix_print();
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
    */
}
