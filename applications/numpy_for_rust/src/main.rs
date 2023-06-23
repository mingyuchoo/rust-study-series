use ndarray::prelude::{Array, Array1, ArrayBase, Dim, array };
use ndarray::{ OwnedRepr, ShapeError};
use std::f64::consts::PI;

fn main() -> Result<(), ShapeError> {
    let a: Array1<f64> = array![0., 30., 45., 60., 90.];

    println!("angles {}", a);
    println!("sine(a) {}", (a * PI / 180_f64).map(|x| x.sin()));

    let a: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec())?;
    let b: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> = array![10., 10., 10.];

    println!("a: {}", &a);
    println!("b: {}", &b);
    println!("a * 2 {}", &a * 2.);
    println!("a + b {}", &a + &b);
    println!("a * b {}", &a * &b);
    println!("average(a) {}", a.sum() / a.len() as f64);
    println!("mean(b) {}", b.mean().unwrap());
    Ok(())
}
