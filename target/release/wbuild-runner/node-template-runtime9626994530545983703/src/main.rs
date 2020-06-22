
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/Users/kirstenrichard/Desktop/substrate-moloch-dao/target/release/build/node-template-runtime-8b6e626665209a05/out/wasm_binary.rs",
						"/Users/kirstenrichard/Desktop/substrate-moloch-dao/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			