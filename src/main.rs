use std::{path::Path, time::Instant};

use kate_ptau_rs::{contribute_with_json_file, get_entropy};

fn main() {
    let start = Instant::now();
    contribute_with_json_file(
        [0; 32],
        Path::new("initialContribution.json"),
        Path::new("out.json"),
    )
    .unwrap();
    println!("total time: {:?}", start.elapsed());
}
