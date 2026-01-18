use zed_extension_api::{self as zed, Result};

struct IgnisExtension;

impl zed::Extension for IgnisExtension {
    fn new() -> Self {
        IgnisExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let env = worktree.shell_env();

        let command = env
            .iter()
            .find(|(k, _)| k == "IGNIS_LSP_PATH")
            .map(|(_, v)| v.clone())
            .or_else(|| worktree.which("ignis"))
            .ok_or(
                "ignis command not found in PATH. Install ignis or set IGNIS_LSP_PATH".to_string(),
            )?;

        Ok(zed::Command {
            command,
            args: vec!["lsp".to_string()],
            env,
        })
    }
}

zed::register_extension!(IgnisExtension);
