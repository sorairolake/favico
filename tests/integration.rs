// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

fn command() -> Command {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.current_dir("tests");
    command
}

#[test]
fn generate_completion() {
    command()
        .arg("--generate-completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("elvish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("nushell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("powershell")
        .assert()
        .success()
        .stdout(predicate::ne(""));
    command()
        .arg("--generate-completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::ne(""));
}

#[test]
fn generate_completion_with_invalid_shell() {
    command()
        .arg("--generate-completion")
        .arg("a")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--generate-completion <SHELL>'",
        ));
}

#[test]
fn long_version() {
    command()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help() {
    command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/after-long-help.md"
        )));
}

#[test]
fn basic_generate() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("data/input/input.png")
        .assert()
        .success();
    assert_eq!(
        image::open(out_dir.join("android-chrome-192x192.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/android-chrome-192x192.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("android-chrome-512x512.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/android-chrome-512x512.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("apple-touch-icon.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/apple-touch-icon.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("favicon-16x16.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/favicon-16x16.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("favicon-32x32.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/favicon-32x32.png")
            .unwrap()
            .into_rgba8()
    );
    assert!(out_dir.join("favicon.ico").exists());
    assert_eq!(
        fs::read_to_string(out_dir.join("site.webmanifest")).unwrap(),
        include_str!("data/output/site.webmanifest")
    );
}

#[test]
fn generate_from_non_existent_file() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    let command = command()
        .arg("-o")
        .arg(out_dir)
        .arg("--filter")
        .arg("nearest")
        .arg("non_existent.txt")
        .assert()
        .failure()
        .code(66)
        .stderr(predicate::str::contains(
            r#"could not read data from "non_existent.txt""#,
        ));
    if cfg!(windows) {
        command.stderr(predicate::str::contains(
            "The system cannot find the file specified. (os error 2)",
        ));
    } else {
        command.stderr(predicate::str::contains(
            "No such file or directory (os error 2)",
        ));
    }
}

#[test]
fn generate_png_ico() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--png")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert_eq!(
        fs::read(out_dir.join("favicon.ico"))
            .unwrap()
            .windows(8)
            .filter(|b| *b == [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])
            .count(),
        3
    );
}

#[test]
fn set_name_member() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--name")
        .arg(env!("CARGO_PKG_NAME"))
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    let webmanifest = fs::read_to_string(out_dir.join("site.webmanifest")).unwrap();
    assert!(webmanifest.contains(r#""name": "favico","#));
    assert!(webmanifest.contains(r#""short_name": "favico","#));
}

#[test]
fn set_short_name_member() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--short-name")
        .arg(env!("CARGO_PKG_NAME"))
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    let webmanifest = fs::read_to_string(out_dir.join("site.webmanifest")).unwrap();
    assert!(webmanifest.contains(r#""name": "","#));
    assert!(webmanifest.contains(r#""short_name": "favico","#));
}

#[test]
fn set_theme_color_member_from_named_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("brown")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#a52a2a","##));
}

#[test]
fn set_background_color_member_from_named_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("lightslategray")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""background_color": "#778899","##));
}

#[test]
fn set_theme_color_member_from_hex_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("#a52a2a")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("a52a2a")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a","##));
    }
}

#[test]
fn set_background_color_member_from_hex_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("#778899")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#778899","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("778899")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#778899","##));
    }
}

#[test]
fn set_theme_color_member_from_hex_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("#a52a2a7f")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a7f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("a52a2a7f")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a7f","##));
    }
}

#[test]
fn set_background_color_member_from_hex_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("#7788997f")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#7788997f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("7788997f")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#7788997f","##));
    }
}

#[test]
fn set_theme_color_member_from_hex_short_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("#111")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#111111","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("111")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#111111","##));
    }
}

#[test]
fn set_background_color_member_from_hex_short_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("#eee")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#eeeeee","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("eee")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#eeeeee","##));
    }
}

#[test]
fn set_theme_color_member_from_short_hex_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("#1118")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#11111188","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("1118")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#11111188","##));
    }
}

#[test]
fn set_background_color_member_from_short_hex_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("#eee8")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#eeeeee88","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("eee8")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#eeeeee88","##));
    }
}

#[test]
fn set_theme_color_member_from_invalid_hex_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("#g")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '#g' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hex format"));
}

#[test]
fn set_background_color_member_from_invalid_hex_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("#g")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '#g' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hex format"));
}

#[test]
fn set_theme_color_member_from_rgb_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("rgb(165 42 42)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("rgb(165, 42, 42)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a","##));
    }
}

#[test]
fn set_background_color_member_from_rgb_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("rgb(119 136 153)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#778899","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("rgb(119, 136, 153)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#778899","##));
    }
}

#[test]
fn set_theme_color_member_from_rgb_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("rgb(165 42 42 / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a7f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("rgb(165, 42, 42, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a7f","##));
    }
}

#[test]
fn set_background_color_member_from_rgb_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("rgb(119 136 153 / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#7788997f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("rgb(119, 136, 153, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#7788997f","##));
    }
}

#[test]
fn set_theme_color_member_from_rgba_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("rgba(165 42 42 / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a7f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("rgba(165, 42, 42, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#a52a2a7f","##));
    }
}

#[test]
fn set_background_color_member_from_rgba_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("rgba(119 136 153 / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#7788997f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("rgba(119, 136, 153, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#7788997f","##));
    }
}

#[test]
fn set_theme_color_member_from_invalid_rgb_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("rgb(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'rgb(0)' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid rgb format"));
}

#[test]
fn set_background_color_member_from_invalid_rgb_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("rgb(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'rgb(0)' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid rgb format"));
}

#[test]
fn set_theme_color_member_from_hsl_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("hsl(248 39% 39.2%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#473d8b","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("hsl(248, 39%, 39.2%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#473d8b","##));
    }
}

#[test]
fn set_background_color_member_from_hsl_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("hsl(0 0% 66.3%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#a9a9a9","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("hsl(0, 0%, 66.3%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#a9a9a9","##));
    }
}

#[test]
fn set_theme_color_member_from_hsl_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("hsl(248 39% 39.2% / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#473d8b7f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("hsl(248, 39%, 39.2%, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#473d8b7f","##));
    }
}

#[test]
fn set_background_color_member_from_hsl_color_with_alpha() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("hsl(0 0% 66.3% / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#a9a9a97f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("hsl(0, 0%, 66.3%, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#a9a9a97f","##));
    }
}

#[test]
fn set_theme_color_member_from_hsla_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("hsla(248 39% 39.2% / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#473d8b7f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--theme-color")
            .arg("hsla(248, 39%, 39.2%, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""theme_color": "#473d8b7f","##));
    }
}

#[test]
fn set_background_color_member_from_hsla_color() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("hsla(0 0% 66.3% / 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#a9a9a97f","##));
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--background-color")
            .arg("hsla(0, 0%, 66.3%, 49.8%)")
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
        assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
            .unwrap()
            .contains(r##""background_color": "#a9a9a97f","##));
    }
}

#[test]
fn set_theme_color_member_from_invalid_hsl_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("hsl(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hsl(0)' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hsl format"));
}

#[test]
fn set_background_color_member_from_invalid_hsl_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("hsl(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hsl(0)' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hsl format"));
}

#[test]
fn set_theme_color_member_from_hwb_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("hwb(50.6 0% 0%)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#ffd700","##));
}

#[test]
fn set_background_color_member_from_hwb_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("hwb(0 66.3% 33.7%)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""background_color": "#a9a9a9","##));
}

#[test]
fn set_theme_color_member_from_hwb_color_with_alpha() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("hwb(50.6 0% 0% / 49.8%)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#ffd7007f","##));
}

#[test]
fn set_background_color_member_from_hwb_color_with_alpha() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("hwb(0 66.3% 33.7% / 49.8%)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""background_color": "#a9a9a97f","##));
}

#[test]
fn set_theme_color_member_from_invalid_hwb_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("hwb(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hwb(0)' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hwb format"));
}

#[test]
fn set_background_color_member_from_invalid_hwb_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("hwb(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hwb(0)' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hwb format"));
}

#[test]
fn set_theme_color_member_from_invalid_color_function() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("fn(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'fn(0)' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn set_background_color_member_from_invalid_color_function() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("fn(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'fn(0)' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn set_theme_color_member_from_unknown_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("a")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid unknown format"));
}

#[test]
fn set_background_color_member_from_unknown_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("a")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid unknown format"));
}

#[test]
fn generate_with_filter() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--filter")
            .arg("nearest")
            .arg("data/input/input.png")
            .assert()
            .success();
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--filter")
            .arg("triangle")
            .arg("data/input/input.png")
            .assert()
            .success();
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--filter")
            .arg("catmullrom")
            .arg("data/input/input.png")
            .assert()
            .success();
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--filter")
            .arg("gaussian")
            .arg("data/input/input.png")
            .assert()
            .success();
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("--filter")
            .arg("lanczos3")
            .arg("data/input/input.png")
            .assert()
            .success();
    }
}

#[test]
fn generate_with_invalid_filter() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--filter")
        .arg("a")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--filter <FILTER>'",
        ));
}

#[test]
fn generate_from_non_image_file() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--filter")
        .arg("nearest")
        .arg("data/output/site.webmanifest")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_bmp() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.bmp")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("bmp")
            .arg("data/input/input.bmp")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_dds() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.dds")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("dds")
            .arg("data/input/input.dds")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_farbfeld() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.ff")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("farbfeld")
            .arg("data/input/input.ff")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_gif() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.gif")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("gif")
            .arg("data/input/input.gif")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_hdr() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.hdr")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("hdr")
            .arg("data/input/input.hdr")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_jpeg() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.jpeg")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("jpeg")
            .arg("data/input/input.jpeg")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_open_exr() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.exr")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("openexr")
            .arg("data/input/input.exr")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[test]
fn decode_from_png() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("-f")
        .arg("png")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert_eq!(
        image::open(out_dir.join("android-chrome-192x192.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/android-chrome-192x192.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("android-chrome-512x512.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/android-chrome-512x512.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("apple-touch-icon.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/apple-touch-icon.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("favicon-16x16.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/favicon-16x16.png")
            .unwrap()
            .into_rgba8()
    );
    assert_eq!(
        image::open(out_dir.join("favicon-32x32.png"))
            .unwrap()
            .into_rgba8(),
        image::open("tests/data/output/favicon-32x32.png")
            .unwrap()
            .into_rgba8()
    );
    assert!(out_dir.join("favicon.ico").exists());
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_ppm() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.ppm")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("pnm")
            .arg("data/input/input.ppm")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_qoi() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.qoi")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("qoi")
            .arg("data/input/input.qoi")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_tga() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.tga")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("tga")
            .arg("data/input/input.tga")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_tiff() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.tiff")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("tiff")
            .arg("data/input/input.tiff")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[allow(clippy::too_many_lines)]
#[test]
fn decode_from_web_p() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("data/input/input.webp")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        command()
            .arg("-o")
            .arg(out_dir)
            .arg("-f")
            .arg("webp")
            .arg("data/input/input.webp")
            .assert()
            .success();
        assert_eq!(
            image::open(out_dir.join("android-chrome-192x192.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-192x192.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("android-chrome-512x512.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/android-chrome-512x512.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("apple-touch-icon.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/apple-touch-icon.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-16x16.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-16x16.png")
                .unwrap()
                .into_rgba8()
        );
        assert_eq!(
            image::open(out_dir.join("favicon-32x32.png"))
                .unwrap()
                .into_rgba8(),
            image::open("tests/data/output/favicon-32x32.png")
                .unwrap()
                .into_rgba8()
        );
        assert!(out_dir.join("favicon.ico").exists());
    }
}

#[test]
fn generate_from_invalid_input_format() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    command()
        .arg("-o")
        .arg(out_dir)
        .arg("--filter")
        .arg("nearest")
        .arg("-f")
        .arg("a")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--format <FORMAT>'",
        ));
}
