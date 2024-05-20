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

mod utils;

use std::{env, fs};

use predicates::prelude::predicate;

#[test]
fn basic_generate() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
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
fn generate_without_out_dir() {
    let current_dir = env::current_dir().unwrap();
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg(current_dir.join("tests/data/input/input.png"))
        .current_dir(out_dir)
        .assert()
        .success();
    assert!(out_dir.join("android-chrome-192x192.png").exists());
    assert!(out_dir.join("android-chrome-512x512.png").exists());
    assert!(out_dir.join("apple-touch-icon.png").exists());
    assert!(out_dir.join("favicon-16x16.png").exists());
    assert!(out_dir.join("favicon-32x32.png").exists());
    assert!(out_dir.join("favicon.ico").exists());
    assert!(out_dir.join("site.webmanifest").exists());
}

#[test]
fn generate_from_non_existent_file() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    let command = utils::command::command()
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
    utils::command::command()
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
fn generate_with_filter() {
    {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
    utils::command::command()
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
