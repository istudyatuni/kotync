// todo: if this file will be extended to affect clean build, update
// on.update.paths in .github/workflows
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
