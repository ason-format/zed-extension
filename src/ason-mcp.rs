use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@ason-format/mcp-server";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/@ason-format/mcp-server/dist/index.js";

#[derive(Debug, Deserialize, JsonSchema)]
struct AsonMcpExtensionSettings {
    /// Indentation level for output (0 = no indentation, 1 = compact, 2+ = pretty print)
    #[serde(default = "default_indent")]
    indent: u8,

    /// Field delimiter (pipe "|" is default for better token efficiency)
    #[serde(default = "default_delimiter")]
    delimiter: String,

    /// Enable $var references to avoid duplication
    #[serde(default = "default_use_references")]
    use_references: bool,

    /// Enable @section organization for objects
    #[serde(default = "default_use_sections")]
    use_sections: bool,

    /// Enable key:[N]{fields} tabular arrays for uniform data
    #[serde(default = "default_use_tabular")]
    use_tabular: bool,
}

fn default_indent() -> u8 {
    1
}

fn default_delimiter() -> String {
    "|".to_string()
}

fn default_use_references() -> bool {
    true
}

fn default_use_sections() -> bool {
    true
}

fn default_use_tabular() -> bool {
    true
}

impl Default for AsonMcpExtensionSettings {
    fn default() -> Self {
        Self {
            indent: default_indent(),
            delimiter: default_delimiter(),
            use_references: default_use_references(),
            use_sections: default_use_sections(),
            use_tabular: default_use_tabular(),
        }
    }
}

struct AsonExtension;

impl zed::Extension for AsonExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("ason-mcp/ason", project)?;
        let settings_struct: AsonMcpExtensionSettings = match settings.settings {
            Some(v) => serde_json::from_value(v).map_err(|e| e.to_string())?,
            None => AsonMcpExtensionSettings::default(),
        };

        let args = vec![env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string()];

        // Pass configuration as environment variables to MCP server
        let env = vec![
            (
                "ASON_INDENT".to_string(),
                settings_struct.indent.to_string(),
            ),
            ("ASON_DELIMITER".to_string(), settings_struct.delimiter),
            (
                "ASON_USE_REFERENCES".to_string(),
                settings_struct.use_references.to_string(),
            ),
            (
                "ASON_USE_SECTIONS".to_string(),
                settings_struct.use_sections.to_string(),
            ),
            (
                "ASON_USE_TABULAR".to_string(),
                settings_struct.use_tabular.to_string(),
            ),
        ];

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("ason-mcp/ason", project);

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(ason_settings) =
                    serde_json::from_value::<AsonMcpExtensionSettings>(settings_value)
                {
                    // Replace placeholders with actual values
                    default_settings = default_settings
                        .replace(
                            "\"indent\": 1",
                            &format!("\"indent\": {}", ason_settings.indent),
                        )
                        .replace(
                            "\"delimiter\": \"|\"",
                            &format!("\"delimiter\": \"{}\"", ason_settings.delimiter),
                        )
                        .replace(
                            "\"use_references\": true",
                            &format!("\"use_references\": {}", ason_settings.use_references),
                        )
                        .replace(
                            "\"use_sections\": true",
                            &format!("\"use_sections\": {}", ason_settings.use_sections),
                        )
                        .replace(
                            "\"use_tabular\": true",
                            &format!("\"use_tabular\": {}", ason_settings.use_tabular),
                        );
                }
            }
        }

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(AsonMcpExtensionSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(AsonExtension);
