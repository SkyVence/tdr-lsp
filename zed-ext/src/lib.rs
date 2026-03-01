mod lsp;

use zed_extension_api as zed;

struct TdrSceneExtension;

impl TdrSceneExtension {
    const SERVER_BINARY_NAME: &'static str = "tdr-lsp";
}

impl zed::Extension for TdrSceneExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if language_server_id.as_ref() != "tdr_lsp" {
            return Err("unsupported language server".to_string());
        }

        lsp::resolve_language_server(worktree, Self::SERVER_BINARY_NAME)
    }
}

zed::register_extension!(TdrSceneExtension);
