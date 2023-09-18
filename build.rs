extern crate napi_build;

fn main() {
  napi_build::setup();

  println!("cargo:rerun-if-changed=src/service/api.proto");

  let mut conf = prost_build::Config::new();
  conf.service_generator(Box::new(dcl_rpc::codegen::RPCServiceGenerator::new()));
  conf.compile_protos(&["api.proto"], &["./"]).unwrap();
  // Ok(())
}
