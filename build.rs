use rust_bundler::Bundler;
fn main() {
    let mut bundler = Bundler::new(
        "cplit",
        "main.rs",
        "singlefile.rs",
        true,
    );

    bundler.run();
}