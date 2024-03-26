// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs,
    io::{self, Read},
    time::Duration,
};

use anyhow::{bail, Context};
use bat::PrettyPrinter;
use clap::Parser;
use image::ImageFormat;
use indicatif::ProgressBar;
use serde_json::json;

use crate::{cli::Opt, generate};

const HTML: &str = concat!(
    r#"<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />"#,
    '\n',
    r#"<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />"#,
    '\n',
    r#"<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />"#,
    '\n',
    r#"<link rel="manifest" href="/site.webmanifest" />"#,
    '\n'
);

/// Runs the program and returns the result.
#[allow(clippy::too_many_lines)]
pub fn run() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if let Some(shell) = opt.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    let input = match opt.input {
        Some(ref path) if path.to_str().unwrap_or_default() != "-" => {
            fs::read(path).with_context(|| format!("could not read data from {path:?}"))?
        }
        _ => {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("could not read data from stdin")?;
            buf
        }
    };
    let format = if let Some(f) = opt.format {
        f.into()
    } else {
        image::guess_format(&input)
            .or_else(|err| opt.input.map_or_else(|| Err(err), ImageFormat::from_path))
            .context("could not determine the image format")?
    };
    let image = image::load_from_memory_with_format(&input, format)
        .map_err(anyhow::Error::from)
        .context("could not read the image")?;
    if image.width() != image.height() {
        bail!("image is not square");
    }
    let image = image.into_rgba8().into();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(50));
    pb.set_message("Generating favicons");

    let filter = opt.filter.into();
    let android_favicons = generate::for_android(&image, filter);
    let apple_favicon = generate::for_apple(&image, filter);
    let png_favicons = generate::png_favicons(&image, filter);
    let ico_favicon = generate::ico_favicon(&image, filter, opt.png)?;
    let webmanifest = json!({
        "name": opt.name,
        "short_name": opt.short_name.unwrap_or(opt.name),
        "icons": [
            {
                "src": "/android-chrome-192x192.png",
                "sizes": "192x192",
                "type": "image/png"
            },
            {
                "src": "/android-chrome-512x512.png",
                "sizes": "512x512",
                "type": "image/png"
            }
        ],
        "theme_color": opt.theme_color.to_hex_string(),
        "background_color": opt.background_color.to_hex_string(),
        "display": "standalone"
    });

    if !opt.output.exists() {
        fs::create_dir_all(&opt.output).context("could not create output directory")?;
    }
    let out_dir = opt
        .output
        .canonicalize()
        .context("could not canonicalize the output directory path")?;

    for favicon in android_favicons {
        let file = out_dir.join(format!(
            "android-chrome-{}x{}.png",
            favicon.width(),
            favicon.height()
        ));
        favicon
            .save(&file)
            .with_context(|| format!("could not write the image to {file:?}"))?;
    }

    let apple_file = out_dir.join("apple-touch-icon.png");
    apple_favicon
        .save(&apple_file)
        .with_context(|| format!("could not write the image to {apple_file:?}"))?;

    for favicon in png_favicons {
        let file = out_dir.join(format!(
            "favicon-{}x{}.png",
            favicon.width(),
            favicon.height()
        ));
        favicon
            .save(&file)
            .with_context(|| format!("could not write the image to {file:?}"))?;
    }

    let ico_file = out_dir.join("favicon.ico");
    fs::write(&ico_file, ico_favicon)
        .with_context(|| format!("could not write the image to {ico_file:?}"))?;

    let webmanifest =
        serde_json::to_string_pretty(&webmanifest).context("could not serialize as JSON")?;
    let webmanifest_file = out_dir.join("site.webmanifest");
    fs::write(&webmanifest_file, webmanifest)
        .with_context(|| format!("could not write the web app manifest to {webmanifest_file:?}"))?;

    pb.finish_with_message(format!("Saved the generated files to {out_dir:?}."));

    println!("\n");
    println!("Copy the following and paste them into the <head> of your HTML.");
    PrettyPrinter::new()
        .input_from_bytes(HTML.as_bytes())
        .language("html")
        .line_numbers(true)
        .grid(true)
        .print()
        .context("could not pretty-print HTML")?;
    Ok(())
}
