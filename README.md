# Traduora API bindings

This crate implements API bindings to communicate with a [Traduora](https://traduora.co/) instance.
You can find the list of implemented endpoints below. Traduora's general API documentation is available [in their
documentation](https://docs.traduora.co/docs/api/v1/overview) and an (up-to-date) endpoint documentation can be
found [here](https://docs.traduora.co/docs/api/v1/swagger).

The API was originally implemented for version [v0.19.1](https://github.com/ever-co/ever-traduora/releases/tag/v0.19.1) and
was also tested with this version. When you're reading this, it's probably not actively maintained anymore, but it should still
be passively maintained. Feel free to open an issue or a PR and it will probably get answered in a few days.

All endpoints are contained in the [`api`](src/api) module. To use them, you can simply instantiate one, create a `Traduora` client
and call `endpoint.query(&client)` or the async equivalent with an `AsyncTraduora` client and `endpoint.query_async(&async_client)`.
On success, the endpoint then returns a struct that is specific to the endpoint, providing the data that was parsed from Traduora's
response. For more fine-grained control, you can choose your own type to deserialize the response into by calling `endpoint.query_custom(&client)`
instead. The type just has to implement `serde::DeserializeOwned` and you're good to go.

Some endpoints require authentication before Traduora allows you to access them.
This is modelled at type level for this crate. Calling an endpoint requiring authentication without authentication
leads to a compile-time error instead of erroring only at run time. If you have an `Traduora<Unauthenticated>` instance,
this means you are not logged in, while a `Traduora<Authenticated>` instance symbolizes an authenticated client.
This check is not perfect however, so you might still get error related to invalid authentication. This can happen
if your client is alive for a long time so that its access token expires or if you construct your client by passing the
access token yourself (see `TraduoraBuilder::with_access_token`).

# Usage examples

Creating a new term:
```rust no_run
use traduora::{
    api::{terms::CreateTerm, ProjectId},
    Login, Query, Traduora,
};

let client = Traduora::with_auth(
    "localhost:8080",
    Login::password("user@mail.example", "password"),
).unwrap();
let term = CreateTerm::new(
    "hello.world",
    ProjectId::new("b1001dd9-e1c0-4fb0-a60d-eaaec304d332"),
);

let new_term = term.query(&client).unwrap();

assert!(new_term.labels.is_empty());
assert_eq!("hello.world", new_term.value);
```

## Design

The design of this crate is heavily inspired (=outright stolen) from the [GitLab API](https://gitlab.kitware.com/utils/rust-gitlab) crate.
For details, you can read the excellent blog post [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is) by Ben Boeckel.
This crate calls the `Query` and `AsyncQuery` traits of the GitLab crate `CustomQuery` and `AsyncCustomQuery` instead as the normal
`Query` and `AsyncQuery` traits return a `DefaultModel` type to make the crate easier to use.

## Implementation progress

The endpoints from this list are taken from [here](api.json)

**Status values:** &nbsp;&nbsp;&nbsp;&nbsp; ✅ Done &nbsp;&nbsp; | &nbsp;&nbsp; ☑ Done but missing documentation because API docs are unclear &nbsp;&nbsp; | &nbsp;&nbsp; ❌ Not yet implemented


| Status | Method | Endpoint                                                                                | Type                                       |
|--------|--------|-----------------------------------------------------------------------------------------|--------------------------------------------|
|   ❌   | POST   | `/api/v1/auth/change-password`                                                          |                                            |
|   ❌   | POST   | `/api/v1/auth/forgot-password`                                                          |                                            |
|   ✅   | GET    | `/api/v1/auth/providers`                                                                | [`api::auth::Providers`]                   |
|   ❌   | POST   | `/api/v1/auth/reset-password`                                                           |                                            |
|   ❌   | POST   | `/api/v1/auth/signup-provider`                                                          |                                            |
|   ✅   | POST   | `/api/v1/auth/signup`                                                                   | [`api::auth::Signup`]                      |
|   ✅   | POST   | `/api/v1/auth/token`                                                                    | [`api::auth::Token`]                       |
|        |        |                                                                                         |                                            |
|   ❌   | GET    | `/api/v1/locales`                                                                       |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/clients/{clientId}/rotate-secret`                         |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/clients/{clientId}`                                       |                                            |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/clients/{clientId}`                                       |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/clients`                                                  |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/clients`                                                  |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/exports`                                                  |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/imports`                                                  |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/invites/{inviteId}`                                       |                                            |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/invites/{inviteId}`                                       |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/invites`                                                  |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/invites`                                                  |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}/translations/{localeCode}`|                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}/translations/{localeCode}`|                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}`                          |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}`                          |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/labels/{labelId}`                                         |                                            |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/labels/{labelId}`                                         |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/labels`                                                   |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/labels`                                                   |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/plan`                                                     |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/stats`                                                    |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/terms/{termId}`                                           |                                            |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/terms/{termId}`                                           |                                            |
|   ✅   | GET    | `/api/v1/projects/{projectId}/terms`                                                    | [`api::terms::Terms`]                      |
|   ✅   | POST   | `/api/v1/projects/{projectId}/terms`                                                    | [`api::terms::CreateTerm`]                 |
|        |        |                                                                                         |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/translations/{localeCode}`                                |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/translations/{localeCode}`                                |                                            |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/translations/{localeCode}`                                |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/translations`                                             |                                            |
|   ❌   | POST   | `/api/v1/projects/{projectId}/translations`                                             |                                            |
|        |        |                                                                                         |                                            |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/users/{userId}`                                           |                                            |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/users/{userId}`                                           |                                            |
|   ❌   | GET    | `/api/v1/projects/{projectId}/users`                                                    |                                            |
|        |        |                                                                                         |                                            |
|   ✅   | DELETE | `/api/v1/projects/{projectId}`                                                          | [`api::projects::DeleteProject`]           |
|   ✅   | GET    | `/api/v1/projects/{projectId}`                                                          | [`api::projects::ShowProject`]             |
|   ✅   | PATCH  | `/api/v1/projects/{projectId}`                                                          | [`api::projects::EditProject`]             |
|   ✅   | GET    | `/api/v1/projects`                                                                      | [`api::projects::Projects`]                |
|   ✅   | POST   | `/api/v1/projects`                                                                      | [`api::projects::CreateProject`]           |
|        |        |                                                                                         |                                            |
|   ❌   | DELETE | `/api/v1/users/me`                                                                      |                                            |
|   ✅   | GET    | `/api/v1/users/me`                                                                      | [`api::users::Me`]                         |
|   ❌   | PATCH  | `/api/v1/users/me`                                                                      |                                            |

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-Apache-2.0) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

Your choice.