pub fn print_help() {
    println!(
        "Loo-Cast alpha xtask

Commands:
  help
  build [--launcher]         (alias: build_fastdev)
  build_dev
  build_fastdev
  build_release
  build_linux_release
  build_windows_release
  package [--launcher]       (alias: package_fastdev)
  package_dev
  package_fastdev
  package_release
  package_linux_release
  package_windows_release
  run [--launcher]           (alias: run_fastdev)
  run_dev
  run_fastdev
  run_release
  audit
  build_docs
  open_docs
  setup_sdk
  clean_sdk
  cloc
  gource
  deploy

Flags:
  --launcher                 available only for default build/package/run"
    );
}
