mod structures;

use structures::*;

fn main() {
    let mut bpm = BPlusMap::new(10);
    bpm.insert(10, 2);
}
