use walrus::{ExportItem, Module};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        println!("Usage: {} {{file}} {{start-function}}", args[0]);
        std::process::exit(1);
    }

    let file = args[1].as_str();
    let start = args[2].as_str();

    let mut wasm = Module::from_file(file).unwrap();

    if wasm.start.is_some() {
        panic!("module already has a start function")
    }

    let (export, func_id) = wasm
        .exports
        .iter()
        .filter_map(|e| {
            if let ExportItem::Function(id) = e.item {
                Some((e, id))
            } else {
                None
            }
        })
        .find(|(e, _)| e.name == start)
        .unwrap_or_else(|| panic!("failed to find exported function with name `{start}`"));

    wasm.start = Some(func_id);
    wasm.exports.delete(export.id());

    wasm.emit_wasm_file(file).unwrap();
}
