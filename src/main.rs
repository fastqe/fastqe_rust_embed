

use wasmer::{Instance, Module, Store};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Access the embedded WASM
    let wasm_bytes = include_bytes!("simple.wasm");

    // Create a new Wasmer store
    let store = Store::default();

    // Compile the WASM module
    let module = Module::new(&store, wasm_bytes)?;

    // Instantiate the module
    let instance = Instance::new(&module, &[])?;

    // Call an exported function (e.g., "main")
    if let Ok(func) = instance.exports.get_function("main") {
        func.call(&[])?;
    }

    Ok(())
}
