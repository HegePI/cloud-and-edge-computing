use std::time::Instant;

fn main() {
    let start = Instant::now();
    let x = 10000;
    let y = 1000;
    let z = 1;

    let a = vec![vec![0.3; y]; x];
    let b = vec![vec![0.3; x]; y];
    let c = vec![vec![0.3; z]; x];

    let mut ab = vec![vec![0.0; x]; x];
    for i in 0..x {
        for j in 0..x {
            for k in 0..y {
                ab[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    println!("switch");
    let mut out = vec![vec![0.0; z]; x];
    for i in 0..x {
        for j in 0..z {
            for k in 0..x {
                out[i][j] += ab[i][k] * c[k][j];
            }
        }
    }

    println!("done");

    println!("Elapsed time: {}", start.elapsed().as_secs());
}
