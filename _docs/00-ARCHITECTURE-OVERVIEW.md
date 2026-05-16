# Architecture Overview

`flowfull-rust` is an async SDK crate for Flowfull and Flowless-compatible APIs.

Core modules:

- `client`: HTTP methods, retries, response parsing.
- `config`: builder, headers, timeout, retries, session options.
- `session`: priority-based session lookup and `pubflow_*` storage keys.
- `storage`: memory and file storage adapters.
- `auth`: Flowless auth endpoints.
- `bridge`: Bridge validation helpers for backends.
- `query`: universal bracket filter syntax.
- `upload`: multipart uploads.
- `payments`: Bridge Payments API helper.

Default behavior is backend-safe: `include_session = false`.

