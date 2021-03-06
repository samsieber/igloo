extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str) {
  let mut config = compiletest::Config::default();
  let cfg_mode = mode.parse().expect("Invalid mode");

  config.mode = cfg_mode;
  config.src_base = PathBuf::from(format!("tests/{}", mode));
  config.target_rustcflags = Some("-L libiceblock_jade.rlib".to_string());
//  config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());
  config.link_deps();
  config.clean_rmeta();

  compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
  run_mode("compile-fail");
}

#[test]
fn compile_pass() {
  run_mode("run-pass");
}