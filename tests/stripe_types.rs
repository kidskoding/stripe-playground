use stripe::CreateCustomer;

#[test]
fn can_construct_create_customer() {
    let mut params: CreateCustomer<'static> = CreateCustomer::default();
    params.name = Some("Test User");
    params.email = Some("test@example.com");

    assert_eq!(params.name, Some("Test User"));
    assert_eq!(params.email, Some("test@example.com"));
}

