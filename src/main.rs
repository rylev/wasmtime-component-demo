use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!("markdown" in "crates/markdown/wit/markdown.wit");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm = std::env::args().skip(1).next().expect("USAGE: demo WASM");
    // Enable component model (which isn't supported by default)
    let mut config = Config::new();
    config.wasm_component_model(true);

    // Create a wasmtime execution context
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    // Read and compile the wasm component
    let component = Component::from_file(&engine, wasm)?;

    // Instantiate a markdown instance
    let (markdown, _) = Markdown::instantiate(&mut store, &component, &linker)?;
    let res = markdown.call_render(&mut store, "# hello")?;
    assert_eq!(res, "<h1>hello</h1>\n");
    Ok(())
}
