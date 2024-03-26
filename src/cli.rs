// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{self, Write},
    path::PathBuf,
};

use clap::{CommandFactory, Parser, ValueEnum, ValueHint};
use clap_complete::Generator;
use csscolorparser::Color;
use image::{imageops::FilterType, ImageFormat};

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    '\n',
    "Copyright (C) 2024 Shun Sakai\n",
    '\n',
    "This program is distributed under the terms of the GNU General Public License\n",
    "v3.0 or later.\n",
    '\n',
    "This is free software: you are free to change and redistribute it. There is NO\n",
    "WARRANTY, to the extent permitted by law.\n",
    '\n',
    "Report bugs to <https://github.com/sorairolake/favico/issues>."
);

const AFTER_LONG_HELP: &str = "See `favico(1)` for more details.";

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[command(
    version,
    long_version(LONG_VERSION),
    about,
    max_term_width(100),
    after_long_help(AFTER_LONG_HELP),
    arg_required_else_help(true)
)]
pub struct Opt {
    /// Directory to output generated files.
    ///
    /// If the directory does not exist, it will be created.
    #[arg(
        short,
        long,
        default_value("."),
        value_name("PATH"),
        value_hint(ValueHint::DirPath)
    )]
    pub output: PathBuf,

    /// Store PNG images instead of BMP images to an ICO image.
    #[arg(long)]
    pub png: bool,

    /// Set the name member of the web app manifest.
    #[arg(long, default_value_t, value_name("NAME"))]
    pub name: String,

    /// Set the short_name member of the web app manifest.
    ///
    /// If <NAME> is not specified, the name specified in '--name' will be set.
    #[arg(long, value_name("NAME"))]
    pub short_name: Option<String>,

    /// Set the theme_color member of the web app manifest.
    ///
    /// <COLOR> takes a CSS color string.
    #[arg(long, default_value("#ffffff"), value_name("COLOR"))]
    pub theme_color: Color,

    /// Set the background_color member of the web app manifest.
    ///
    /// <COLOR> takes a CSS color string.
    #[arg(long, default_value("#ffffff"), value_name("COLOR"))]
    pub background_color: Color,

    /// Sampling filter used to resize the input image.
    #[arg(
        long,
        value_enum,
        default_value_t,
        value_name("FILTER"),
        ignore_case(true)
    )]
    pub filter: Filter,

    /// The format of the input.
    ///
    /// If <FORMAT> is not specified, the format is determined based on the
    /// extension or the magic number.
    #[arg(short, long, value_enum, value_name("FORMAT"), ignore_case(true))]
    pub format: Option<Format>,

    /// Generate shell completion.
    ///
    /// The completion is output to stdout.
    #[arg(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    /// Input image file.
    ///
    /// If [IMAGE] is not specified, or if "-" is specified, the image will be
    /// read from stdin. Supported raster image formats are based on the formats
    /// supported by the image crate. The format of [IMAGE] is determined based
    /// on the extension or the magic number if possible. If the format cannot
    /// be determined, use '--format'. Note that [IMAGE] must be square.
    #[arg(value_name("IMAGE"), value_hint(ValueHint::FilePath))]
    pub input: Option<PathBuf>,
}

impl Opt {
    /// Generates shell completion and print it.
    pub fn print_completion(gen: impl Generator) {
        clap_complete::generate(
            gen,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bash.
    Bash,

    /// Elvish.
    Elvish,

    /// fish.
    Fish,

    /// Nushell.
    Nushell,

    #[allow(clippy::enum_variant_names)]
    /// PowerShell.
    PowerShell,

    /// Zsh.
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Filter {
    /// Nearest Neighbor.
    Nearest,

    /// Linear Filter.
    Triangle,

    /// Cubic Filter.
    #[default]
    CatmullRom,

    /// Gaussian Filter.
    Gaussian,

    /// Lanczos with window 3.
    Lanczos3,
}

impl From<Filter> for FilterType {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Nearest => Self::Nearest,
            Filter::Triangle => Self::Triangle,
            Filter::CatmullRom => Self::CatmullRom,
            Filter::Gaussian => Self::Gaussian,
            Filter::Lanczos3 => Self::Lanczos3,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Format {
    /// Windows Bitmap.
    Bmp,

    /// DirectDraw Surface.
    Dds,

    /// Farbfeld.
    Farbfeld,

    /// Graphics Interchange Format.
    Gif,

    /// Radiance RGBE.
    Hdr,

    /// ICO file format.
    ///
    /// This value also includes the CUR file format.
    Ico,

    /// JPEG.
    Jpeg,

    /// OpenEXR.
    OpenExr,

    /// Portable Network Graphics.
    Png,

    /// Portable Anymap Format.
    Pnm,

    /// Quite OK Image Format.
    Qoi,

    /// Truevision TGA.
    Tga,

    /// Tag Image File Format.
    Tiff,

    /// WebP.
    WebP,
}

impl From<Format> for ImageFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Bmp => Self::Bmp,
            Format::Dds => Self::Dds,
            Format::Farbfeld => Self::Farbfeld,
            Format::Gif => Self::Gif,
            Format::Hdr => Self::Hdr,
            Format::Ico => Self::Ico,
            Format::Jpeg => Self::Jpeg,
            Format::OpenExr => Self::OpenExr,
            Format::Png => Self::Png,
            Format::Pnm => Self::Pnm,
            Format::Qoi => Self::Qoi,
            Format::Tga => Self::Tga,
            Format::Tiff => Self::Tiff,
            Format::WebP => Self::WebP,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        Opt::command().debug_assert();
    }

    #[test]
    fn file_name_shell() {
        assert_eq!(Shell::Bash.file_name("favico"), "favico.bash");
        assert_eq!(Shell::Elvish.file_name("favico"), "favico.elv");
        assert_eq!(Shell::Fish.file_name("favico"), "favico.fish");
        assert_eq!(Shell::Nushell.file_name("favico"), "favico.nu");
        assert_eq!(Shell::PowerShell.file_name("favico"), "_favico.ps1");
        assert_eq!(Shell::Zsh.file_name("favico"), "_favico");
    }

    #[test]
    fn default_filter() {
        assert_eq!(Filter::default(), Filter::CatmullRom);
    }

    #[test]
    fn from_filter_to_filter_type() {
        assert_eq!(FilterType::from(Filter::Nearest), FilterType::Nearest);
        assert_eq!(FilterType::from(Filter::Triangle), FilterType::Triangle);
        assert_eq!(FilterType::from(Filter::CatmullRom), FilterType::CatmullRom);
        assert_eq!(FilterType::from(Filter::Gaussian), FilterType::Gaussian);
        assert_eq!(FilterType::from(Filter::Lanczos3), FilterType::Lanczos3);
    }

    #[test]
    fn from_format_to_image_format() {
        assert_eq!(ImageFormat::from(Format::Bmp), ImageFormat::Bmp);
        assert_eq!(ImageFormat::from(Format::Dds), ImageFormat::Dds);
        assert_eq!(ImageFormat::from(Format::Farbfeld), ImageFormat::Farbfeld);
        assert_eq!(ImageFormat::from(Format::Gif), ImageFormat::Gif);
        assert_eq!(ImageFormat::from(Format::Hdr), ImageFormat::Hdr);
        assert_eq!(ImageFormat::from(Format::Ico), ImageFormat::Ico);
        assert_eq!(ImageFormat::from(Format::Jpeg), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::from(Format::OpenExr), ImageFormat::OpenExr);
        assert_eq!(ImageFormat::from(Format::Png), ImageFormat::Png);
        assert_eq!(ImageFormat::from(Format::Pnm), ImageFormat::Pnm);
        assert_eq!(ImageFormat::from(Format::Qoi), ImageFormat::Qoi);
        assert_eq!(ImageFormat::from(Format::Tga), ImageFormat::Tga);
        assert_eq!(ImageFormat::from(Format::Tiff), ImageFormat::Tiff);
        assert_eq!(ImageFormat::from(Format::WebP), ImageFormat::WebP);
    }
}
