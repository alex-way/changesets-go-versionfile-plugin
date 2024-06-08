extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/protos/plugin.proto"], &["src/protos/"]).unwrap();
}
