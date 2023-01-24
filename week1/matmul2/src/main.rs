use rand::distributions::Uniform;
use rand::{self, Rng};
use std::time::Instant;

fn main() {
    fn mat_mul(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let x = a.len();
        let y = a[0].len();
        let z = b[0].len();
        let mut ab = vec![vec![0.0; x]; x];
        for i in 0..x {
            for j in 0..z {
                for k in 0..y {
                    ab[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        ab
    }

    let start = Instant::now();
    let x = 10000;
    let y = 10;
    let z = 1;

    let mut rng = rand::thread_rng();
    let range = Uniform::new(0., 1.);

    let a = vec![(0..y).map(|_| rng.sample(&range)).collect::<Vec<f32>>(); x];
    let b = vec![(0..x).map(|_| rng.sample(&range)).collect::<Vec<f32>>(); y];
    let c = vec![(0..x).map(|_| rng.sample(&range)).collect::<Vec<f32>>(); z];
    println!("matrices created");

    let d = mat_mul(mat_mul(a, b), c);

    // let mut ab = vec![vec![0.0; x]; x];
    // for i in 0..x {
    //     for j in 0..x {
    //         for k in 0..y {
    //             ab[i][j] += a[i][k] * b[k][j];
    //         }
    //     }
    // }

    // println!("switch");
    // let mut out = vec![vec![0.0; z]; x];
    // for i in 0..x {
    //     for j in 0..z {
    //         for k in 0..x {
    //             out[i][j] += ab[i][k] * c[k][j];
    //         }
    //     }
    // }

    println!("done");

    println!("Elapsed time: {}", start.elapsed().as_secs());
}
