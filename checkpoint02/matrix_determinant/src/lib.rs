pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {

    let a = matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[2][1] * matrix[1][2]);
    let b = matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[2][0] * matrix[1][2]);
    let c = matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[2][0] * matrix[1][1]);
    
    return a - b + c;
}