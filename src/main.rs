use wasmtime::*;

// Embed the local WASM file
const EMBEDDED_WASM: &[u8] = include_bytes!("embedded_module.wasm");

// Function to access the embedded WASM
pub fn get_embedded_wasm() -> &'static [u8] {
    EMBEDDED_WASM
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Access the embedded WASM
    let wasm_bytes = get_embedded_wasm();

    // Initialize the Wasmtime engine and module
    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)?;

    // Create a new Store
    let mut store = Store::new(&engine, ());

    // Instantiate the module
    let instance = Instance::new(&mut store, &module, &[])?;

    // Call a function exported by the WASM module (if any)
    if let Some(func) = instance.get_typed_func::<(), ()>(&mut store, "main").ok() {
        func.call(&mut store, ())?;
    }

    Ok(())
}
