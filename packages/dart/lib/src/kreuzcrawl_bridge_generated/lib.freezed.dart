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
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AuthConfig()';
}


}

/// @nodoc
class $AuthConfigCopyWith<$Res>  {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AuthConfig_Basic value)?  basic,TResult Function( AuthConfig_Bearer value)?  bearer,TResult Function( AuthConfig_Header value)?  header,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that);case AuthConfig_Bearer() when bearer != null:
return bearer(_that);case AuthConfig_Header() when header != null:
return header(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AuthConfig_Basic value)  basic,required TResult Function( AuthConfig_Bearer value)  bearer,required TResult Function( AuthConfig_Header value)  header,}){
final _that = this;
switch (_that) {
case AuthConfig_Basic():
return basic(_that);case AuthConfig_Bearer():
return bearer(_that);case AuthConfig_Header():
return header(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AuthConfig_Basic value)?  basic,TResult? Function( AuthConfig_Bearer value)?  bearer,TResult? Function( AuthConfig_Header value)?  header,}){
final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that);case AuthConfig_Bearer() when bearer != null:
return bearer(_that);case AuthConfig_Header() when header != null:
return header(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String username,  String password)?  basic,TResult Function( String token)?  bearer,TResult Function( String name,  String value)?  header,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that.username,_that.password);case AuthConfig_Bearer() when bearer != null:
return bearer(_that.token);case AuthConfig_Header() when header != null:
return header(_that.name,_that.value);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String username,  String password)  basic,required TResult Function( String token)  bearer,required TResult Function( String name,  String value)  header,}) {final _that = this;
switch (_that) {
case AuthConfig_Basic():
return basic(_that.username,_that.password);case AuthConfig_Bearer():
return bearer(_that.token);case AuthConfig_Header():
return header(_that.name,_that.value);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String username,  String password)?  basic,TResult? Function( String token)?  bearer,TResult? Function( String name,  String value)?  header,}) {final _that = this;
switch (_that) {
case AuthConfig_Basic() when basic != null:
return basic(_that.username,_that.password);case AuthConfig_Bearer() when bearer != null:
return bearer(_that.token);case AuthConfig_Header() when header != null:
return header(_that.name,_that.value);case _:
  return null;

}
}

}

/// @nodoc


class AuthConfig_Basic extends AuthConfig {
  const AuthConfig_Basic({required this.username, required this.password}): super._();


/// Username sent in the `Authorization: Basic` header.
 final  String username;
/// Password sent in the `Authorization: Basic` header.
 final  String password;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AuthConfig_BasicCopyWith<AuthConfig_Basic> get copyWith => _$AuthConfig_BasicCopyWithImpl<AuthConfig_Basic>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig_Basic&&(identical(other.username, username) || other.username == username)&&(identical(other.password, password) || other.password == password));
}


@override
int get hashCode => Object.hash(runtimeType,username,password);

@override
String toString() {
  return 'AuthConfig.basic(username: $username, password: $password)';
}


}

/// @nodoc
abstract mixin class $AuthConfig_BasicCopyWith<$Res> implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_BasicCopyWith(AuthConfig_Basic value, $Res Function(AuthConfig_Basic) _then) = _$AuthConfig_BasicCopyWithImpl;
@useResult
$Res call({
 String username, String password
});




}
/// @nodoc
class _$AuthConfig_BasicCopyWithImpl<$Res>
    implements $AuthConfig_BasicCopyWith<$Res> {
  _$AuthConfig_BasicCopyWithImpl(this._self, this._then);

  final AuthConfig_Basic _self;
  final $Res Function(AuthConfig_Basic) _then;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? username = null,Object? password = null,}) {
  return _then(AuthConfig_Basic(
username: null == username ? _self.username : username // ignore: cast_nullable_to_non_nullable
as String,password: null == password ? _self.password : password // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AuthConfig_Bearer extends AuthConfig {
  const AuthConfig_Bearer({required this.token}): super._();


/// Token sent in the `Authorization: Bearer` header.
 final  String token;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AuthConfig_BearerCopyWith<AuthConfig_Bearer> get copyWith => _$AuthConfig_BearerCopyWithImpl<AuthConfig_Bearer>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig_Bearer&&(identical(other.token, token) || other.token == token));
}


@override
int get hashCode => Object.hash(runtimeType,token);

@override
String toString() {
  return 'AuthConfig.bearer(token: $token)';
}


}

/// @nodoc
abstract mixin class $AuthConfig_BearerCopyWith<$Res> implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_BearerCopyWith(AuthConfig_Bearer value, $Res Function(AuthConfig_Bearer) _then) = _$AuthConfig_BearerCopyWithImpl;
@useResult
$Res call({
 String token
});




}
/// @nodoc
class _$AuthConfig_BearerCopyWithImpl<$Res>
    implements $AuthConfig_BearerCopyWith<$Res> {
  _$AuthConfig_BearerCopyWithImpl(this._self, this._then);

  final AuthConfig_Bearer _self;
  final $Res Function(AuthConfig_Bearer) _then;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? token = null,}) {
  return _then(AuthConfig_Bearer(
token: null == token ? _self.token : token // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AuthConfig_Header extends AuthConfig {
  const AuthConfig_Header({required this.name, required this.value}): super._();


/// HTTP header name to set on each request.
 final  String name;
/// HTTP header value to send.
 final  String value;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AuthConfig_HeaderCopyWith<AuthConfig_Header> get copyWith => _$AuthConfig_HeaderCopyWithImpl<AuthConfig_Header>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AuthConfig_Header&&(identical(other.name, name) || other.name == name)&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,name,value);

@override
String toString() {
  return 'AuthConfig.header(name: $name, value: $value)';
}


}

/// @nodoc
abstract mixin class $AuthConfig_HeaderCopyWith<$Res> implements $AuthConfigCopyWith<$Res> {
  factory $AuthConfig_HeaderCopyWith(AuthConfig_Header value, $Res Function(AuthConfig_Header) _then) = _$AuthConfig_HeaderCopyWithImpl;
@useResult
$Res call({
 String name, String value
});




}
/// @nodoc
class _$AuthConfig_HeaderCopyWithImpl<$Res>
    implements $AuthConfig_HeaderCopyWith<$Res> {
  _$AuthConfig_HeaderCopyWithImpl(this._self, this._then);

  final AuthConfig_Header _self;
  final $Res Function(AuthConfig_Header) _then;

/// Create a copy of AuthConfig
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? value = null,}) {
  return _then(AuthConfig_Header(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$CrawlEvent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'CrawlEvent()';
}


}

/// @nodoc
class $CrawlEventCopyWith<$Res>  {
$CrawlEventCopyWith(CrawlEvent _, $Res Function(CrawlEvent) __);
}


/// Adds pattern-matching-related methods to [CrawlEvent].
extension CrawlEventPatterns on CrawlEvent {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CrawlEvent_Page value)?  page,TResult Function( CrawlEvent_Error value)?  error,TResult Function( CrawlEvent_Complete value)?  complete,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that);case CrawlEvent_Error() when error != null:
return error(_that);case CrawlEvent_Complete() when complete != null:
return complete(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CrawlEvent_Page value)  page,required TResult Function( CrawlEvent_Error value)  error,required TResult Function( CrawlEvent_Complete value)  complete,}){
final _that = this;
switch (_that) {
case CrawlEvent_Page():
return page(_that);case CrawlEvent_Error():
return error(_that);case CrawlEvent_Complete():
return complete(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CrawlEvent_Page value)?  page,TResult? Function( CrawlEvent_Error value)?  error,TResult? Function( CrawlEvent_Complete value)?  complete,}){
final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that);case CrawlEvent_Error() when error != null:
return error(_that);case CrawlEvent_Complete() when complete != null:
return complete(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( CrawlPageResult result)?  page,TResult Function( String url,  String error)?  error,TResult Function( PlatformInt64 pagesCrawled)?  complete,required TResult orElse(),}) {final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that.result);case CrawlEvent_Error() when error != null:
return error(_that.url,_that.error);case CrawlEvent_Complete() when complete != null:
return complete(_that.pagesCrawled);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( CrawlPageResult result)  page,required TResult Function( String url,  String error)  error,required TResult Function( PlatformInt64 pagesCrawled)  complete,}) {final _that = this;
switch (_that) {
case CrawlEvent_Page():
return page(_that.result);case CrawlEvent_Error():
return error(_that.url,_that.error);case CrawlEvent_Complete():
return complete(_that.pagesCrawled);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( CrawlPageResult result)?  page,TResult? Function( String url,  String error)?  error,TResult? Function( PlatformInt64 pagesCrawled)?  complete,}) {final _that = this;
switch (_that) {
case CrawlEvent_Page() when page != null:
return page(_that.result);case CrawlEvent_Error() when error != null:
return error(_that.url,_that.error);case CrawlEvent_Complete() when complete != null:
return complete(_that.pagesCrawled);case _:
  return null;

}
}

}

/// @nodoc


class CrawlEvent_Page extends CrawlEvent {
  const CrawlEvent_Page({required this.result}): super._();


/// The crawled page result.
 final  CrawlPageResult result;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CrawlEvent_PageCopyWith<CrawlEvent_Page> get copyWith => _$CrawlEvent_PageCopyWithImpl<CrawlEvent_Page>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent_Page&&(identical(other.result, result) || other.result == result));
}


@override
int get hashCode => Object.hash(runtimeType,result);

@override
String toString() {
  return 'CrawlEvent.page(result: $result)';
}


}

/// @nodoc
abstract mixin class $CrawlEvent_PageCopyWith<$Res> implements $CrawlEventCopyWith<$Res> {
  factory $CrawlEvent_PageCopyWith(CrawlEvent_Page value, $Res Function(CrawlEvent_Page) _then) = _$CrawlEvent_PageCopyWithImpl;
@useResult
$Res call({
 CrawlPageResult result
});




}
/// @nodoc
class _$CrawlEvent_PageCopyWithImpl<$Res>
    implements $CrawlEvent_PageCopyWith<$Res> {
  _$CrawlEvent_PageCopyWithImpl(this._self, this._then);

  final CrawlEvent_Page _self;
  final $Res Function(CrawlEvent_Page) _then;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? result = null,}) {
  return _then(CrawlEvent_Page(
result: null == result ? _self.result : result // ignore: cast_nullable_to_non_nullable
as CrawlPageResult,
  ));
}


}

/// @nodoc


class CrawlEvent_Error extends CrawlEvent {
  const CrawlEvent_Error({required this.url, required this.error}): super._();


/// The URL that failed.
 final  String url;
/// The error message.
 final  String error;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CrawlEvent_ErrorCopyWith<CrawlEvent_Error> get copyWith => _$CrawlEvent_ErrorCopyWithImpl<CrawlEvent_Error>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent_Error&&(identical(other.url, url) || other.url == url)&&(identical(other.error, error) || other.error == error));
}


@override
int get hashCode => Object.hash(runtimeType,url,error);

@override
String toString() {
  return 'CrawlEvent.error(url: $url, error: $error)';
}


}

/// @nodoc
abstract mixin class $CrawlEvent_ErrorCopyWith<$Res> implements $CrawlEventCopyWith<$Res> {
  factory $CrawlEvent_ErrorCopyWith(CrawlEvent_Error value, $Res Function(CrawlEvent_Error) _then) = _$CrawlEvent_ErrorCopyWithImpl;
@useResult
$Res call({
 String url, String error
});




}
/// @nodoc
class _$CrawlEvent_ErrorCopyWithImpl<$Res>
    implements $CrawlEvent_ErrorCopyWith<$Res> {
  _$CrawlEvent_ErrorCopyWithImpl(this._self, this._then);

  final CrawlEvent_Error _self;
  final $Res Function(CrawlEvent_Error) _then;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? url = null,Object? error = null,}) {
  return _then(CrawlEvent_Error(
url: null == url ? _self.url : url // ignore: cast_nullable_to_non_nullable
as String,error: null == error ? _self.error : error // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class CrawlEvent_Complete extends CrawlEvent {
  const CrawlEvent_Complete({required this.pagesCrawled}): super._();


/// Total number of pages crawled.
 final  PlatformInt64 pagesCrawled;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CrawlEvent_CompleteCopyWith<CrawlEvent_Complete> get copyWith => _$CrawlEvent_CompleteCopyWithImpl<CrawlEvent_Complete>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CrawlEvent_Complete&&(identical(other.pagesCrawled, pagesCrawled) || other.pagesCrawled == pagesCrawled));
}


@override
int get hashCode => Object.hash(runtimeType,pagesCrawled);

@override
String toString() {
  return 'CrawlEvent.complete(pagesCrawled: $pagesCrawled)';
}


}

/// @nodoc
abstract mixin class $CrawlEvent_CompleteCopyWith<$Res> implements $CrawlEventCopyWith<$Res> {
  factory $CrawlEvent_CompleteCopyWith(CrawlEvent_Complete value, $Res Function(CrawlEvent_Complete) _then) = _$CrawlEvent_CompleteCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 pagesCrawled
});




}
/// @nodoc
class _$CrawlEvent_CompleteCopyWithImpl<$Res>
    implements $CrawlEvent_CompleteCopyWith<$Res> {
  _$CrawlEvent_CompleteCopyWithImpl(this._self, this._then);

  final CrawlEvent_Complete _self;
  final $Res Function(CrawlEvent_Complete) _then;

/// Create a copy of CrawlEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? pagesCrawled = null,}) {
  return _then(CrawlEvent_Complete(
pagesCrawled: null == pagesCrawled ? _self.pagesCrawled : pagesCrawled // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc
mixin _$PageAction {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PageAction()';
}


}

/// @nodoc
class $PageActionCopyWith<$Res>  {
$PageActionCopyWith(PageAction _, $Res Function(PageAction) __);
}


/// Adds pattern-matching-related methods to [PageAction].
extension PageActionPatterns on PageAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( PageAction_Click value)?  click,TResult Function( PageAction_TypeText value)?  typeText,TResult Function( PageAction_Press value)?  press,TResult Function( PageAction_Scroll value)?  scroll,TResult Function( PageAction_Wait value)?  wait,TResult Function( PageAction_Screenshot value)?  screenshot,TResult Function( PageAction_ExecuteJs value)?  executeJs,TResult Function( PageAction_Scrape value)?  scrape,required TResult orElse(),}){
final _that = this;
switch (_that) {
case PageAction_Click() when click != null:
return click(_that);case PageAction_TypeText() when typeText != null:
return typeText(_that);case PageAction_Press() when press != null:
return press(_that);case PageAction_Scroll() when scroll != null:
return scroll(_that);case PageAction_Wait() when wait != null:
return wait(_that);case PageAction_Screenshot() when screenshot != null:
return screenshot(_that);case PageAction_ExecuteJs() when executeJs != null:
return executeJs(_that);case PageAction_Scrape() when scrape != null:
return scrape(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( PageAction_Click value)  click,required TResult Function( PageAction_TypeText value)  typeText,required TResult Function( PageAction_Press value)  press,required TResult Function( PageAction_Scroll value)  scroll,required TResult Function( PageAction_Wait value)  wait,required TResult Function( PageAction_Screenshot value)  screenshot,required TResult Function( PageAction_ExecuteJs value)  executeJs,required TResult Function( PageAction_Scrape value)  scrape,}){
final _that = this;
switch (_that) {
case PageAction_Click():
return click(_that);case PageAction_TypeText():
return typeText(_that);case PageAction_Press():
return press(_that);case PageAction_Scroll():
return scroll(_that);case PageAction_Wait():
return wait(_that);case PageAction_Screenshot():
return screenshot(_that);case PageAction_ExecuteJs():
return executeJs(_that);case PageAction_Scrape():
return scrape(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( PageAction_Click value)?  click,TResult? Function( PageAction_TypeText value)?  typeText,TResult? Function( PageAction_Press value)?  press,TResult? Function( PageAction_Scroll value)?  scroll,TResult? Function( PageAction_Wait value)?  wait,TResult? Function( PageAction_Screenshot value)?  screenshot,TResult? Function( PageAction_ExecuteJs value)?  executeJs,TResult? Function( PageAction_Scrape value)?  scrape,}){
final _that = this;
switch (_that) {
case PageAction_Click() when click != null:
return click(_that);case PageAction_TypeText() when typeText != null:
return typeText(_that);case PageAction_Press() when press != null:
return press(_that);case PageAction_Scroll() when scroll != null:
return scroll(_that);case PageAction_Wait() when wait != null:
return wait(_that);case PageAction_Screenshot() when screenshot != null:
return screenshot(_that);case PageAction_ExecuteJs() when executeJs != null:
return executeJs(_that);case PageAction_Scrape() when scrape != null:
return scrape(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String selector)?  click,TResult Function( String selector,  String text)?  typeText,TResult Function( String key)?  press,TResult Function( ScrollDirection direction,  String selector,  PlatformInt64 amount)?  scroll,TResult Function( PlatformInt64 milliseconds,  String selector)?  wait,TResult Function( bool fullPage)?  screenshot,TResult Function( String script)?  executeJs,TResult Function()?  scrape,required TResult orElse(),}) {final _that = this;
switch (_that) {
case PageAction_Click() when click != null:
return click(_that.selector);case PageAction_TypeText() when typeText != null:
return typeText(_that.selector,_that.text);case PageAction_Press() when press != null:
return press(_that.key);case PageAction_Scroll() when scroll != null:
return scroll(_that.direction,_that.selector,_that.amount);case PageAction_Wait() when wait != null:
return wait(_that.milliseconds,_that.selector);case PageAction_Screenshot() when screenshot != null:
return screenshot(_that.fullPage);case PageAction_ExecuteJs() when executeJs != null:
return executeJs(_that.script);case PageAction_Scrape() when scrape != null:
return scrape();case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String selector)  click,required TResult Function( String selector,  String text)  typeText,required TResult Function( String key)  press,required TResult Function( ScrollDirection direction,  String selector,  PlatformInt64 amount)  scroll,required TResult Function( PlatformInt64 milliseconds,  String selector)  wait,required TResult Function( bool fullPage)  screenshot,required TResult Function( String script)  executeJs,required TResult Function()  scrape,}) {final _that = this;
switch (_that) {
case PageAction_Click():
return click(_that.selector);case PageAction_TypeText():
return typeText(_that.selector,_that.text);case PageAction_Press():
return press(_that.key);case PageAction_Scroll():
return scroll(_that.direction,_that.selector,_that.amount);case PageAction_Wait():
return wait(_that.milliseconds,_that.selector);case PageAction_Screenshot():
return screenshot(_that.fullPage);case PageAction_ExecuteJs():
return executeJs(_that.script);case PageAction_Scrape():
return scrape();}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String selector)?  click,TResult? Function( String selector,  String text)?  typeText,TResult? Function( String key)?  press,TResult? Function( ScrollDirection direction,  String selector,  PlatformInt64 amount)?  scroll,TResult? Function( PlatformInt64 milliseconds,  String selector)?  wait,TResult? Function( bool fullPage)?  screenshot,TResult? Function( String script)?  executeJs,TResult? Function()?  scrape,}) {final _that = this;
switch (_that) {
case PageAction_Click() when click != null:
return click(_that.selector);case PageAction_TypeText() when typeText != null:
return typeText(_that.selector,_that.text);case PageAction_Press() when press != null:
return press(_that.key);case PageAction_Scroll() when scroll != null:
return scroll(_that.direction,_that.selector,_that.amount);case PageAction_Wait() when wait != null:
return wait(_that.milliseconds,_that.selector);case PageAction_Screenshot() when screenshot != null:
return screenshot(_that.fullPage);case PageAction_ExecuteJs() when executeJs != null:
return executeJs(_that.script);case PageAction_Scrape() when scrape != null:
return scrape();case _:
  return null;

}
}

}

/// @nodoc


class PageAction_Click extends PageAction {
  const PageAction_Click({required this.selector}): super._();


/// CSS selector for the element to click.
 final  String selector;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_ClickCopyWith<PageAction_Click> get copyWith => _$PageAction_ClickCopyWithImpl<PageAction_Click>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_Click&&(identical(other.selector, selector) || other.selector == selector));
}


@override
int get hashCode => Object.hash(runtimeType,selector);

@override
String toString() {
  return 'PageAction.click(selector: $selector)';
}


}

/// @nodoc
abstract mixin class $PageAction_ClickCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_ClickCopyWith(PageAction_Click value, $Res Function(PageAction_Click) _then) = _$PageAction_ClickCopyWithImpl;
@useResult
$Res call({
 String selector
});




}
/// @nodoc
class _$PageAction_ClickCopyWithImpl<$Res>
    implements $PageAction_ClickCopyWith<$Res> {
  _$PageAction_ClickCopyWithImpl(this._self, this._then);

  final PageAction_Click _self;
  final $Res Function(PageAction_Click) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? selector = null,}) {
  return _then(PageAction_Click(
selector: null == selector ? _self.selector : selector // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PageAction_TypeText extends PageAction {
  const PageAction_TypeText({required this.selector, required this.text}): super._();


/// CSS selector for the input element.
 final  String selector;
/// Text to type into the element.
 final  String text;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_TypeTextCopyWith<PageAction_TypeText> get copyWith => _$PageAction_TypeTextCopyWithImpl<PageAction_TypeText>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_TypeText&&(identical(other.selector, selector) || other.selector == selector)&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,selector,text);

@override
String toString() {
  return 'PageAction.typeText(selector: $selector, text: $text)';
}


}

/// @nodoc
abstract mixin class $PageAction_TypeTextCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_TypeTextCopyWith(PageAction_TypeText value, $Res Function(PageAction_TypeText) _then) = _$PageAction_TypeTextCopyWithImpl;
@useResult
$Res call({
 String selector, String text
});




}
/// @nodoc
class _$PageAction_TypeTextCopyWithImpl<$Res>
    implements $PageAction_TypeTextCopyWith<$Res> {
  _$PageAction_TypeTextCopyWithImpl(this._self, this._then);

  final PageAction_TypeText _self;
  final $Res Function(PageAction_TypeText) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? selector = null,Object? text = null,}) {
  return _then(PageAction_TypeText(
selector: null == selector ? _self.selector : selector // ignore: cast_nullable_to_non_nullable
as String,text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PageAction_Press extends PageAction {
  const PageAction_Press({required this.key}): super._();


/// Key name to press.
 final  String key;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_PressCopyWith<PageAction_Press> get copyWith => _$PageAction_PressCopyWithImpl<PageAction_Press>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_Press&&(identical(other.key, key) || other.key == key));
}


@override
int get hashCode => Object.hash(runtimeType,key);

@override
String toString() {
  return 'PageAction.press(key: $key)';
}


}

/// @nodoc
abstract mixin class $PageAction_PressCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_PressCopyWith(PageAction_Press value, $Res Function(PageAction_Press) _then) = _$PageAction_PressCopyWithImpl;
@useResult
$Res call({
 String key
});




}
/// @nodoc
class _$PageAction_PressCopyWithImpl<$Res>
    implements $PageAction_PressCopyWith<$Res> {
  _$PageAction_PressCopyWithImpl(this._self, this._then);

  final PageAction_Press _self;
  final $Res Function(PageAction_Press) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? key = null,}) {
  return _then(PageAction_Press(
key: null == key ? _self.key : key // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PageAction_Scroll extends PageAction {
  const PageAction_Scroll({required this.direction, required this.selector, required this.amount}): super._();


/// Direction to scroll.
 final  ScrollDirection direction;
/// Optional CSS selector for a scrollable element. Scrolls the page if absent.
 final  String selector;
/// Optional pixel amount to scroll. Uses a default if absent.
 final  PlatformInt64 amount;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_ScrollCopyWith<PageAction_Scroll> get copyWith => _$PageAction_ScrollCopyWithImpl<PageAction_Scroll>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_Scroll&&(identical(other.direction, direction) || other.direction == direction)&&(identical(other.selector, selector) || other.selector == selector)&&(identical(other.amount, amount) || other.amount == amount));
}


@override
int get hashCode => Object.hash(runtimeType,direction,selector,amount);

@override
String toString() {
  return 'PageAction.scroll(direction: $direction, selector: $selector, amount: $amount)';
}


}

/// @nodoc
abstract mixin class $PageAction_ScrollCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_ScrollCopyWith(PageAction_Scroll value, $Res Function(PageAction_Scroll) _then) = _$PageAction_ScrollCopyWithImpl;
@useResult
$Res call({
 ScrollDirection direction, String selector, PlatformInt64 amount
});




}
/// @nodoc
class _$PageAction_ScrollCopyWithImpl<$Res>
    implements $PageAction_ScrollCopyWith<$Res> {
  _$PageAction_ScrollCopyWithImpl(this._self, this._then);

  final PageAction_Scroll _self;
  final $Res Function(PageAction_Scroll) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? direction = null,Object? selector = null,Object? amount = null,}) {
  return _then(PageAction_Scroll(
direction: null == direction ? _self.direction : direction // ignore: cast_nullable_to_non_nullable
as ScrollDirection,selector: null == selector ? _self.selector : selector // ignore: cast_nullable_to_non_nullable
as String,amount: null == amount ? _self.amount : amount // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

/// @nodoc


class PageAction_Wait extends PageAction {
  const PageAction_Wait({required this.milliseconds, required this.selector}): super._();


/// Milliseconds to wait. Ignored if `selector` is provided.
 final  PlatformInt64 milliseconds;
/// CSS selector to wait for.
 final  String selector;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_WaitCopyWith<PageAction_Wait> get copyWith => _$PageAction_WaitCopyWithImpl<PageAction_Wait>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_Wait&&(identical(other.milliseconds, milliseconds) || other.milliseconds == milliseconds)&&(identical(other.selector, selector) || other.selector == selector));
}


@override
int get hashCode => Object.hash(runtimeType,milliseconds,selector);

@override
String toString() {
  return 'PageAction.wait(milliseconds: $milliseconds, selector: $selector)';
}


}

/// @nodoc
abstract mixin class $PageAction_WaitCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_WaitCopyWith(PageAction_Wait value, $Res Function(PageAction_Wait) _then) = _$PageAction_WaitCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 milliseconds, String selector
});




}
/// @nodoc
class _$PageAction_WaitCopyWithImpl<$Res>
    implements $PageAction_WaitCopyWith<$Res> {
  _$PageAction_WaitCopyWithImpl(this._self, this._then);

  final PageAction_Wait _self;
  final $Res Function(PageAction_Wait) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? milliseconds = null,Object? selector = null,}) {
  return _then(PageAction_Wait(
milliseconds: null == milliseconds ? _self.milliseconds : milliseconds // ignore: cast_nullable_to_non_nullable
as PlatformInt64,selector: null == selector ? _self.selector : selector // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PageAction_Screenshot extends PageAction {
  const PageAction_Screenshot({required this.fullPage}): super._();


/// Whether to capture the full scrollable page. Defaults to viewport only.
///
/// Accepts both the canonical `fullPage` (camelCase) form and the
/// `full_page` (snake_case) alias so language bindings and fixtures can
/// use either convention without error.
 final  bool fullPage;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_ScreenshotCopyWith<PageAction_Screenshot> get copyWith => _$PageAction_ScreenshotCopyWithImpl<PageAction_Screenshot>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_Screenshot&&(identical(other.fullPage, fullPage) || other.fullPage == fullPage));
}


@override
int get hashCode => Object.hash(runtimeType,fullPage);

@override
String toString() {
  return 'PageAction.screenshot(fullPage: $fullPage)';
}


}

/// @nodoc
abstract mixin class $PageAction_ScreenshotCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_ScreenshotCopyWith(PageAction_Screenshot value, $Res Function(PageAction_Screenshot) _then) = _$PageAction_ScreenshotCopyWithImpl;
@useResult
$Res call({
 bool fullPage
});




}
/// @nodoc
class _$PageAction_ScreenshotCopyWithImpl<$Res>
    implements $PageAction_ScreenshotCopyWith<$Res> {
  _$PageAction_ScreenshotCopyWithImpl(this._self, this._then);

  final PageAction_Screenshot _self;
  final $Res Function(PageAction_Screenshot) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? fullPage = null,}) {
  return _then(PageAction_Screenshot(
fullPage: null == fullPage ? _self.fullPage : fullPage // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc


class PageAction_ExecuteJs extends PageAction {
  const PageAction_ExecuteJs({required this.script}): super._();


/// JavaScript source code to execute. Max 1 MB.
 final  String script;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PageAction_ExecuteJsCopyWith<PageAction_ExecuteJs> get copyWith => _$PageAction_ExecuteJsCopyWithImpl<PageAction_ExecuteJs>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_ExecuteJs&&(identical(other.script, script) || other.script == script));
}


@override
int get hashCode => Object.hash(runtimeType,script);

@override
String toString() {
  return 'PageAction.executeJs(script: $script)';
}


}

/// @nodoc
abstract mixin class $PageAction_ExecuteJsCopyWith<$Res> implements $PageActionCopyWith<$Res> {
  factory $PageAction_ExecuteJsCopyWith(PageAction_ExecuteJs value, $Res Function(PageAction_ExecuteJs) _then) = _$PageAction_ExecuteJsCopyWithImpl;
@useResult
$Res call({
 String script
});




}
/// @nodoc
class _$PageAction_ExecuteJsCopyWithImpl<$Res>
    implements $PageAction_ExecuteJsCopyWith<$Res> {
  _$PageAction_ExecuteJsCopyWithImpl(this._self, this._then);

  final PageAction_ExecuteJs _self;
  final $Res Function(PageAction_ExecuteJs) _then;

/// Create a copy of PageAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? script = null,}) {
  return _then(PageAction_ExecuteJs(
script: null == script ? _self.script : script // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class PageAction_Scrape extends PageAction {
  const PageAction_Scrape(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PageAction_Scrape);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PageAction.scrape()';
}


}




// dart format on
