use std::path::PathBuf;
use zed_extension_api::{self as zed, GithubReleaseOptions};

const LSP_REPO: &str = "skyvence/tdr-lsp";

pub fn resolve_language_server(
    worktree: &zed::Worktree,
    binary_name: &str,
) -> zed::Result<zed::Command> {
    let (os, arch) = zed::current_platform();
    let binary_name = get_binary_name(binary_name, os);
    let arch_dir = get_arch_dir(os, arch)?;

    if let Some(path) = worktree.which(&binary_name) {
        return Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        });
    }

    let binary_path = find_local_binary(worktree, &binary_name, &arch_dir);
    if let Some(path) = binary_path {
        if std::path::Path::new(&path).exists() {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: vec![],
            });
        }
    }

    download_and_cache(&binary_name, &arch_dir, os)
}

fn get_binary_name(binary_name: &str, os: zed::Os) -> String {
    if matches!(os, zed::Os::Windows) {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    }
}

fn get_arch_dir(os: zed::Os, arch: zed::Architecture) -> zed::Result<String> {
    match (os, arch) {
        (zed::Os::Windows, zed::Architecture::X8664) => Ok("win32-x64".to_string()),
        (zed::Os::Linux, zed::Architecture::X8664) => Ok("linux-x64".to_string()),
        (zed::Os::Mac, zed::Architecture::X8664) => Ok("darwin-x64".to_string()),
        (zed::Os::Mac, zed::Architecture::Aarch64) => Ok("darwin-aarch64".to_string()),
        (zed::Os::Linux, zed::Architecture::Aarch64) => Ok("linux-aarch64".to_string()),
        (zed::Os::Windows, zed::Architecture::Aarch64) => Ok("win32-aarch64".to_string()),
        _ => Err(format!("unsupported platform: {:?}", (os, arch))),
    }
}

fn find_local_binary(
    worktree: &zed::Worktree,
    binary_name: &str,
    arch_dir: &str,
) -> Option<String> {
    let root = worktree.root_path();
    let search_path = PathBuf::from(&root)
        .join("bin")
        .join(arch_dir)
        .join(binary_name);

    if search_path.exists() {
        return Some(search_path.to_string_lossy().into_owned());
    }
    None
}

fn download_and_cache(binary_name: &str, arch_dir: &str, os: zed::Os) -> zed::Result<zed::Command> {
    let release = zed::latest_github_release(
        LSP_REPO,
        GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )
    .map_err(|e| format!("Failed to get latest release: {}", e))?;
    println!("Latest release: {}", release.version);

    // Try both "arch_dir/binary" and "binary" asset name formats so releases
    // that don't include the arch directory prefix still work.
    let asset_name_with_dir = format!("{}/{}", arch_dir, binary_name);
    let asset_name_no_dir = binary_name.to_string();
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name_with_dir || a.name == asset_name_no_dir)
        .ok_or_else(|| {
            format!(
                "Asset '{}' or '{}' not found in latest release. Available: {:?}",
                asset_name_with_dir,
                asset_name_no_dir,
                release.assets.iter().map(|a| &a.name).collect::<Vec<_>>()
            )
        })?;
    println!("Selected asset: {}", asset.name);

    let dest_dir = format!("bin/{}", arch_dir);
    let dest_path = format!("{}/{}", dest_dir, binary_name);

    zed::download_file(
        &asset.download_url,
        &dest_path,
        zed::DownloadedFileType::Uncompressed,
    )
    .map_err(|e| format!("Failed to download LSP: {}", e))?;

    if matches!(os, zed::Os::Windows) {
        zed::make_file_executable(&dest_path)
            .map_err(|e| format!("Failed to set executable bit on '{}': {}", dest_path, e))?;
    }

    Ok(zed::Command {
        command: dest_path,
        args: vec![],
        env: vec![],
    })
}
