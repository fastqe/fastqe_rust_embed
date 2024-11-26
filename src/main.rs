
use wasmer::{Instance, Module, Store, imports};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!("embedded_module.wasm");

    // Create a store for static execution
    let store = Store::default();

    // Compile the WASM module
    let module = Module::new(&store, wasm_bytes)?;

    // Create an import object
    let import_object = imports! {};

    // Instantiate the module
    let instance = Instance::new(&module, &import_object)?;

    // Call the "main" function
    if let Ok(func) = instance.exports.get_function("main") {
        func.call(&[])?;
    }

    Ok(())
}

