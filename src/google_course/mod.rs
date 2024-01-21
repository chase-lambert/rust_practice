#![allow(unused_variables, dead_code)]

// mod ffi;
mod gui;
mod health;
mod library;
mod luhn;
mod polygon;
mod prefix;

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for x in 0..matrix.len() {
        for y in 0..matrix.len() {
            result[x][y] = matrix[y][x];
        }
    }

    result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    // println!("{matrix:#?}");
    for row in matrix {
        println!("{row:?}");
    }
}

pub fn run() {
    // let matrix = [
    //     [101, 102, 103], // <-- the comment makes rustfmt add a newline
    //     [201, 202, 203],
    //     [301, 302, 303],
    // ];

    // println!("matrix:");
    // pretty_print(&matrix);

    // let transposed = transpose(matrix);
    // println!("transposed:");
    // pretty_print(&transposed);
    library::run();
    // health::run();
    // polygon::run();
    // luhn::luhn("4263 9826 4026 9299");
    // gui::run();
    // ffi::run();
}
