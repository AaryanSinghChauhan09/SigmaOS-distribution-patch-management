//! SigmaOS-distribution-patch-management — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_distribution_patch_management_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-distribution-patch-management: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-distribution-patch-management: ok");
}
