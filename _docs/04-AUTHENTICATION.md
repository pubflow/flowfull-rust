# Authentication

The Rust client exposes authentication through `client.auth()`.

Implemented categories:

- Core auth: login, register, logout, me.
- Session validation: validation and Bridge validation.
- Password management: request, validate, complete, change.
- Token auth: create, validate, validate GET, login.
- Social auth: providers, OAuth URL, API login, accounts, unlink.
- Profile: update, upload picture, delete picture.

Successful login/register/token/social auth stores:

- `pubflow_session_id`
- `pubflow_user_data`

Logout clears both keys.

