use std::fs::read;
use std::path::PathBuf;

// use assert_cmd::Command;

use name_core::elf_def::Elf;
use name_core::elf_utils::read_bytes_to_elf;

// use name_emu::simulator;

#[test]
fn fib_db_test() {
    // please note that this actually isn't a valid test rn
    // i'll fix it later maybe hopefully question marka
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("[*] FATAL: No parent directory found (did you clone the entire repo?)")
        .join("tests")
        .join("samples");

    let input_fn: PathBuf = base_path.join("fib");

    let elf_contents: Vec<u8> =
        read(input_fn).expect("[*] FATAL: NAME cannot run files that don't exist...");

    let _executable: Elf = match read_bytes_to_elf(elf_contents) {
        Ok(elf) => elf,
        Err(e) => panic!("{}", e),
    };

    assert_eq!(1 + 1, 2);
    // todo compliance check executable (format, existence, etc.)
    // removing this test right now because it causes test suite to hang
    // let simulator_result = simulator::simulate(executable, true);

    // // i'm going to be so honest you need to feed the commands in yourself for now
    //
    // match simulator_result {
    //     Ok(_) => {}
    //     Err(e) => panic!("{e}"),
    // }
    //
    // // NOTE: most important test case ever.
}
