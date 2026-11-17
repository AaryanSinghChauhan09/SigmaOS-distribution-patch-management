// Integration test for SigmaOS-distribution-patch-management
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use distribution_patch_management as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
