use rust_bundler::Bundler;
fn main() {
    let mut bundler = Bundler::new(
        "cplit",
        "singlefile.rs.example",
        "singlefile.rs",
        true,
    );

    bundler.run();
}