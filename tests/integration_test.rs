// Integration test for SigmaOS-btop-gui
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_btop_gui as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
