use std::{io, path::Path, process::ExitCode};

use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
    targets::{Features, Targets},
};

const STYLESHEETS: &str = "stylesheets/";

fn main() -> ExitCode {
    let dest = std::env::args().nth(1).unwrap_or_else(|| "out/".into());
    let dest = Path::new(&dest);

    if init_dir(dest) {
        return ExitCode::FAILURE;
    }

    let mut stylesheets = match std::fs::read_dir(STYLESHEETS) {
        Ok(read_dir) => read_dir,
        Err(e) => {
            eprintln!("Unable to read {STYLESHEETS}. {e}");
            return ExitCode::FAILURE;
        }
    };

    if stylesheets.all(|entry| process_stylesheet(dest, entry)) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn init_dir(dest: &Path) -> bool {
    let dest = &dest.join(STYLESHEETS);

    match std::fs::remove_dir_all(dest) {
        Ok(()) => println!("Cleared {}", dest.display()),
        Err(e) if matches!(e.kind(), io::ErrorKind::NotFound) => {}
        Err(e) => {
            println!("Unable to clear {}. {e}", dest.display());
            return true;
        }
    }

    false
}

fn process_stylesheet(dest: &Path, entry: Result<std::fs::DirEntry, io::Error>) -> bool {
    let entry = match entry {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("Unable to read an entry from {STYLESHEETS}. {e}");
            return false;
        }
    };

    let path = &entry.path();
    let code = match std::fs::read_to_string(path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Unable to read {}. {e}", path.display());
            return false;
        }
    };

    let minified = match StyleSheet::parse(
        &code,
        ParserOptions {
            filename: path.display().to_string(),
            ..Default::default()
        },
    ) {
        Ok(mut stylesheet) => {
            let targets = Targets {
                include: Features::all(),
                ..Targets::default()
            };
            if let Err(e) = stylesheet.minify(MinifyOptions {
                targets,
                ..MinifyOptions::default()
            }) {
                eprintln!("Unable to mininify stylesheet to browser targets. {e}");
            }
            match stylesheet.to_css(PrinterOptions {
                minify: true,
                targets,
                ..Default::default()
            }) {
                Ok(result) => Some(result.code),
                Err(e) => {
                    eprintln!("Unable to serialize minified stylesheet. {e}");
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("Unable to parse stylesheet. {e}");
            None
        }
    };

    let dest = &dest.join(path);

    if let Some(parent) = dest.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        eprintln!(
            "Unable to create parent directories for {}. {e}",
            dest.display()
        );
    }

    match minified {
        Some(code) => {
            if let Err(e) = std::fs::write(dest, code) {
                eprintln!("Unable to write minification into {}. {e}", dest.display());
            } else {
                eprintln!("Created {}", dest.display());
            }
        }
        None => {
            if let Err(e) = std::fs::copy(path, dest) {
                eprintln!(
                    "Unable to copy {} into {}. {e}",
                    path.display(),
                    dest.display()
                );
            } else {
                eprintln!("Copied {} into {}", path.display(), dest.display());
            }
        }
    }

    true
}
