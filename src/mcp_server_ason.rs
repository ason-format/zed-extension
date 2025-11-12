use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "@ason-format/mcp-server";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/@ason-format/mcp-server/dist/index.js";

#[derive(Debug, Deserialize, JsonSchema)]

struct AsonExtension;

impl zed::Extension for AsonExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let args = vec![env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string()];

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(AsonExtension);
