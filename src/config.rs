pub struct Config {
    pub secret_key: Option<String>,
}

impl Config {
    pub fn from_secret_key(secret_key: Option<String>) -> Self {
        Self { secret_key }
    }

    pub fn from_env() -> Self {
        let secret_key = std::env::var("STRIPE_SECRET_KEY").ok();
        Self::from_secret_key(secret_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_secret_key_sets_value_when_present() {
        let config = Config::from_secret_key(Some("sk_test_123".to_string()));
        assert_eq!(config.secret_key.as_deref(), Some("sk_test_123"));
    }

    #[test]
    fn from_secret_key_is_none_when_absent() {
        let config = Config::from_secret_key(None);
        assert!(config.secret_key.is_none());
    }
}
