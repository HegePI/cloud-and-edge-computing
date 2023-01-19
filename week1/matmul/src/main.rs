use std::time::Instant;

use ndarray::Array;
use ndarray_einsum_beta::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
fn main() {
    let now = Instant::now();
    println!("Hello");
    let x = 100000;
    let y = 1000;
    let z = 1;
    let m1 = Array::random((x, y), Uniform::new(0., 1.));
    let m2 = Array::random((y, x), Uniform::new(0., 1.));
    let m3 = Array::random((x, z), Uniform::new(0., 1.));

    let res1 = einsum("ij,jk->ik", &[&m1, &m2]).unwrap();
    let _res2 = einsum("ij,jk->ik", &[&res1, &m3]).unwrap();

    let elapsed_time = now.elapsed();
    println!("Running took {} seconds", elapsed_time.as_secs());
}
