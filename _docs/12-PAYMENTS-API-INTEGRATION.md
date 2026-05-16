# Payments API Integration

The `client.pay()` helper wraps `/bridge-payment/*` endpoints.

Implemented groups:

- Payment intents.
- Payments.
- Payment methods.
- Addresses.
- Customers.
- Subscriptions.
- Organizations.
- Organization members.
- Guest conversion.

Guest checkout can use request options with `PaymentsClient::guest_options(token)` for endpoints that need `X-Guest-Token`.

