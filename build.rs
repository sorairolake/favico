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
        .args(["-a", concat!("revnumber=", env!("CARGO_PKG_VERSION"))]);
    #[cfg(feature = "dds")]
    command.args(["-a", "dds"]);
    #[cfg(feature = "ff")]
    command.args(["-a", "ff"]);
    #[cfg(feature = "gif")]
    command.args(["-a", "gif"]);
    #[cfg(feature = "hdr")]
    command.args(["-a", "hdr"]);
    #[cfg(feature = "jpeg")]
    command.args(["-a", "jpeg"]);
    #[cfg(feature = "exr")]
    command.args(["-a", "exr"]);
    #[cfg(feature = "pnm")]
    command.args(["-a", "pnm"]);
    #[cfg(feature = "qoi")]
    command.args(["-a", "qoi"]);
    #[cfg(feature = "tga")]
    command.args(["-a", "tga"]);
    #[cfg(feature = "tiff")]
    command.args(["-a", "tiff"]);
    #[cfg(feature = "webp")]
    command.args(["-a", "webp"]);
    command
        .args(["-D", out_dir])
        .arg(man_dir.join("*.1.adoc"))
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
