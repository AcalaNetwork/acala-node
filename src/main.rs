// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod chain_spec;

use polkadot_parachain_lib::{run, CommandConfig};

fn main() -> color_eyre::eyre::Result<()> {
	color_eyre::install()?;

    let config = CommandConfig {
		chain_spec_loader: Some(Box::new(chain_spec::ChainSpecLoader)),
		runtime_resolver: Some(Box::new(chain_spec::RuntimeResolver)),
	};
	Ok(run(config)?)
}
