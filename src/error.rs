#[derive(Debug)]
pub enum PlaygroundError {
    Stripe(stripe::StripeError),
    Config(String),
}

impl From<stripe::StripeError> for PlaygroundError {
    fn from(err: stripe::StripeError) -> Self {
        PlaygroundError::Stripe(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_stores_message() {
        let err = PlaygroundError::Config("missing key".to_string());

        match err {
            PlaygroundError::Config(msg) => assert_eq!(msg, "missing key"),
            _ => panic!("expected Config variant"),
        }
    }
}

