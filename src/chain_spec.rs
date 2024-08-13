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

use polkadot_parachain_lib::chain_spec::{GenericChainSpec, LoadSpec};
use sc_chain_spec::ChainSpec;

#[derive(Debug)]
pub(crate) struct ChainSpecLoader;
impl LoadSpec for ChainSpecLoader {
    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        Ok(match id {
            "" | "acala" => Box::new(GenericChainSpec::from_json_bytes(include_bytes!(
                "../chainspecs/acala-dist.json"
            ))?),
            "karura" => Box::new(GenericChainSpec::from_json_bytes(include_bytes!(
                "../chainspecs/karura-dist.json"
            ))?),
            path => Box::new(GenericChainSpec::from_json_file(path.into())?),
        })
    }
}
