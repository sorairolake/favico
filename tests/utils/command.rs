// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use assert_cmd::Command;

pub fn command() -> Command {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.current_dir("tests");
    command
}
