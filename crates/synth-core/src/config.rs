//! CLI-facing config wrapper: a file holding a single `spec`.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::spec::GenSpec;

/// A config file's contents: a wrapper carrying one [`GenSpec`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub spec: GenSpec,
}

impl Config {
    /// Parse a config from JSON (`{"spec": {…}}`) and validate the spec.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed JSON, [`Error::BadSpec`] on an invalid spec.
    pub fn from_json(s: &str) -> Result<Self> {
        let cfg: Config = serde_json::from_str(s)?;
        cfg.spec.validate()?;
        Ok(cfg)
    }

    /// Parse a config from TOML (`[spec]` table) and validate the spec.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed TOML, [`Error::BadSpec`] on an invalid spec.
    pub fn from_toml(s: &str) -> Result<Self> {
        let cfg: Config = toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        cfg.spec.validate()?;
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    const JSON: &str = r#"{ "spec": { "seed": 1, "bars": 4, "start_price": 100.0,
        "regimes": [ { "kind": "range", "len": 4, "drift": 0.1, "vol": 0.01 } ],
        "microstructure": { "book_depth": 2, "spread_bps": 3.0, "trade_rate": 1.0 } } }"#;

    #[test]
    fn from_json_parses_and_validates() {
        let cfg = Config::from_json(JSON).unwrap();
        assert_eq!(cfg.spec.bars, 4);
    }

    #[test]
    fn from_toml_parses_and_validates() {
        let toml = r#"
[spec]
seed = 7
bars = 3
start_price = 50.0
[[spec.regimes]]
kind = "vol"
len = 3
drift = 0.0
vol = 0.02
[spec.microstructure]
book_depth = 2
spread_bps = 5.0
trade_rate = 2.0
"#;
        let cfg = Config::from_toml(toml).unwrap();
        assert_eq!(cfg.spec.seed, 7);
        assert_eq!(cfg.spec.bars, 3);
    }

    #[test]
    fn invalid_spec_rejected() {
        let bad = r#"{ "spec": { "seed": 1, "bars": 5, "start_price": 100.0,
            "regimes": [ { "kind": "range", "len": 3, "drift": 0.1, "vol": 0.01 } ],
            "microstructure": { "book_depth": 2, "spread_bps": 3.0, "trade_rate": 1.0 } } }"#;
        assert!(Config::from_json(bad).is_err());
    }
}
