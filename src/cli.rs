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

#[allow(clippy::doc_markdown, clippy::struct_excessive_bools)]
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

#[allow(clippy::doc_markdown)]
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

#[allow(clippy::doc_markdown)]
#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Format {
    /// Windows Bitmap.
    Bmp,

    /// DirectDraw Surface.
    #[cfg(feature = "dds")]
    Dds,

    /// Farbfeld.
    #[cfg(feature = "ff")]
    Farbfeld,

    /// Graphics Interchange Format.
    #[cfg(feature = "gif")]
    Gif,

    /// Radiance RGBE.
    #[cfg(feature = "hdr")]
    Hdr,

    /// ICO file format.
    ///
    /// This value also includes the CUR file format.
    Ico,

    /// JPEG.
    #[cfg(feature = "jpeg")]
    Jpeg,

    /// OpenEXR.
    #[cfg(feature = "exr")]
    OpenExr,

    /// Portable Network Graphics.
    Png,

    /// Portable Anymap Format.
    #[cfg(feature = "pnm")]
    Pnm,

    /// Quite OK Image Format.
    #[cfg(feature = "qoi")]
    Qoi,

    /// Truevision TGA.
    #[cfg(feature = "tga")]
    Tga,

    /// Tag Image File Format.
    #[cfg(feature = "tiff")]
    Tiff,

    /// WebP.
    #[cfg(feature = "webp")]
    WebP,
}

impl From<Format> for ImageFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Bmp => Self::Bmp,
            #[cfg(feature = "dds")]
            Format::Dds => Self::Dds,
            #[cfg(feature = "ff")]
            Format::Farbfeld => Self::Farbfeld,
            #[cfg(feature = "gif")]
            Format::Gif => Self::Gif,
            #[cfg(feature = "hdr")]
            Format::Hdr => Self::Hdr,
            Format::Ico => Self::Ico,
            #[cfg(feature = "jpeg")]
            Format::Jpeg => Self::Jpeg,
            #[cfg(feature = "exr")]
            Format::OpenExr => Self::OpenExr,
            Format::Png => Self::Png,
            #[cfg(feature = "pnm")]
            Format::Pnm => Self::Pnm,
            #[cfg(feature = "qoi")]
            Format::Qoi => Self::Qoi,
            #[cfg(feature = "tga")]
            Format::Tga => Self::Tga,
            #[cfg(feature = "tiff")]
            Format::Tiff => Self::Tiff,
            #[cfg(feature = "webp")]
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
        #[cfg(feature = "dds")]
        assert_eq!(ImageFormat::from(Format::Dds), ImageFormat::Dds);
        #[cfg(feature = "ff")]
        assert_eq!(ImageFormat::from(Format::Farbfeld), ImageFormat::Farbfeld);
        #[cfg(feature = "gif")]
        assert_eq!(ImageFormat::from(Format::Gif), ImageFormat::Gif);
        #[cfg(feature = "hdr")]
        assert_eq!(ImageFormat::from(Format::Hdr), ImageFormat::Hdr);
        assert_eq!(ImageFormat::from(Format::Ico), ImageFormat::Ico);
        #[cfg(feature = "jpeg")]
        assert_eq!(ImageFormat::from(Format::Jpeg), ImageFormat::Jpeg);
        #[cfg(feature = "exr")]
        assert_eq!(ImageFormat::from(Format::OpenExr), ImageFormat::OpenExr);
        assert_eq!(ImageFormat::from(Format::Png), ImageFormat::Png);
        #[cfg(feature = "pnm")]
        assert_eq!(ImageFormat::from(Format::Pnm), ImageFormat::Pnm);
        #[cfg(feature = "qoi")]
        assert_eq!(ImageFormat::from(Format::Qoi), ImageFormat::Qoi);
        #[cfg(feature = "tga")]
        assert_eq!(ImageFormat::from(Format::Tga), ImageFormat::Tga);
        #[cfg(feature = "tiff")]
        assert_eq!(ImageFormat::from(Format::Tiff), ImageFormat::Tiff);
        #[cfg(feature = "webp")]
        assert_eq!(ImageFormat::from(Format::WebP), ImageFormat::WebP);
    }
}
