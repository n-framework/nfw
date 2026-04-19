use cliclack::{multiselect};

fn main() {
    // We'll try common names and see which one compiles.
    // Uncomment one at a time to test.
    let _ = multiselect("test").help("test help");
    // let _ = multiselect("test").footer("test footer");
    // let _ = multiselect("test").hint("test hint");
}
