use camino::Utf8Path;

use uniffi_kotlin_multiplatform::KotlinBindingGenerator;

fn main() {
    let out_dir = Utf8Path::new("target/bindings");
    uniffi_bindgen::generate_external_bindings(
        KotlinBindingGenerator {},
        "./src/callbacks.udl",
        None::<&Utf8Path>,
        Some(out_dir),
    ).unwrap();
}
