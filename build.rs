use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(CHANNEL_NIGHTLY)");
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo::rustc-cfg=CHANNEL_NIGHTLY")
    };
}
