# Testing a Verilog project

> [!NOTE]
> This tutorial is aimed at Unix-like systems like macOS, Linux, and WSL.

In this tutorial, we'll setup a SystemVerilog project and test our code with
dumbname. You can find the full source code for this tutorial [here](../verilog-support/example-project/). We won't touch on the advanced aspects or features; the goal is just to provide a simple overfiew sufficient to get started.

## Part 1: The Basics

Let's call our project "tutorial-project" (you are free to call it however you
like):
```shell
mkdir tutorial-project
cd tutorial-project
git init # optional, if using git
```

We'll write a very simple SystemVerilog module: one that forwards its inputs to
its outputs.
```shell
mkdir sv
vi sv/main.sv
```
I'm using the `vi` editor here, but you can use whichever editor you prefer.

For our forwarding module, we'll just pass a medium-sized input to a
corresponding output:
```systemverilog
// file: sv/main.sv
module main(
    input[31:0] medium_input,
    output[31:0] medium_output
);
    assign medium_output = medium_input;
endmodule
```

## Part 2: Testing

Now that we have the setup out of the way, we can start testing our code from Rust.
We'll initialize a Rust project:

```shell
cargo init --bin .
```

In the `Cargo.toml` generated, we'll want to add some dependencies:

```
# file: Cargo.toml
[dependencies]
# other dependencies...
verilog = { git = "https://github.com/ethanuppal/dumbname" }
snafu = "0.8.5" # optional, whatever version
colog = "1.3.0" # optional, whatever version
```

The only required package is `verilog` from dumbname; everything else is just
for fun.

Finally, we'll want to actually write the code that drives our project in `main.rs`:

```rust
// file: src/main.rs
use snafu::Whatever;
use verilog::{verilog, VerilatorRuntime};

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

    let mut main = runtime.create_model::<Main>()?;

    main.medium_input = u32::MAX;
    println!("{}", main.medium_output);
    assert_eq!(main.medium_output, 0);
    main.eval();
    println!("{}", main.medium_output);
    assert_eq!(main.medium_output, u32::MAX);

    Ok(())
}
```

Let's break down the relevant parts of what's going on here:

1. ```rust
#[verilog(src = "sv/main.sv", name = "main")]
struct Main;
``` declares that the Rust `struct Main` binds to the Verilog module `main` as
defined in `sv/main.sv` (this path is relative to the `Cargo.toml` parent directory).

2. ```rust
let mut runtime = VerilatorRuntime::new(
    "artifacts".into(),
    &["sv/main.sv".as_ref()],
    true,
)?;
``` creates a Verilog runtime powered by verilator, allowing you to run Verilog
from Rust.

3. ```rust
let mut main = runtime.create_model::<Main>()?;
``` asks the runtime to create a new version of `Main`, that is, our `main`
model.

I won't comment on the rest; it's just regular Rust --- including the part where
we assign to values and call `eval()` on the model object! (Yes, that is the
same as Verilator's evaluation method).

> [!TIP]
> If you are using `git`, add the `artifacts/` directory managed by the Verilator
runtime to your `.gitignore`.

Finally, we can simply use `cargo run` to drive our design!
