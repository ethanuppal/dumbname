// Copyright (C) 2024 Ethan Uppal.
//
// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;

use snafu::{ResultExt, Whatever};
use verilog::{verilog, PortDirection, VerilatorRuntime};

#[verilog(src = "sv/main.sv", name = "main")]
struct Main;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    colog::init();

    let mut runtime = VerilatorRuntime::new(
        "artifacts".into(),
        &["sv/main.sv".as_ref()],
        true,
    )?;

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("sv/main.sv")
        .to_string_lossy()
        .to_string();
    let mut main = runtime.create_dyn_model(
        "main",
        &path,
        &[
            ("medium_input", 31, 0, PortDirection::Input),
            ("medium_output", 31, 0, PortDirection::Output),
        ],
    )?;

    main.pin("medium_input", u32::MAX).whatever_context("pin")?;
    println!("{}", main.read("medium_output").whatever_context("read")?);
    assert_eq!(
        main.read("medium_output").whatever_context("read")?,
        0u32.into()
    );
    main.eval();
    println!("{}", main.read("medium_output").whatever_context("read")?);
    assert_eq!(
        main.read("medium_output").whatever_context("read")?,
        u32::MAX.into()
    );

    Ok(())
}
