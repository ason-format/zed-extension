use zed_extension_api::{self as zed, ContextServerId, Result};

struct AsonExtension;

impl zed::Extension for AsonExtension {
    fn new() -> Self {
        AsonExtension
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &zed::Project,
    ) -> Result<zed::Command> {
        // Use npx to run the latest version of @ason-format/mcp-server
        // This ensures users always get the latest updates without manual installation
        Ok(zed::Command {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "@ason-format/mcp-server@latest".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AsonExtension);
