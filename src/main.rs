// SPDX-FileCopyrightText: 2026 Alex Brandt
// SPDX-License-Identifier: MIT

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {}

fn main() {
    Cli::parse();
}
