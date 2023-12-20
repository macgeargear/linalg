use linalg::Matrix;
fn main() {
    let m: Matrix = Matrix::new(3, 3);
    println!("{:?}", m);
    let m1: Matrix = Matrix::from_file("src/2b2.txt");
    println!("{:#?}", m1);
}

