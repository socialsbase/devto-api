use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let src = Path::new("openapi/api_v1.json");
    println!("cargo:rerun-if-changed={}", src.display());

    let file = File::open(src).expect("Failed to open OpenAPI spec");
    let spec = serde_json::from_reader(file).expect("Failed to parse OpenAPI spec");

    let mut generator = progenitor::Generator::default();

    let tokens = generator.generate_tokens(&spec).expect("Failed to generate code");
    let ast = syn::parse2(tokens).expect("Failed to parse generated code");
    let content = prettyplease::unparse(&ast);

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_dir).join("codegen.rs");

    std::fs::write(&out_path, content).expect("Failed to write generated code");
}
