use std::path::PathBuf;
use zed_extension_api as zed;

const LSP_REPO_OWNER: &str = "SkyVence";
const LSP_REPO_NAME: &str = "traceurs-de-rayons";
const LSP_BINARY_NAME: &str = "tdr-lsp";

pub fn resolve_language_server(worktree: &zed::Worktree) -> zed::Result<zed::Command> {
    let binary_name = get_binary_name();
    let (os, arch) = zed::current_platform();
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

    download_and_cache(worktree, &binary_name, &arch_dir)
}

fn get_binary_name() -> String {
    if cfg!(target_os = "windows") {
        format!("{}.exe", LSP_BINARY_NAME)
    } else {
        LSP_BINARY_NAME.to_string()
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
    let search_paths = [
        PathBuf::from(&root)
            .join("tdr-lsp")
            .join("zed-ext")
            .join("bin")
            .join(arch_dir)
            .join(binary_name),
        PathBuf::from(&root)
            .join("zed-ext")
            .join("bin")
            .join(arch_dir)
            .join(binary_name),
        PathBuf::from(&root)
            .join("tdr-lsp")
            .join("bin")
            .join(arch_dir)
            .join(binary_name),
        PathBuf::from(&root)
            .join("bin")
            .join(arch_dir)
            .join(binary_name),
    ];

    for path in &search_paths {
        if path.exists() {
            return Some(path.to_string_lossy().into_owned());
        }
    }
    None
}

fn download_and_cache(
    worktree: &zed::Worktree,
    binary_name: &str,
    arch_dir: &str,
) -> zed::Result<zed::Command> {
    let release = zed::latest_github_release(LSP_REPO_OWNER, LSP_REPO_NAME)
        .map_err(|e| format!("Failed to get latest release: {}", e))?;

    let asset_name = format!("{}/{}", arch_dir, binary_name);
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| {
            format!(
                "Asset '{}' not found in latest release. Available: {:?}",
                asset_name,
                release.assets.iter().map(|a| &a.name).collect::<Vec<_>>()
            )
        })?;

    let dest_dir = format!("bin/{}", arch_dir);
    let dest_path = format!("{}/{}", dest_dir, binary_name);

    zed::download_file(
        &asset.download_url,
        &dest_path,
        zed::DownloadedFileType::Uncompressed,
    )
    .map_err(|e| format!("Failed to download LSP: {}", e))?;

    if !cfg!(target_os = "windows") {
        let _ = zed::make_file_executable(&dest_path);
    }

    Ok(zed::Command {
        command: dest_path,
        args: vec![],
        env: vec![],
    })
}

    Ok(zed::Command {
        command: dest_path.to_string_lossy().into_owned(),
        args: vec![],
        env: vec![],
    })
}
