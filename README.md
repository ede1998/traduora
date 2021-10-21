# Traduora API bindings

Tested against version [v0.19.1](https://github.com/ever-co/ever-traduora/releases/tag/v0.19.1)



# Implementation progress

The endpoints from this list are taken from [here](api.json)

**Status values:** &nbsp;&nbsp;&nbsp;&nbsp; ✅ Done &nbsp;&nbsp; | &nbsp;&nbsp; ☑ Done but missing documentation because API docs are unclear &nbsp;&nbsp; | &nbsp;&nbsp; ❌ Not yet implemented


| Status | Method | Endpoint                                                                                | Type                   |
|--------|--------|-----------------------------------------------------------------------------------------|------------------------|
|   ❌   | POST   | `/api/v1/auth/change-password`                                                          |                        |
|   ❌   | POST   | `/api/v1/auth/forgot-password`                                                          |                        |
|   ☑   | GET    | `/api/v1/auth/providers`                                                                | [`api::auth::Providers`] |
|   ❌   | POST   | `/api/v1/auth/reset-password`                                                           |                        |
|   ❌   | POST   | `/api/v1/auth/signup-provider`                                                          |                        |
|   ✅   | POST   | `/api/v1/auth/signup`                                                                   | [`api::auth::Signup`]  |
|   ✅   | POST   | `/api/v1/auth/token`                                                                    | [`api::auth::Token`]   |
|        |        |                                                                                         |                        |
|   ❌   | GET    | `/api/v1/locales`                                                                       |                        |
|        |        |                                                                                         |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/clients/{clientId}/rotate-secret`                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/clients/{clientId}`                                       |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/clients/{clientId}`                                       |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/clients`                                                  |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/clients`                                                  |                        |
|        |        |                                                                                         |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/exports`                                                  |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/imports`                                                  |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/invites/{inviteId}`                                       |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/invites/{inviteId}`                                       |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/invites`                                                  |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/invites`                                                  |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}/translations/{localeCode}`|                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}/translations/{localeCode}`|                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}`                          |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/labels/{labelId}/terms/{termId}`                          |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/labels/{labelId}`                                         |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/labels/{labelId}`                                         |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/labels`                                                   |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/labels`                                                   |                        |
|        |        |                                                                                         |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/plan`                                                     |                        |
|        |        |                                                                                         |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/stats`                                                    |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/terms/{termId}`                                           |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/terms/{termId}`                                           |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/terms`                                                    |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/terms`                                                    |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/translations/{localeCode}`                                |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/translations/{localeCode}`                                |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/translations/{localeCode}`                                |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/translations`                                             |                        |
|   ❌   | POST   | `/api/v1/projects/{projectId}/translations`                                             |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}/users/{userId}`                                           |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}/users/{userId}`                                           |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}/users`                                                    |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/projects/{projectId}`                                                          |                        |
|   ❌   | GET    | `/api/v1/projects/{projectId}`                                                          |                        |
|   ❌   | PATCH  | `/api/v1/projects/{projectId}`                                                          |                        |
|   ❌   | GET    | `/api/v1/projects`                                                                      |                        |
|   ❌   | POST   | `/api/v1/projects`                                                                      |                        |
|        |        |                                                                                         |                        |
|   ❌   | DELETE | `/api/v1/users/me`                                                                      |                        |
|   ✅   | GET    | `/api/v1/users/me`                                                                      | [`api::users::Me`]     |
|   ❌   | PATCH  | `/api/v1/users/me`                                                                      |                        |