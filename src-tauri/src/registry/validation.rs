use super::model::{CommandSpec, ParameterKind, ParameterSpec, RegistryError, ToolProfile};
use std::path::Path;

const MAX_ID_LEN: usize = 64;
const MAX_NAME_LEN: usize = 200;
const MAX_DESCRIPTION_LEN: usize = 4000;
const MAX_TAGS: usize = 16;
const MAX_TAG_LEN: usize = 32;

// Command/parameter metadata limits (schema 1.1)
const MAX_COMMAND_DEPTH: usize = 5;
const MAX_TOTAL_COMMANDS: usize = 200;
const MAX_PARAMS_PER_COMMAND: usize = 100;
const MAX_SUBCOMMANDS: usize = 50;
const MAX_VERSION_CONSTRAINTS: usize = 8;
const MAX_VERSION_CONSTRAINT_LEN: usize = 64;
const MAX_TOKEN_LEN: usize = 120;
const MAX_SUMMARY_LEN: usize = 300;
const MAX_SYNTAX_LEN: usize = 200;
const MAX_EXAMPLES: usize = 16;
const MAX_EXAMPLE_LEN: usize = 200;
const MAX_ALIASES: usize = 8;

const RESERVED_DEVICE_NAMES: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

pub const ALLOWED_EXECUTABLE_EXTENSIONS: [&str; 2] = ["exe", "com"];

/// Validates a tool id for use as a registry filename on all supported
/// platforms. Must be called before any filesystem operation on the id.
pub fn validate_id(id: &str) -> Result<(), RegistryError> {
    if id.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "id must not be empty",
            "id",
        ));
    }
    if id.len() > MAX_ID_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("id must be at most {MAX_ID_LEN} characters"),
            "id",
        ));
    }
    let mut first = true;
    for c in id.chars() {
        let ok = c.is_ascii_lowercase()
            || c.is_ascii_digit()
            || c == '.'
            || c == '_'
            || c == '-';
        if !ok {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "id may contain only lowercase letters, digits, '.', '_' and '-'",
                "id",
            ));
        }
        if first && !(c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "id must start with a lowercase letter or digit",
                "id",
            ));
        }
        first = false;
    }
    if id.ends_with('.') || id.ends_with(' ') {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "id must not end with a dot or space",
            "id",
        ));
    }
    // Windows reserved device names also apply to the base name of
    // "NAME.ext" (e.g. "con.json" is reserved).
    let base = id.split('.').next().unwrap_or(id).to_ascii_uppercase();
    if RESERVED_DEVICE_NAMES.contains(&base.as_str()) {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("id must not use the reserved device name {base}"),
            "id",
        ));
    }
    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), RegistryError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "name must not be empty",
            "name",
        ));
    }
    if trimmed.len() > MAX_NAME_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("name must be at most {MAX_NAME_LEN} characters"),
            "name",
        ));
    }
    Ok(())
}

pub fn validate_description(description: &str) -> Result<(), RegistryError> {
    if description.len() > MAX_DESCRIPTION_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("description must be at most {MAX_DESCRIPTION_LEN} characters"),
            "description",
        ));
    }
    Ok(())
}

pub fn validate_tags(tags: &[String]) -> Result<(), RegistryError> {
    if tags.len() > MAX_TAGS {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {MAX_TAGS} tags are allowed"),
            "tags",
        ));
    }
    for tag in tags {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "tags must not be empty",
                "tags",
            ));
        }
        if trimmed.len() > MAX_TAG_LEN {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!("each tag must be at most {MAX_TAG_LEN} characters"),
                "tags",
            ));
        }
    }
    Ok(())
}

/// Accepts only absolute http(s) URLs, e.g. for documentation links.
pub fn validate_optional_url(url: &str, field: &str) -> Result<(), RegistryError> {
    let lower = url.trim().to_ascii_lowercase();
    let rest = if let Some(r) = lower.strip_prefix("https://") {
        r
    } else if let Some(r) = lower.strip_prefix("http://") {
        r
    } else {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "URL must start with http:// or https://",
            field,
        ));
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    if host.is_empty() || host.contains(' ') {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "URL must contain a valid host",
            field,
        ));
    }
    Ok(())
}

/// Format-level executable path validation (no filesystem access).
/// Existence and file-type checks are performed separately by the service.
pub fn validate_path_format(path: &str) -> Result<(), RegistryError> {
    if path.trim().is_empty() {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path must not be empty",
            "executablePath",
        ));
    }
    if path.contains('\0') {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path contains invalid characters",
            "executablePath",
        ));
    }
    let p = Path::new(path);
    if !p.is_absolute() {
        return Err(RegistryError::with_field(
            "invalid_path",
            "executable path must be an absolute path",
            "executablePath",
        ));
    }
    let ext = p
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    if !ALLOWED_EXECUTABLE_EXTENSIONS.contains(&ext.as_str()) {
        return Err(RegistryError::with_field(
            "invalid_path",
            "only .exe and .com executables are supported",
            "executablePath",
        ));
    }
    Ok(())
}

pub fn validate_schema_version(version: &str) -> Result<(), RegistryError> {
    let major = version.split('.').next().unwrap_or("");
    if major != "1" {
        return Err(RegistryError::new(
            "unsupported_schema_version",
            format!("unsupported profile schemaVersion {version}, expected major version 1"),
        ));
    }
    Ok(())
}

/// Full validation of a stored or incoming profile.
pub fn validate_profile(profile: &ToolProfile) -> Result<(), RegistryError> {
    validate_schema_version(&profile.schema_version)?;
    validate_id(&profile.id)?;
    validate_name(&profile.name)?;
    if let Some(desc) = &profile.description {
        validate_description(desc)?;
    }
    validate_tags(&profile.tags)?;
    validate_path_format(&profile.executable.path)?;
    if profile.executable.name.trim().is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "executable name must not be empty",
            "executable.name",
        ));
    }
    if profile.platforms.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "at least one platform is required",
            "platforms",
        ));
    }
    if let Some(url) = &profile.documentation_url {
        validate_optional_url(url, "documentationUrl")?;
    }
    if let Some(author) = &profile.author {
        if author.name.trim().is_empty() {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "author name must not be empty",
                "author.name",
            ));
        }
        if let Some(url) = &author.url {
            validate_optional_url(url, "author.url")?;
        }
    }
    if profile.created_at.trim().is_empty() || profile.updated_at.trim().is_empty() {
        return Err(RegistryError::new(
            "invalid_profile",
            "createdAt and updatedAt must not be empty",
        ));
    }
    if let Some(commands) = &profile.commands {
        validate_commands(commands)?;
    }
    Ok(())
}

/// Repaths a validator error to a nested field path (e.g.
/// `commands[1].parameters[0].id`).
fn repath(err: RegistryError, path: &str) -> RegistryError {
    match err.field {
        Some(field) => RegistryError::with_field(&err.code, err.message, &format!("{path}.{field}")),
        None => RegistryError::with_field(&err.code, err.message, path),
    }
}

fn validate_token_str(token: &str, path: &str) -> Result<(), RegistryError> {
    if token.trim().is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "token must not be empty",
            path,
        ));
    }
    if token.len() > MAX_TOKEN_LEN {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("token must be at most {MAX_TOKEN_LEN} characters"),
            path,
        ));
    }
    if token.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "token must not contain whitespace or control characters",
            path,
        ));
    }
    Ok(())
}

fn validate_version_constraints(items: &[String], path: &str) -> Result<(), RegistryError> {
    if items.len() > MAX_VERSION_CONSTRAINTS {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {MAX_VERSION_CONSTRAINTS} version constraints are allowed"),
            path,
        ));
    }
    for item in items {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "version constraints must not be empty",
                path,
            ));
        }
        if trimmed.len() > MAX_VERSION_CONSTRAINT_LEN {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!(
                    "version constraints must be at most {MAX_VERSION_CONSTRAINT_LEN} characters"
                ),
                path,
            ));
        }
    }
    Ok(())
}

fn validate_string_list(
    items: &[String],
    max_items: usize,
    max_len: usize,
    path: &str,
) -> Result<(), RegistryError> {
    if items.len() > max_items {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {max_items} entries are allowed"),
            path,
        ));
    }
    for item in items {
        if item.trim().is_empty() {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "entries must not be empty",
                path,
            ));
        }
        if item.len() > max_len {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!("entries must be at most {max_len} characters"),
                path,
            ));
        }
    }
    Ok(())
}

/// Validates a command tree (schema 1.1 metadata). `path` is the field path
/// prefix used in errors (e.g. `commands[1].subcommands[0]`).
pub fn validate_commands(commands: &[CommandSpec]) -> Result<(), RegistryError> {
    if commands.len() > MAX_TOTAL_COMMANDS {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {MAX_TOTAL_COMMANDS} commands are allowed"),
            "commands",
        ));
    }
    let direct_count = commands.iter().filter(|c| c.token.is_none()).count();
    if direct_count > 1 {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "at most one top-level command may use a null token (direct mode)",
            "commands",
        ));
    }
    let mut command_ids: Vec<String> = Vec::new();
    let mut sibling_tokens: Vec<String> = Vec::new();
    for (index, command) in commands.iter().enumerate() {
        let path = format!("commands[{index}]");
        if let Some(token) = &command.token {
            if sibling_tokens.contains(token) {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    format!("duplicate sibling command token `{token}`"),
                    &format!("{path}.token"),
                ));
            }
            sibling_tokens.push(token.clone());
        }
        validate_command(command, 1, &path, &mut command_ids)?;
    }
    Ok(())
}

fn validate_command(
    command: &CommandSpec,
    depth: usize,
    path: &str,
    command_ids: &mut Vec<String>,
) -> Result<(), RegistryError> {
    if depth > MAX_COMMAND_DEPTH {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("command nesting exceeds the maximum depth of {MAX_COMMAND_DEPTH}"),
            path,
        ));
    }
    if command_ids.contains(&command.id) {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("duplicate command id `{}`", command.id),
            &format!("{path}.id"),
        ));
    }
    command_ids.push(command.id.clone());
    validate_id(&command.id).map_err(|e| repath(e, &format!("{path}.id")))?;
    validate_name(&command.name).map_err(|e| repath(e, &format!("{path}.name")))?;
    if let Some(summary) = &command.summary {
        if summary.len() > MAX_SUMMARY_LEN {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!("summary must be at most {MAX_SUMMARY_LEN} characters"),
                &format!("{path}.summary"),
            ));
        }
    }
    if let Some(description) = &command.description {
        validate_description(description)
            .map_err(|e| repath(e, &format!("{path}.description")))?;
    }
    validate_string_list(
        &command.syntax_examples,
        MAX_EXAMPLES,
        MAX_SYNTAX_LEN,
        &format!("{path}.syntaxExamples"),
    )?;
    validate_version_constraints(&command.version_constraints, &format!("{path}.versionConstraints"))?;
    match &command.token {
        None => {
            if depth > 1 {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "nested commands require a non-null token",
                    &format!("{path}.token"),
                ));
            }
        }
        Some(token) => validate_token_str(token, &format!("{path}.token"))?,
    }

    if command.parameters.len() > MAX_PARAMS_PER_COMMAND {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {MAX_PARAMS_PER_COMMAND} parameters are allowed per command"),
            &format!("{path}.parameters"),
        ));
    }
    let mut param_ids: Vec<String> = Vec::new();
    let mut param_tokens: Vec<String> = Vec::new();
    let mut positional_orders: Vec<i64> = Vec::new();
    for (index, parameter) in command.parameters.iter().enumerate() {
        let param_path = format!("{path}.parameters[{index}]");
        if param_ids.contains(&parameter.id) {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!("duplicate parameter id `{}`", parameter.id),
                &format!("{param_path}.id"),
            ));
        }
        param_ids.push(parameter.id.clone());
        if let Some(token) = &parameter.token {
            if param_tokens.contains(token) {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    format!("duplicate parameter token `{token}` in command"),
                    &format!("{param_path}.token"),
                ));
            }
            param_tokens.push(token.clone());
        }
        if let Some(aliases) = &parameter.aliases {
            for alias in aliases {
                if param_tokens.contains(alias) {
                    return Err(RegistryError::with_field(
                        "invalid_profile",
                        format!("duplicate parameter token/alias `{alias}` in command"),
                        &format!("{param_path}.aliases"),
                    ));
                }
                param_tokens.push(alias.clone());
            }
        }
        if parameter.kind == ParameterKind::Positional {
            match parameter.order {
                Some(order) => {
                    if order < 1 {
                        return Err(RegistryError::with_field(
                            "invalid_profile",
                            "positional order must be a positive integer",
                            &format!("{param_path}.order"),
                        ));
                    }
                    if positional_orders.contains(&order) {
                        return Err(RegistryError::with_field(
                            "invalid_profile",
                            format!("duplicate positional order {order}"),
                            &format!("{param_path}.order"),
                        ));
                    }
                    positional_orders.push(order);
                }
                None => {
                    return Err(RegistryError::with_field(
                        "invalid_profile",
                        "positional parameters require an order",
                        &format!("{param_path}.order"),
                    ));
                }
            }
        }
        validate_parameter(parameter, &param_path)?;
    }

    // Cross-reference integrity: requires/conflicts must name sibling parameters.
    for (index, parameter) in command.parameters.iter().enumerate() {
        let param_path = format!("{path}.parameters[{index}]");
        for referenced in &parameter.requires {
            if referenced == &parameter.id {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "a parameter cannot require itself",
                    &format!("{param_path}.requires"),
                ));
            }
            if parameter.conflicts.contains(referenced) {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    format!("`{referenced}` cannot be both required and conflicting"),
                    &format!("{param_path}.requires"),
                ));
            }
            if !command.parameters.iter().any(|p| p.id == *referenced) {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    format!("unknown parameter reference `{referenced}`"),
                    &format!("{param_path}.requires"),
                ));
            }
        }
        for referenced in &parameter.conflicts {
            if referenced == &parameter.id {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "a parameter cannot conflict with itself",
                    &format!("{param_path}.conflicts"),
                ));
            }
            if !command.parameters.iter().any(|p| p.id == *referenced) {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    format!("unknown parameter reference `{referenced}`"),
                    &format!("{param_path}.conflicts"),
                ));
            }
        }
    }

    if command.subcommands.len() > MAX_SUBCOMMANDS {
        return Err(RegistryError::with_field(
            "invalid_profile",
            format!("at most {MAX_SUBCOMMANDS} subcommands are allowed per command"),
            &format!("{path}.subcommands"),
        ));
    }
    let mut sub_tokens: Vec<String> = Vec::new();
    for (index, subcommand) in command.subcommands.iter().enumerate() {
        let sub_path = format!("{path}.subcommands[{index}]");
        if let Some(token) = &subcommand.token {
            if sub_tokens.contains(token) {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    format!("duplicate sibling command token `{token}`"),
                    &format!("{sub_path}.token"),
                ));
            }
            sub_tokens.push(token.clone());
        }
        validate_command(subcommand, depth + 1, &sub_path, command_ids)?;
    }
    Ok(())
}

fn validate_parameter(parameter: &ParameterSpec, path: &str) -> Result<(), RegistryError> {
    validate_id(&parameter.id).map_err(|e| repath(e, &format!("{path}.id")))?;
    validate_name(&parameter.label).map_err(|e| repath(e, &format!("{path}.label")))?;
    if let Some(token) = &parameter.token {
        validate_token_str(token, &format!("{path}.token"))?;
    }
    if let Some(aliases) = &parameter.aliases {
        validate_string_list(
            aliases,
            MAX_ALIASES,
            MAX_TOKEN_LEN,
            &format!("{path}.aliases"),
        )?;
    }
    if let Some(description) = &parameter.description {
        validate_description(description).map_err(|e| repath(e, &format!("{path}.description")))?;
    }
    if let Some(syntax) = &parameter.syntax {
        if syntax.len() > MAX_SYNTAX_LEN {
            return Err(RegistryError::with_field(
                "invalid_profile",
                format!("syntax must be at most {MAX_SYNTAX_LEN} characters"),
                &format!("{path}.syntax"),
            ));
        }
    }
    validate_string_list(
        &parameter.examples,
        MAX_EXAMPLES,
        MAX_EXAMPLE_LEN,
        &format!("{path}.examples"),
    )?;
    validate_version_constraints(
        &parameter.version_constraints,
        &format!("{path}.versionConstraints"),
    )?;

    match parameter.kind {
        ParameterKind::Flag => {
            if parameter.token.is_none() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "flag parameters require a token",
                    &format!("{path}.token"),
                ));
            }
            if parameter.value.is_some() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "boolean flags must not declare a value",
                    &format!("{path}.value"),
                ));
            }
        }
        ParameterKind::Option => {
            if parameter.token.is_none() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "option parameters require a token",
                    &format!("{path}.token"),
                ));
            }
            if parameter.value.is_none() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "option parameters require a value specification",
                    &format!("{path}.value"),
                ));
            }
        }
        ParameterKind::Positional => {
            if parameter.token.is_some() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "positional parameters must not declare a token",
                    &format!("{path}.token"),
                ));
            }
            if parameter.value.is_none() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "positional parameters require a value specification",
                    &format!("{path}.value"),
                ));
            }
            if parameter.repeatable {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "positional parameters must not be repeatable; use value.multiple",
                    &format!("{path}.repeatable"),
                ));
            }
        }
        ParameterKind::Custom => {
            // Closed contract, stage 2: known-but-forbidden fields reach
            // semantic validation (stage 1 handled unknown fields).
            if parameter.value.is_some() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "field `value` is not allowed for kind \"custom\"",
                    &format!("{path}.value"),
                ));
            }
            if parameter.aliases.is_some() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "field `aliases` is not allowed for kind \"custom\"",
                    &format!("{path}.aliases"),
                ));
            }
            if parameter.order.is_some() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "field `order` is not allowed for kind \"custom\"",
                    &format!("{path}.order"),
                ));
            }
            if parameter.syntax.is_none() && parameter.description.is_none() {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "custom parameters require a syntax or description",
                    &format!("{path}.syntax"),
                ));
            }
        }
    }

    if let Some(value) = &parameter.value {
        let value_path = format!("{path}.value");
        match value.value_type {
            super::model::ValueType::Enum => {
                if value.choices.is_empty() {
                    return Err(RegistryError::with_field(
                        "invalid_profile",
                        "enum values require at least one choice",
                        &format!("{value_path}.choices"),
                    ));
                }
                let mut seen: Vec<&String> = Vec::new();
                for choice in &value.choices {
                    if seen.contains(&choice) {
                        return Err(RegistryError::with_field(
                            "invalid_profile",
                            format!("duplicate choice `{choice}`"),
                            &format!("{value_path}.choices"),
                        ));
                    }
                    seen.push(choice);
                }
            }
            _ => {
                if !value.choices.is_empty() {
                    return Err(RegistryError::with_field(
                        "invalid_profile",
                        "choices are only allowed when valueType is enum",
                        &format!("{value_path}.choices"),
                    ));
                }
            }
        }
        if value.optional && parameter.kind != ParameterKind::Option {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "value.optional is only allowed on option parameters",
                &format!("{value_path}.optional"),
            ));
        }
        if value.multiple && value.optional {
            return Err(RegistryError::with_field(
                "invalid_profile",
                "value.multiple and value.optional are mutually exclusive",
                &format!("{value_path}.multiple"),
            ));
        }
        if let Some(default) = &value.default {
            if default.len() > 200 {
                return Err(RegistryError::with_field(
                    "invalid_profile",
                    "default must be at most 200 characters",
                    &format!("{value_path}.default"),
                ));
            }
        }
    }
    Ok(())
}

/// Derives a filesystem-safe slug from a tool name. Returns an error if no
/// valid id can be derived.
pub fn derive_id(name: &str) -> Result<String, RegistryError> {
    let mut slug = String::new();
    let mut last_dash = false;
    for c in name.trim().chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        return Err(RegistryError::with_field(
            "invalid_profile",
            "cannot derive a valid id from the given name; provide an explicit id",
            "id",
        ));
    }
    validate_id(&slug)?;
    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::model::*;

    fn valid_profile() -> ToolProfile {
        ToolProfile {
            schema_version: SCHEMA_VERSION.to_string(),
            id: "nmap".to_string(),
            name: "Nmap".to_string(),
            description: Some("Network scanner".to_string()),
            category: Category::Scanning,
            tags: vec!["network".to_string()],
            integration_type: IntegrationType::Cli,
            executable: ExecutableInfo {
                name: "nmap.exe".to_string(),
                path: "C:\\Tools\\nmap.exe".to_string(),
            },
            version: None,
            discovery: DiscoveryInfo {
                source: DiscoverySource::Manual,
                discovered_at: "2026-11-03T00:00:00Z".to_string(),
            },
            verification: VerificationInfo {
                status: VerificationStatus::Unverified,
                source: "manual-import".to_string(),
                tool_version: None,
            },
            platforms: vec![Platform::Windows],
            commands: None,
            documentation_url: Some("https://nmap.org".to_string()),
            author: None,
            created_at: "2026-11-03T00:00:00Z".to_string(),
            updated_at: "2026-11-03T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn ids_accept_valid_forms() {
        for id in ["nmap", "nmap2", "wireshark-ce", "hashcat_2", "tool.v2", "a"] {
            validate_id(id).unwrap_or_else(|e| panic!("{id} should be valid: {e}"));
        }
    }

    #[test]
    fn ids_reject_invalid_forms() {
        for id in [
            "",              // empty
            ".hidden",       // leading dot
            "-lead",         // leading dash
            "Upper",         // uppercase
            "has space",     // space
            "has/slash",     // separator
            "has\\back",     // separator
            "trailing.",     // trailing dot
            "a..b..",        // trailing dot
            "col:on",        // invalid char
        ] {
            let id: String = id.to_string();
            validate_id(&id).expect_err(&format!("{id} should be rejected"));
        }
        validate_id(&"x".repeat(65)).expect_err("too long");
    }

    #[test]
    fn ids_reject_windows_reserved_device_names() {
        for id in [
            "con", "prn", "aux", "nul", "com1", "com9", "lpt1", "lpt9",
            "con.json", // reserved base name with extension-like id
        ] {
            validate_id(id).expect_err(&format!("{id} should be rejected"));
        }
        // com0/lpt0 and com10 are not reserved
        for id in ["com0", "lpt0", "com10", "console"] {
            validate_id(id).unwrap_or_else(|e| panic!("{id} should be valid: {e}"));
        }
    }

    #[test]
    fn name_rules() {
        validate_name("Nmap").unwrap();
        validate_name("  Nmap  ").unwrap();
        validate_name("").expect_err("empty name");
        validate_name("   ").expect_err("blank name");
        validate_name(&"x".repeat(201)).expect_err("too long");
    }

    #[test]
    fn description_and_tag_rules() {
        validate_description(&"d".repeat(4000)).unwrap();
        validate_description(&"d".repeat(4001)).expect_err("too long");
        validate_tags(&[]).unwrap();
        validate_tags(&vec!["a".to_string(); 16]).unwrap();
        validate_tags(&vec!["a".to_string(); 17]).expect_err("too many");
        validate_tags(&["".to_string()]).expect_err("empty tag");
        validate_tags(&["x".repeat(33)]).expect_err("tag too long");
    }

    #[test]
    fn url_rules() {
        validate_optional_url("https://nmap.org/book", "u").unwrap();
        validate_optional_url("http://localhost:8080/docs", "u").unwrap();
        validate_optional_url("javascript:alert(1)", "u").expect_err("js scheme");
        validate_optional_url("file:///etc/passwd", "u").expect_err("file scheme");
        validate_optional_url("https://", "u").expect_err("no host");
        validate_optional_url("nmap.org", "u").expect_err("no scheme");
    }

    #[test]
    fn path_format_rules() {
        validate_path_format("C:\\Tools\\nmap.exe").unwrap();
        validate_path_format("C:\\Tools\\nmap.EXE").unwrap(); // case-insensitive ext
        validate_path_format("C:\\Tools\\cmd.com").unwrap();
        validate_path_format("C:\\Tools\\nmap.bat").expect_err("bat unsupported");
        validate_path_format("C:\\Tools\\run.ps1").expect_err("ps1 unsupported");
        validate_path_format("nmap.exe").expect_err("relative path");
        validate_path_format("").expect_err("empty");
    }

    #[test]
    fn schema_version_must_match() {
        validate_schema_version("1.0").unwrap();
        validate_schema_version("2.0").expect_err("future major");
    }

    #[test]
    fn full_profile_validation() {
        let mut p = valid_profile();
        validate_profile(&p).unwrap();
        p.id = "CON".to_string();
        validate_profile(&p).expect_err("reserved id");
    }

    #[test]
    fn derive_id_slugs_names() {
        assert_eq!(derive_id("Nmap").unwrap(), "nmap");
        assert_eq!(derive_id("John the Ripper").unwrap(), "john-the-ripper");
        assert_eq!(derive_id("  SQLMap! ").unwrap(), "sqlmap");
        assert_eq!(derive_id("Wireshark (GUI)").unwrap(), "wireshark-gui");
        derive_id("!!!").expect_err("no alnum");
    }

    #[test]
    fn canonical_profile_serialization_uses_expected_keys() {
        // Anti-drift guard between Rust model and schemas/tool-profile.schema.json:
        // if a field is renamed here, the published schema must be updated too.
        let value = serde_json::to_value(valid_profile()).unwrap();
        let obj = value.as_object().unwrap();
        let mut keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            vec![
                "category",
                "createdAt",
                "description",
                "discovery",
                "documentationUrl",
                "executable",
                "id",
                "integrationType",
                "name",
                "platforms",
                "schemaVersion",
                "tags",
                "updatedAt",
                "verification",
            ]
        );
    }

    #[test]
    fn category_values_match_product_specification() {
        let names: Vec<&str> = Category::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "Reconnaissance",
                "Scanning",
                "Web Application Security",
                "Exploitation",
                "Password Security Testing",
                "Wireless Security",
                "Network Security",
                "Privilege Escalation",
                "Post-Exploitation",
                "Active Directory and Windows Security",
                "Reverse Engineering",
                "Malware Analysis",
                "Digital Forensics",
                "Social Engineering",
                "Defensive Security",
                "other",
            ]
        );
    }

    // ---- command/parameter metadata validation (schema 1.1) ----

    fn cmd(json: &str) -> CommandSpec {
        serde_json::from_str(json).expect("test command must deserialize")
    }

    fn check(json: &str) -> RegistryError {
        let command = cmd(json);
        validate_commands(std::slice::from_ref(&command)).expect_err("should be invalid")
    }

    fn param_check(param: &str) -> RegistryError {
        check(&format!(
            r#"{{"id":"c","token":null,"name":"C","parameters":[{param}],"subcommands":[]}}"#
        ))
    }

    fn param_ok(param: &str) {
        let command = cmd(&format!(
            r#"{{"id":"c","token":null,"name":"C","parameters":[{param}],"subcommands":[]}}"#
        ));
        validate_commands(std::slice::from_ref(&command)).unwrap();
    }

    #[test]
    fn s1_flag_with_value_is_rejected() {
        let err = param_check(
            r#"{"id":"f","kind":"flag","label":"F","token":"-v","value":{"valueType":"string"}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".value"), "{err:?}");
    }

    #[test]
    fn s2_flag_may_be_repeatable() {
        param_ok(r#"{"id":"f","kind":"flag","label":"F","token":"-v","repeatable":true}"#);
    }

    #[test]
    fn s3_option_requires_value() {
        let err = param_check(r#"{"id":"o","kind":"option","label":"O","token":"-o"}"#);
        assert!(err.field.as_deref().unwrap().ends_with(".value"), "{err:?}");
    }

    #[test]
    fn s4_value_optional_only_on_options() {
        param_ok(
            r#"{"id":"o","kind":"option","label":"O","token":"--color","value":{"valueType":"enum","choices":["a"],"optional":true}}"#,
        );
        let err = param_check(
            r#"{"id":"p","kind":"positional","label":"P","order":1,"value":{"valueType":"string","optional":true}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".optional"), "{err:?}");
    }

    #[test]
    fn s5_multiple_and_optional_conflict() {
        let err = param_check(
            r#"{"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"string","multiple":true,"optional":true}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".multiple"), "{err:?}");
    }

    #[test]
    fn s6_option_repeatable_with_multiple_values_is_allowed() {
        param_ok(
            r#"{"id":"o","kind":"option","label":"O","token":"-H","repeatable":true,"value":{"valueType":"string","multiple":true}}"#,
        );
    }

    #[test]
    fn s7_positional_repeatable_is_rejected() {
        let err = param_check(
            r#"{"id":"p","kind":"positional","label":"P","repeatable":true,"order":1,"value":{"valueType":"string"}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".repeatable"), "{err:?}");
    }

    #[test]
    fn s8_positional_requires_unique_order() {
        let err = param_check(
            r#"{"id":"p","kind":"positional","label":"P","value":{"valueType":"string"}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".order"), "{err:?}");
        let err = check(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"a","kind":"positional","label":"A","order":1,"value":{"valueType":"string"}},
                {"id":"b","kind":"positional","label":"B","order":1,"value":{"valueType":"string"}}],"subcommands":[]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".order"), "{err:?}");
    }

    #[test]
    fn s9_choices_only_for_enum_and_unique() {
        let err = param_check(
            r#"{"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"enum"}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".choices"), "{err:?}");
        let err = param_check(
            r#"{"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"string","choices":["a"]}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".choices"), "{err:?}");
        let err = param_check(
            r#"{"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"enum","choices":["a","a"]}}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".choices"), "{err:?}");
    }

    #[test]
    fn s10_custom_requires_syntax_or_description() {
        let err = param_check(r#"{"id":"x","kind":"custom","label":"X"}"#);
        assert!(err.field.as_deref().unwrap().ends_with(".syntax"), "{err:?}");
        param_ok(r#"{"id":"x","kind":"custom","label":"X","syntax":"-C <file>"}"#);
        param_ok(r#"{"id":"x","kind":"custom","label":"X","description":"config mode"}"#);
    }

    #[test]
    fn custom_forbidden_fields_rejected_with_field_paths() {
        let err = param_check(
            r#"{"id":"x","kind":"custom","label":"X","syntax":"s","value":{"valueType":"string"}}"#,
        );
        assert_eq!(err.field.as_deref(), Some("commands[0].parameters[0].value"));
        let err = param_check(
            r#"{"id":"x","kind":"custom","label":"X","syntax":"s","aliases":["-y"]}"#,
        );
        assert_eq!(err.field.as_deref(), Some("commands[0].parameters[0].aliases"));
        let err = param_check(r#"{"id":"x","kind":"custom","label":"X","syntax":"s","order":2}"#);
        assert_eq!(err.field.as_deref(), Some("commands[0].parameters[0].order"));
    }

    #[test]
    fn nested_null_token_is_rejected() {
        let err = check(
            r#"{"id":"root","token":null,"name":"Root","subcommands":[{"id":"sub","token":null,"name":"Sub"}]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".token"), "{err:?}");
    }

    #[test]
    fn only_one_top_level_direct_mode_entry() {
        validate_commands(&[cmd(r#"{"id":"a","token":null,"name":"A"}"#)]).unwrap();
        let err = validate_commands(&[
            cmd(r#"{"id":"a","token":null,"name":"A"}"#),
            cmd(r#"{"id":"b","token":null,"name":"B"}"#),
        ])
        .expect_err("two direct-mode entries");
        assert!(err.message.contains("null token"), "{err:?}");
        validate_commands(&[cmd(r#"{"id":"a","token":"run","name":"A"}"#)]).unwrap();
    }

    #[test]
    fn duplicate_command_ids_across_tree_rejected() {
        let err = check(
            r#"{"id":"same","token":"a","name":"A","subcommands":[{"id":"same","token":"b","name":"B"}]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".id"), "{err:?}");
    }

    #[test]
    fn duplicate_parameter_ids_and_tokens_rejected() {
        let err = check(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"p","kind":"flag","label":"P","token":"-a"},
                {"id":"p","kind":"flag","label":"Q","token":"-b"}],"subcommands":[]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".id"), "{err:?}");
        let err = check(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"p","kind":"flag","label":"P","token":"-a"},
                {"id":"q","kind":"flag","label":"Q","token":"-a"}],"subcommands":[]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".token"), "{err:?}");
        let err = check(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"p","kind":"flag","label":"P","token":"-a"},
                {"id":"q","kind":"flag","label":"Q","token":"-b","aliases":["-a"]}],"subcommands":[]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".aliases"), "{err:?}");
    }

    #[test]
    fn requires_conflicts_reference_integrity() {
        let err = param_check(
            r#"{"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"string"},"requires":["missing"]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".requires"), "{err:?}");
        let err = param_check(
            r#"{"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"string"},"requires":["o"]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".requires"), "{err:?}");
        let err = check(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"a","kind":"flag","label":"A","token":"-a","requires":["b"],"conflicts":["b"]},
                {"id":"b","kind":"flag","label":"B","token":"-b"}],"subcommands":[]}"#,
        );
        assert!(err.field.as_deref().unwrap().ends_with(".requires"), "{err:?}");
        // valid pair
        let valid = cmd(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"a","kind":"flag","label":"A","token":"-a","conflicts":["b"]},
                {"id":"b","kind":"flag","label":"B","token":"-b"}],"subcommands":[]}"#,
        );
        validate_commands(std::slice::from_ref(&valid)).unwrap();
        let valid = cmd(
            r#"{"id":"c","token":null,"name":"C","parameters":[
                {"id":"o","kind":"option","label":"O","token":"-o","value":{"valueType":"string"},"conflicts":["b"]},
                {"id":"b","kind":"flag","label":"B","token":"-b"}],"subcommands":[]}"#,
        );
        validate_commands(std::slice::from_ref(&valid)).unwrap();
    }

    #[test]
    fn depth_and_total_command_caps() {
        let mut json = r#"{"id":"c5","token":"t5","name":"N"}"#.to_string();
        for depth in (0..5).rev() {
            json = format!(
                r#"{{"id":"c{depth}","token":"t{depth}","name":"N","subcommands":[{json}]}}"#
            );
        }
        let err = check(&json);
        assert!(err.message.contains("depth"), "{err:?}");

        let commands: Vec<CommandSpec> = (0..201)
            .map(|i| CommandSpec {
                id: format!("c{i}"),
                token: Some(format!("t{i}")),
                name: "N".to_string(),
                summary: None,
                description: None,
                syntax_examples: vec![],
                version_constraints: vec![],
                parameters: vec![],
                subcommands: vec![],
            })
            .collect();
        let err = validate_commands(&commands).expect_err("over cap");
        assert!(err.message.contains("200"), "{err:?}");
    }

    #[test]
    fn version_constraints_are_display_only_shape_checked() {
        param_ok(
            r#"{"id":"x","kind":"custom","label":"X","description":"d","versionConstraints":[">= 7.90","anything at all"]}"#,
        );
        let err = param_check(
            r#"{"id":"x","kind":"custom","label":"X","description":"d","versionConstraints":[""]}"#,
        );
        assert!(err.field.as_deref().unwrap().contains("versionConstraints"), "{err:?}");
        let err = param_check(
            r#"{"id":"x","kind":"custom","label":"X","description":"d","versionConstraints":["xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"]}"#,
        );
        assert!(err.field.as_deref().unwrap().contains("versionConstraints"), "{err:?}");
    }

    #[test]
    fn token_strings_reject_whitespace() {
        let err = param_check(r#"{"id":"f","kind":"flag","label":"F","token":"-v x"}"#);
        assert!(err.field.as_deref().unwrap().ends_with(".token"), "{err:?}");
    }
}
