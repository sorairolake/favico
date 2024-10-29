// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use std::fs;

use predicates::prelude::predicate;

#[test]
fn set_name_member() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
        utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
fn set_theme_color_member_from_oklab_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("oklab(50.4% -0.0906 0.0069)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#0f7562","##));
}

#[test]
fn set_background_color_member_from_oklab_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("oklab(61.9% -0.0120 -0.0302)")
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
fn set_theme_color_member_from_oklab_color_with_alpha() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("oklab(50.4% -0.0906 0.0069 / 0.5)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#0f756280","##));
}

#[test]
fn set_background_color_member_from_oklab_color_with_alpha() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("oklab(61.9% -0.0120 -0.0302 / 0.5)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""background_color": "#77889980","##));
}

#[test]
fn set_theme_color_member_from_invalid_oklab_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("oklab(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklab(0)' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid oklab format"));
}

#[test]
fn set_background_color_member_from_invalid_oklab_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("oklab(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklab(0)' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid oklab format"));
}

#[test]
fn set_theme_color_member_from_oklch_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("oklch(59.41% 0.16 301.29)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#9063cd","##));
}

#[test]
fn set_background_color_member_from_oklch_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("oklch(61.9% 0.032 248.35)")
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
fn set_theme_color_member_from_oklch_color_with_alpha() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("oklch(59.41% 0.16 301.29 / 49.8%)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""theme_color": "#9063cd7f","##));
}

#[test]
fn set_background_color_member_from_oklch_color_with_alpha() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("oklch(61.9% 0.032 248.35 / 49.8%)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .success();
    assert!(fs::read_to_string(out_dir.join("site.webmanifest"))
        .unwrap()
        .contains(r##""background_color": "#7788997f","##));
}

#[test]
fn set_theme_color_member_from_invalid_oklch_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--theme-color")
        .arg("oklch(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklch(0)' for '--theme-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid oklch format"));
}

#[test]
fn set_background_color_member_from_invalid_oklch_color() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
        .arg("-o")
        .arg(out_dir)
        .arg("--background-color")
        .arg("oklch(0)")
        .arg("--filter")
        .arg("nearest")
        .arg("data/input/input.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklch(0)' for '--background-color <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid oklch format"));
}

#[test]
fn set_theme_color_member_from_invalid_color_function() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir = out_dir.path();
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
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
