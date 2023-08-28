// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use substrate_wasm_builder::WasmBuilder;

const NO_GENESIS_BUILDER_ENV: &str = "BUILD_NO_GENESIS_BUILDER_SUPPORT";

fn main() {
	WasmBuilder::new()
		.with_current_project()
		.import_memory()
		.export_heap_base()
		.build();

	if std::env::var(NO_GENESIS_BUILDER_ENV).is_ok() {
		WasmBuilder::new()
			.with_current_project()
			.import_memory()
			.export_heap_base()
			.enable_feature("disable-genesis-builder")
			.set_file_name("polkadot_runtime_no_genesis_builder")
			.build();
	};
	println!("cargo:rerun-if-env-changed={}", NO_GENESIS_BUILDER_ENV);
}
