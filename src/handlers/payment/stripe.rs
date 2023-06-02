use axum::response::IntoResponse;

#[derive(Debug)]
struct PaymentIntentData {
    amount: u128,
    currency: String,
    automatic_payment_methods: bool,
}

pub async fn create_payment_intent(data: web::Query<PaymentIntentData>) -> impl IntoResponse {
    let client = StripeClient::new(
        &std::env::var("STRIPE_PUBLISHABLE_KEY")?,
        &std::env::var("STRIPE_SECRET_KEY")?,
    );

    let mut payment_intent_data = data.into_inner()?;
    payment_intent_data.automatic_payment_methods = true;

    let payment_intent = PaymentIntent::create(&client, payment_intent_data)?;

    (StatusCode::OK, serde_json::json!(payment_intent))
}

pub async fn send_config() -> impl IntoResponse {
    let publishable_key = std::env::var("STRIPE_PUBLISHABLE_KEY")?;

    (
        StatusCode::OK,
        serde_json::json!({
            "publishable_key": publishable_key,
        }),
    )
}
