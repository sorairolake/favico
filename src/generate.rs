// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Cursor;

use image::{
    ColorType, DynamicImage, ImageFormat, ImageResult,
    codecs::ico::{IcoEncoder, IcoFrame},
    imageops::FilterType,
};

/// Creates favicons for Android.
pub fn for_android(image: &DynamicImage, filter: FilterType) -> Vec<DynamicImage> {
    let mut favicons = vec![DynamicImage::default(); 2];
    let sizes = [192, 512];
    for (i, size) in sizes.iter().enumerate() {
        favicons[i] = image.resize(*size, *size, filter);
    }
    favicons
}

/// Creates favicon for iOS.
pub fn for_apple(image: &DynamicImage, filter: FilterType) -> DynamicImage {
    let size = 180;
    image.resize(size, size, filter)
}

/// Creates PNG favicons.
pub fn png_favicons(image: &DynamicImage, filter: FilterType) -> Vec<DynamicImage> {
    let mut favicons = vec![DynamicImage::default(); 2];
    let sizes = [16, 32];
    for (i, size) in sizes.iter().enumerate() {
        favicons[i] = image.resize(*size, *size, filter);
    }
    favicons
}

/// Creates ICO favicon.
pub fn ico_favicon(image: &DynamicImage, filter: FilterType, is_png: bool) -> ImageResult<Vec<u8>> {
    let mut frames = Vec::with_capacity(3);
    let sizes = [16, 32, 48];
    for size in sizes {
        let favicon = image.resize(size, size, filter).into_rgba8();
        let format = if is_png {
            ImageFormat::Png
        } else {
            ImageFormat::Bmp
        };
        let mut buf = Vec::new();
        favicon.write_to(&mut Cursor::new(&mut buf), format)?;
        let frame = IcoFrame::with_encoded(buf, size, size, ColorType::Rgba8.into())?;
        frames.push(frame);
    }
    let mut buf = Vec::new();
    let encoder = IcoEncoder::new(&mut buf);
    encoder.encode_images(&frames)?;
    Ok(buf)
}
