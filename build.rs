#![allow(missing_docs)]
fn main() {
    #[cfg(feature = "r1")]
    println!("cargo:rebuild-if-changed=device_R1.yaml");
    #[cfg(feature = "r4")]
    println!("cargo:rebuild-if-changed=device_R4.yaml");

    #[cfg(not(any(feature = "r1", feature = "r4")))]
    compile_error!("Please enable a chip revision feature that corresponds to the one you are using!");

    #[cfg(all(feature = "r1", feature = "r4"))]
    compile_error!("Please enable only one chip revision that corresponds to the one you are using!");
}
