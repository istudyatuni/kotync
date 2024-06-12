// todo: if this file will be extended to affect clean build, update
// on.update.paths in .github/workflows
fn main() {
    println!("cargo:rerun-if-changed=migrations");
    const VERSION: &str = "cargo:rustc-env=VERSION";
    if let Ok(version) = std::fs::read_to_string(".version") {
        println!("{VERSION}={version}");
    } else {
        println!("{VERSION}=local");
    }
}
