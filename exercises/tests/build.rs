//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;

use std::time::SystemTime;

fn main() {
    // For tests7, set up an environment variable called TEST_FOO.
    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    env::set_var("TEST_FOO", format!("{}", timestamp));
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); // Corrected line for Cargo

    // For tests8, enable the "pass" feature.
    println!("cargo:rustc-cfg=feature=\"pass\"");

    // The following lines are placeholders and won't affect the build process.
    let your_tests7_command = format!("cargo:env=TEST_FOO={}", timestamp);
    let your_tests8_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("cargo:info={} was executed.", your_tests7_command);
    println!("cargo:info={} was executed.", your_tests8_command);
}
