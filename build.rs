// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env, io,
    process::{Command, ExitStatus},
};

fn generate_man_page(out_dir: &str) -> io::Result<ExitStatus> {
    let man_dir = env::current_dir()?.join("docs/man/man1");
    let mut command = Command::new("asciidoctor");
    command
        .args(["-b", "manpage"])
        .args(["-D", out_dir])
        .arg(man_dir.join("favico.1.adoc"))
        .status()
}

fn main() {
    println!("cargo:rerun-if-changed=docs/man");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not defined");
    match generate_man_page(&out_dir) {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("cargo:warning=Asciidoctor failed: {exit_status}");
            }
        }
        Err(err) => {
            println!("cargo:warning=failed to execute Asciidoctor: {err}");
        }
    }
}
