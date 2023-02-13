fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            transposed[col][row] = matrix[row][col];            
        }   
    }
    return transposed;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    println!("{matrix:#?}")
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

// ANCHOR: tests
#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    let expected = [
        [101, 201, 301],
        [102, 202, 302],
        [103, 203, 303],
    ];
    let trasposed = transpose(matrix);
    assert_eq!(trasposed, expected);
}