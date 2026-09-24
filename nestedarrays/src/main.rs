fn transpose_matrix(matrix: [[i32;3];3]) -> [[i32;3];3] {
    let mut result = [[0;3];3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}
fn main() {
    let matrix = [
        [101,102,103],
        [201,202,203],
        [301,302,303],
    ];
    println!("original!");
    for row in matrix {
        println!("{row:?}");
    }
    println!("transposed!");
    let transposed = transpose_matrix(matrix);
    for row in transposed {
        println!("{row:?}");
    }
}
