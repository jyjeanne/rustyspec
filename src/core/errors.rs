#![allow(dead_code)]
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum RustySpecError {
    #[error("Config error at {path}: {message}\n  Fix: {fix}")]
    Config {
        path: PathBuf,
        message: String,
        fix: String,
    },

    #[error("Spec error for feature {feature_id}: {message}\n  Fix: {fix}")]
    Spec {
        feature_id: String,
        message: String,
        fix: String,
    },

    #[error("Template error in {template}: {message}\n  Fix: {fix}")]
    Template {
        template: String,
        message: String,
        fix: String,
    },

    #[error("Git error: {message}\n  Fix: {fix}")]
    Git { message: String, fix: String },

    #[error("Feature error: {message}\n  Fix: {fix}")]
    Feature { message: String, fix: String },

    #[error("Init error at {path}: {message}\n  Fix: {fix}")]
    Init {
        path: PathBuf,
        message: String,
        fix: String,
    },

    #[error("Validation error: {message}")]
    Validation { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_displays_path_and_fix() {
        let err = RustySpecError::Config {
            path: PathBuf::from("/project/rustyspec.toml"),
            message: "missing field 'name'".into(),
            fix: "Add [project] name = \"my_project\" to rustyspec.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("/project/rustyspec.toml"));
        assert!(msg.contains("missing field 'name'"));
        assert!(msg.contains("Fix:"));
    }

    #[test]
    fn spec_error_displays_feature_id() {
        let err = RustySpecError::Spec {
            feature_id: "003".into(),
            message: "spec.md not found".into(),
            fix: "Run 'rustyspec specify' first".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("003"));
        assert!(msg.contains("spec.md not found"));
    }

    #[test]
    fn all_error_variants_produce_nonempty_messages() {
        let errors: Vec<RustySpecError> = vec![
            RustySpecError::Config {
                path: PathBuf::from("x"),
                message: "m".into(),
                fix: "f".into(),
            },
            RustySpecError::Spec {
                feature_id: "1".into(),
                message: "m".into(),
                fix: "f".into(),
            },
            RustySpecError::Template {
                template: "t".into(),
                message: "m".into(),
                fix: "f".into(),
            },
            RustySpecError::Git {
                message: "m".into(),
                fix: "f".into(),
            },
            RustySpecError::Feature {
                message: "m".into(),
                fix: "f".into(),
            },
            RustySpecError::Init {
                path: PathBuf::from("x"),
                message: "m".into(),
                fix: "f".into(),
            },
            RustySpecError::Validation {
                message: "m".into(),
            },
        ];
        for err in errors {
            assert!(!err.to_string().is_empty());
        }
    }
}
