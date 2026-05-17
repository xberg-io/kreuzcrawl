// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lib.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AuthConfig {
  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is AuthConfig);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  String toString() {
    return 'AuthConfig()';
  }
}

/// @nodoc
class $AuthConfigCopyWith<$Res> {
  $AuthConfigCopyWith(AuthConfig _, $Res Function(AuthConfig) __);
}

/// Adds pattern-matching-related methods to [AuthConfig].
extension AuthConfigPatterns on AuthConfig {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AuthConfig_Basic value)? basic,
    TResult Function(AuthConfig_Bearer value)? bearer,
    TResult Function(AuthConfig_Header value)? header,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AuthConfig_Basic() when basic != null:
        return basic(_that);
      case AuthConfig_Bearer() when bearer != null:
        return bearer(_that);
      case AuthConfig_Header() when header != null:
        return header(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AuthConfig_Basic value) basic,
    required TResult Function(AuthConfig_Bearer value) bearer,
    required TResult Function(AuthConfig_Header value) header,
  }) {
    final _that = this;
    switch (_that) {
      case AuthConfig_Basic():
        return basic(_that);
      case AuthConfig_Bearer():
        return bearer(_that);
      case AuthConfig_Header():
        return header(_that);
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AuthConfig_Basic value)? basic,
    TResult? Function(AuthConfig_Bearer value)? bearer,
    TResult? Function(AuthConfig_Header value)? header,
  }) {
    final _that = this;
    switch (_that) {
      case AuthConfig_Basic() when basic != null:
        return basic(_that);
      case AuthConfig_Bearer() when bearer != null:
        return bearer(_that);
      case AuthConfig_Header() when header != null:
        return header(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String username, String password)? basic,
    TResult Function(String token)? bearer,
    TResult Function(String name, String value)? header,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case AuthConfig_Basic() when basic != null:
        return basic(_that.username, _that.password);
      case AuthConfig_Bearer() when bearer != null:
        return bearer(_that.token);
      case AuthConfig_Header() when header != null:
        return header(_that.name, _that.value);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String username, String password) basic,
    required TResult Function(String token) bearer,
    required TResult Function(String name, String value) header,
  }) {
    final _that = this;
    switch (_that) {
      case AuthConfig_Basic():
        return basic(_that.username, _that.password);
      case AuthConfig_Bearer():
        return bearer(_that.token);
      case AuthConfig_Header():
        return header(_that.name, _that.value);
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String username, String password)? basic,
    TResult? Function(String token)? bearer,
    TResult? Function(String name, String value)? header,
  }) {
    final _that = this;
    switch (_that) {
      case AuthConfig_Basic() when basic != null:
        return basic(_that.username, _that.password);
      case AuthConfig_Bearer() when bearer != null:
        return bearer(_that.token);
      case AuthConfig_Header() when header != null:
        return header(_that.name, _that.value);
      case _:
        return null;
    }
  }
}

/// @nodoc

class AuthConfig_Basic extends AuthConfig {
  const AuthConfig_Basic({required this.username, required this.password})
      : super._();

  /// Username sent in the `Authorization: Basic` header.
  final String username;

  /// Password sent in the `Authorization: Basic` header.
  final String password;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AuthConfig_BasicCopyWith<AuthConfig_Basic> get copyWith =>
      _$AuthConfig_BasicCopyWithImpl<AuthConfig_Basic>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AuthConfig_Basic &&
            (identical(other.username, username) ||
                other.username == username) &&
            (identical(other.password, password) ||
                other.password == password));
  }

  @override
  int get hashCode => Object.hash(runtimeType, username, password);

  @override
  String toString() {
    return 'AuthConfig.basic(username: $username, password: $password)';
  }
}

/// @nodoc
abstract mixin class $AuthConfig_BasicCopyWith<$Res>
    implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_BasicCopyWith(
          AuthConfig_Basic value, $Res Function(AuthConfig_Basic) _then) =
      _$AuthConfig_BasicCopyWithImpl;
  @useResult
  $Res call({String username, String password});
}

/// @nodoc
class _$AuthConfig_BasicCopyWithImpl<$Res>
    implements $AuthConfig_BasicCopyWith<$Res> {
  _$AuthConfig_BasicCopyWithImpl(this._self, this._then);

  final AuthConfig_Basic _self;
  final $Res Function(AuthConfig_Basic) _then;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? username = null,
    Object? password = null,
  }) {
    return _then(AuthConfig_Basic(
      username: null == username
          ? _self.username
          : username // ignore: cast_nullable_to_non_nullable
              as String,
      password: null == password
          ? _self.password
          : password // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class AuthConfig_Bearer extends AuthConfig {
  const AuthConfig_Bearer({required this.token}) : super._();

  /// Token sent in the `Authorization: Bearer` header.
  final String token;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AuthConfig_BearerCopyWith<AuthConfig_Bearer> get copyWith =>
      _$AuthConfig_BearerCopyWithImpl<AuthConfig_Bearer>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AuthConfig_Bearer &&
            (identical(other.token, token) || other.token == token));
  }

  @override
  int get hashCode => Object.hash(runtimeType, token);

  @override
  String toString() {
    return 'AuthConfig.bearer(token: $token)';
  }
}

/// @nodoc
abstract mixin class $AuthConfig_BearerCopyWith<$Res>
    implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_BearerCopyWith(
          AuthConfig_Bearer value, $Res Function(AuthConfig_Bearer) _then) =
      _$AuthConfig_BearerCopyWithImpl;
  @useResult
  $Res call({String token});
}

/// @nodoc
class _$AuthConfig_BearerCopyWithImpl<$Res>
    implements $AuthConfig_BearerCopyWith<$Res> {
  _$AuthConfig_BearerCopyWithImpl(this._self, this._then);

  final AuthConfig_Bearer _self;
  final $Res Function(AuthConfig_Bearer) _then;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? token = null,
  }) {
    return _then(AuthConfig_Bearer(
      token: null == token
          ? _self.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class AuthConfig_Header extends AuthConfig {
  const AuthConfig_Header({required this.name, required this.value})
      : super._();

  /// HTTP header name to set on each request.
  final String name;

  /// HTTP header value to send.
  final String value;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AuthConfig_HeaderCopyWith<AuthConfig_Header> get copyWith =>
      _$AuthConfig_HeaderCopyWithImpl<AuthConfig_Header>(this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AuthConfig_Header &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.value, value) || other.value == value));
  }

  @override
  int get hashCode => Object.hash(runtimeType, name, value);

  @override
  String toString() {
    return 'AuthConfig.header(name: $name, value: $value)';
  }
}

/// @nodoc
abstract mixin class $AuthConfig_HeaderCopyWith<$Res>
    implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_HeaderCopyWith(
          AuthConfig_Header value, $Res Function(AuthConfig_Header) _then) =
      _$AuthConfig_HeaderCopyWithImpl;
  @useResult
  $Res call({String name, String value});
}

/// @nodoc
class _$AuthConfig_HeaderCopyWithImpl<$Res>
    implements $AuthConfig_HeaderCopyWith<$Res> {
  _$AuthConfig_HeaderCopyWithImpl(this._self, this._then);

  final AuthConfig_Header _self;
  final $Res Function(AuthConfig_Header) _then;

  /// Create a copy of AuthConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  $Res call({
    Object? name = null,
    Object? value = null,
  }) {
    return _then(AuthConfig_Header(
      name: null == name
          ? _self.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      value: null == value
          ? _self.value
          : value // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

// dart format on
