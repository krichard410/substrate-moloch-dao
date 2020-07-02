
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/Users/kirstenrichard/Desktop/substrate-moloch-dao/target/release/build/node-template-runtime-315feb5973d97062/out/wasm_binary.rs",
						"/Users/kirstenrichard/Desktop/substrate-moloch-dao/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			