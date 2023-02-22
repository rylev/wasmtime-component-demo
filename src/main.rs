use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!("markdown" in "crates/markdown/wit/markdown.wit");

struct Imports {
    stdin: std::io::Stdin,
}
impl MarkdownImports for Imports {
    fn readline(&mut self) -> anyhow::Result<String> {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf)?;
        Ok(buf)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm = std::env::args().skip(1).next().expect("USAGE: demo WASM");
    // Enable component model (which isn't supported by default)
    let mut config = Config::new();
    config.wasm_component_model(true);

    // Create a wasmtime execution context
    let engine = Engine::new(&config)?;
    let mut store = Store::new(
        &engine,
        Imports {
            stdin: std::io::stdin(),
        },
    );
    let mut linker = Linker::new(&engine);
    Markdown::add_to_linker(&mut linker, |state: &mut Imports| state)?;

    // Read and compile the wasm component
    let component = Component::from_file(&engine, wasm)?;

    // Instantiate a markdown instance
    let (markdown, _) = Markdown::instantiate(&mut store, &component, &linker)?;
    println!("Enter some markdown:");
    let res = markdown.call_render(&mut store)?;
    println!("Html:\n{res}");
    Ok(())
}
