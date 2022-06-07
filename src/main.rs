mod structures;

use structures::*;

fn main() {
    let mut bpm = BPlusMap::new(2);
    for i in 0..20 {
        bpm.insert(i, i as i32 * 1000);
    }
    println!("{:#?}", bpm);
}
