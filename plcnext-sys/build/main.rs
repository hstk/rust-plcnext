use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
  let axc_2152_default_sysroot = String::from("/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi");
  let _includes =
    "/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext";

  let env_sysroot = PathBuf::from(env::var("PLCNEXT_TARGET_SYSROOT").unwrap_or(axc_2152_default_sysroot));

  // // Tell cargo to tell rustc to link the PLCnext ANSI-C libraries.
  println!("cargo:rustc-link-lib=Arp.System.Commons");
  println!("cargo:rustc-link-lib=Arp.System.ModuleLib");
  println!("cargo:rustc-link-lib=Arp.System.Module");
  println!("cargo:rustc-link-lib=Arp.System.Rsc");
  println!("cargo:rustc-link-lib=Arp.Plc.AnsiC");
  // println!("cargo:rustc-link-lib=stdc++");
  let headers = "
        #include \"Arp/Plc/AnsiC/ArpPlc.h\"
        #include \"Arp/Plc/AnsiC/Device/Device.h\"
        #include \"Arp/Plc/AnsiC/Domain/PlcManager.h\"
        #include \"Arp/Plc/AnsiC/Domain/PlcOperation.h\"
        #include \"Arp/Plc/AnsiC/Domain/PlcOperationHandler.h\"
        #include \"Arp/Plc/AnsiC/Gds/DataLayout.h\"
        #include \"Arp/Plc/AnsiC/Gds/GdsBuffer.h\"
        #include \"Arp/Plc/AnsiC/Io/Axio.h\"
        #include \"Arp/Plc/AnsiC/Io/FbIoSystem.h\"";

  let builder = bindgen::Builder::default()
    .header("include/wrapper.h")
    // .header_contents("wrap.h", headers)
    .clang_arg("--target=arm-pxc-linux-gnueabi")
    .clang_arg("--sysroot=/opt/pxc/2020.0")
    .clang_arg("-march=armv7-a")
    .clang_arg("-mthumb")
    .clang_arg("-mfpu=neon")
    .clang_arg("-mfloat-abi=hard")
    .clang_arg("-mcpu=cortex-a9")
    .clang_arg("-I/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext")
    .clang_arg("-v");
  // .clang_args(vec!("-target arm-pxc-linux-gnueabi", "--sysroot=/opt/pxc/2020.0", "-I/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext"));
  // .clang_arg("--include-directory=/usr/lib/llvm-8/lib/clang/8.0.0/include")

  // export BINDGEN_EXTRA_CLANG_ARGS="-target arm-pxc-linux-gnueabi --sysroot=/opt/pxc/2020.0 -I/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext"

  // export BINDGEN_EXTRA_CLANG_ARGS="-I/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext"

  // Finish the builder and generate the bindings.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  let flags = builder.command_line_flags();

  println!("HEY MAN");
  println!("{:#?}", flags);

  let bindings = builder.generate().expect("Unable to generate bindings");

  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Unable to write bindings to file");
}
