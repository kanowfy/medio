metadata package = [ { namespace: "org.kny.medio.user", crate: "user_crate" } ]

namespace org.kny.medio.user

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

/// Description of Article service
@wasmbus( actorReceive: true )
service User {
  version: "0.1",
  operations: [ Register, Login, GetUser ]
}

operation Register {
  input: RegisterRequest,
  output: RegisterResponse
}

operation Login {
  input: LoginRequest,
  output: LoginResponse
}

operation GetUser {
  input: U64,
  output: GetUserResponse
}

structure GetUserResponse {
  @required
  success: Boolean,
  reason: String,
  user: UserDto
}

/// operation UpdateUser {
/// input: UpdateUserRequest,
/// output: UpdateUserResponse
/// }

structure RegisterRequest {
  @required
  username: String,
  @required
  password: String,
  @required
  email: String,
  displayName: String
}

structure RegisterResponse {
  @required
  success: Boolean,
  reason: String
}

structure LoginRequest {
  @required
  username: String,
  @required
  password: String
}

structure LoginResponse {
  @required
  success: Boolean,
  /// JWT Token containing user id if successful
  token: String,
  user: UserDto,
  reason: String
}

structure UserDto {
  @required
  id: U64,
  @required
  username: String,
  @required
  displayName: String,
  /// Do we need password here?
  @required
  email: String,
  @required
  createdAt: U64
}
