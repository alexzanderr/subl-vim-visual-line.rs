#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unused_macros
)]

use std::{
    fs,
    io,
    path::Path,
};

/// remove everything inside but not the directory
fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        fs::remove_file(entry?.path())?;
    }
    Ok(())
}

pub fn _main() -> io::Result<()> {
    let release_wheels = "release";
    let release_wheels = Path::new(release_wheels);
    if !release_wheels.exists() {
        fs::create_dir(&release_wheels)?;
    }

    let target_wheels = "target/wheels";
    let mut entries = fs::read_dir(target_wheels)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort_unstable();
    // target/wheels/vim_visual_line-1.1.1-cp38-cp38-manylinux_2_34_x86_64.whl
    let latest_wheel = entries.pop().unwrap();
    println!("original wheel:\n{:?}", latest_wheel);

    let wheel_path = latest_wheel.components().nth(2).unwrap();
    let wheel_path = Path::new(&wheel_path);
    println!("just the filename:\n{:?}", wheel_path);

    let release_latest_wheel = release_wheels.join(&wheel_path);
    println!(
        "release wheel (doesnt exist yet):\n{:?}",
        release_latest_wheel
    );

    // clear everything everytime and just keep the latest one
    remove_dir_contents(&release_wheels)?;
    fs::copy(&latest_wheel, &release_latest_wheel)?;

    // let mut renamed = release_latest_wheel.clone();
    // renamed.set_file_name("vim_visual_line_release-cp38-cp38-manylinux_2_34_x86_64.whl");
    // println!("renamed wheel:\n{:?}", renamed);

    // fs::rename(&release_latest_wheel, &renamed);
    // if !release_latest_wheel.exists() {

    // }

    // The entries have now been sorted by their path.
    Ok(())
}

fn _run_main() {
    if let Err(error) = _main() {
        eprintln!("[error]: {:?}", error);
    }
}

fn main() {
    _run_main()
}
