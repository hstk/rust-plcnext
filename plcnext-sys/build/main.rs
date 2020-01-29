use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  // // Tell cargo to tell rustc to link the PLCnext ANSI-C libraries.
  println!("cargo:rustc-link-lib=Arp.System.Commons");
  println!("cargo:rustc-link-lib=Arp.System.ModuleLib");
  println!("cargo:rustc-link-lib=Arp.System.Module");
  println!("cargo:rustc-link-lib=Arp.System.Rsc");
  println!("cargo:rustc-link-lib=Arp.Plc.AnsiC");

  let builder = bindgen::Builder::default()
    .header("include/wrapper.h")
    .generate()
    .expect("Couldn't generate bindings!")
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
