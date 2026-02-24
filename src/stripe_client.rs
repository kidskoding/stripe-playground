pub struct StripeClient {
    pub inner: stripe::Client,
}

impl StripeClient {
    pub fn new(secret_key: impl Into<String>) -> Self {
        let inner = stripe::Client::new(secret_key);
        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_client() {
        let _client = StripeClient::new("sk_test_dummy");
    }
}
