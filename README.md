# elify-lang

`elify-lang` is a Rust library for:

1. Transpiling a JS-like language into **Miden Assembly (MASM)**.
2. Executing MASM programs with **proof generation**.
3. Verifying execution proofs.
4. Running deployed MASM programs via an in-process execution engine with per-program queues.

## Install

In another Rust project:

```toml
[dependencies]
elify-lang = { git = "https://github.com/<your-org>/<your-repo>", package = "elify-lang" }
```

Or if published to crates.io:

```toml
[dependencies]
elify-lang = "0.1"
```

## Quick start

```rust
use elify_lang::{
    transpile_js_to_masm, execute_with_proof, stack_outputs_from_ints, verify_execution,
};

fn main() {
    let source = r#"
        function mul_add(a, b, c) {
            let t = a * b;
            return t + c;
        }

        let x = 6;
        let y = 7;
        let z = 1;
        return mul_add(x, y, z);
    "#;

    let masm = transpile_js_to_masm(source).unwrap();
    let artifacts = execute_with_proof(&masm, &[]).unwrap();

    let outputs = stack_outputs_from_ints(&artifacts.stack_outputs).unwrap();
    let security_bits = verify_execution(
        artifacts.program_info,
        artifacts.stack_inputs,
        outputs,
        &artifacts.proof_bytes,
    )
    .unwrap();

    println!("result={}, security={} bits", artifacts.stack_outputs[0], security_bits);
}
```

## Execution engine API

The `ExecutionEngine` lets you deploy MASM once, then enqueue single tasks or batches:

- Tasks for the same program run sequentially.
- Different programs run in parallel (separate worker threads).
- Events can be decoded from outputs and used to update shared state.

See `tests/execution_engine.rs` for working usage examples.
