use std::env;
use std::path::PathBuf;

// FIXME: make the case analysis here complete and rigorous

#[cfg(target_os = "macos")]
pub const OS: &'static str = "darwin";

#[cfg(target_os = "linux")]
pub const OS: &'static str = "linux";

#[cfg(target_arch = "x86")]
pub const ARCH: &'static str = "x86";

#[cfg(target_arch = "x86_64")]
pub const ARCH: &'static str = "x64";

// ~/
//     .nodeup/
//         cache/                                          cache_dir
//             node/                                       node_cache_dir
//                 node-dist-v4.8.4-linux-x64.tar.gz       archive_file("4.8.4")
//                 node-dist-v6.11.3-linux-x64.tar.gz
//                 node-dist-v8.6.0-linux-x64.tar.gz
//                 ...
//         versions/                                       versions_dir
//             node/                                       node_versions_dir
//                 4.8.4/                                  node_version_dir("4.8.4")
//                   bin/                                  node_version_bin_dir("4.8.4")
//                 6.11.3/
//                 8.6.0/
//                 ...
//         bin/                                            bin_dir
//             nodeup                                      nodeup_file
//         toolchain/                                      toolchain_dir
//             node                                        toolchain_file("node")
//             npm
//             npx
//             ...
//         launchbin                                       launchbin_file
//         launchscript                                    launchscript_file
//         config.toml                                     user_config_file

fn nodeup_home() -> ::Result<PathBuf> {
    let home = env::home_dir().ok_or_else(|| { ::ErrorKind::UnknownSystemFolder(String::from("HOME")) })?;
    Ok(home.join(".nodeup"))
}

pub fn cache_dir() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("cache"))
}

pub fn node_cache_dir() -> ::Result<PathBuf> {
    Ok(cache_dir()?.join("node"))
}

pub fn archive_extension() -> String {
    String::from("tar.gz")
}

pub fn versions_dir() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("versions"))
}

pub fn node_versions_dir() -> ::Result<PathBuf> {
    Ok(versions_dir()?.join("node"))
}

pub fn node_version_dir(version: &str) -> ::Result<PathBuf> {
    Ok(node_versions_dir()?.join(version))
}

pub fn node_version_bin_dir(version: &str) -> ::Result<PathBuf> {
    Ok(node_version_dir(version)?.join("bin"))
}

pub fn bin_dir() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("bin"))
}

pub fn nodeup_file() -> ::Result<PathBuf> {
    Ok(bin_dir()?.join("nodeup"))
}

pub fn toolchain_dir() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("toolchain"))
}

pub fn toolchain_file(toolname: &str) -> ::Result<PathBuf> {
    Ok(toolchain_dir()?.join(toolname))
}

pub fn launchbin_file() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("launchbin"))
}

pub fn launchscript_file() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("launchscript"))
}

pub fn user_config_file() -> ::Result<PathBuf> {
    Ok(nodeup_home()?.join("config.toml"))
}
