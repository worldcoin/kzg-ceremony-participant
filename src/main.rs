use std::{path::Path, time::Instant};

use kate_ptau_rs::contribute_with_file;

fn main() {
    // let start = Instant::now();
    contribute_with_file(Path::new("initialContribution.json"), Path::new("out.json")).unwrap();
    // println!("total time: {:?}", start.elapsed());
}