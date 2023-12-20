use linalg::Matrix;
fn main() {
    let m: Matrix = Matrix::new(3, 3);
    println!("{:?}", m);

    let m1: Matrix = Matrix::from_file("src/2b2.txt");
    println!("{:?}", m1);

    let m2: Matrix = Matrix::from_string("1 2 3; 4 5 6; 7 8 9");
    println!("{:?}", m2);
}

