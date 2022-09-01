use std::path::Path;

use kate_ptau_rs::contribute_with_file;

fn main() {
    contribute_with_file(Path::new("initialContribution.json"), Path::new("out.json")).unwrap();
}