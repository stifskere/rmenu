use std::collections::HashSet;
use std::env::{split_paths, var_os};
use std::ffi::OsString;
use std::fs::read_dir;
use std::io::Error as IoError;
use std::process::Command;

use log::info;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathError {
    #[error("A problem occurred while reading files from a path.")]
    IoError(#[from] IoError),
}

pub fn get_nushell_path() -> Option<OsString> {
    let output = Command::new("nu")
        .args(&["--commands", r#"echo ($env.PATH | str join ":")"#])
        .output()
        .ok()?; // if nu is not installed simply do nothing about it.

    if output.status.success() {
        let path_str = String::from_utf8_lossy(&output.stdout)
            .to_string();

        info!("Nushell found, sourcing $env.PATH");

        if !path_str.is_empty() {
            return Some(OsString::from(path_str));
        }
    }

    None
}

pub fn get_path_programs() -> Result<HashSet<String>, PathError> {
    let mut programs = HashSet::new();

    let mut paths = HashSet::new();

    if let Some(system_path) = var_os("PATH") {
        paths.extend(split_paths(&system_path));
    }

    if let Some(nu_path) = get_nushell_path() {
        paths.extend(split_paths(&nu_path));
    }

    for path in paths {
        if let Ok(entries) = read_dir(&path) {
            for entry in entries.flatten() {
                if entry
                    .path()
                    .is_file()
                {
                    let metadata = entry.metadata()?;

                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;

                        if metadata
                            .permissions()
                            .mode()
                            & 0o111
                            != 0
                        {
                            if let Some(file_name) = entry
                                .path()
                                .file_name()
                                .and_then(|n| n.to_str())
                            {
                                programs.insert(file_name.to_string());
                            }
                        }
                    }

                    #[cfg(windows)]
                    {
                        // !!! THIS IS NOT TESTED !!!

                        if let Some(ext) = entry
                            .path()
                            .extension()
                            .and_then(|e| e.to_str())
                        {
                            if ["exe", "bat", "cmd"].contains(
                                &ext.to_lowercase()
                                    .as_str(),
                            ) {
                                if let Some(file_name) = path
                                    .file_stem()
                                    .and_then(|n| n.to_str())
                                {
                                    programs.insert(file_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    info!("Loaded {} PATH entries", programs.len());

    Ok(programs)
}
