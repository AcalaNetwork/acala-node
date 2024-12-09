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

use std::fs;
use std::path::Path;

fn main() {
    substrate_build_script_utils::generate_cargo_keys();
    orml_build_script_utils::check_file_licenses("./src", include_bytes!("./HEADER-GPL3"), &[]);

    let out_dir = "chainspecs";
    fs::create_dir_all(out_dir).expect("Failed to create chainspecs directory");

    let files = [
        (
            "acala-dist.json",
            "https://github.com/AcalaNetwork/Acala/raw/refs/heads/master/resources/acala-dist.json",
        ),
        (
            "karura-dist.json",
            "https://github.com/AcalaNetwork/Acala/raw/refs/heads/master/resources/karura-dist.json",
        ),
    ];

    for (filename, url) in files.iter() {
        let path = Path::new(out_dir).join(filename);
        if !path.exists() {
            let response = ureq::get(url).call().expect("Failed to fetch chainspec");
            let mut file = fs::File::create(&path).expect("Failed to create file");
            let mut reader = response.into_reader();
            std::io::copy(&mut reader, &mut file).expect("Failed to copy data");
        }
    }
}
