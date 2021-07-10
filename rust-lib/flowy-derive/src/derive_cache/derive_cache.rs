pub enum TypeCategory {
    Array,
    Map,
    Str,
    Protobuf,
    Bytes,
    Enum,
    Opt,
    Primitive,
}
// auto generate, do not edit
pub fn category_from_str(type_str: &str) -> TypeCategory {
    match type_str {
        "Vec" => TypeCategory::Array,
        "HashMap" => TypeCategory::Map,
        "u8" => TypeCategory::Bytes,
        "String" => TypeCategory::Str,
        "KeyValue"
        | "KeyValueQuery"
        | "FFIRequest"
        | "FFIResponse"
        | "User"
        | "SignUpRequest"
        | "SignUpParams"
        | "SignUpResponse"
        | "SignInRequest"
        | "SignInParams"
        | "SignInResponse"
        => TypeCategory::Protobuf,
        "FFIStatusCode"
        => TypeCategory::Enum,

        "Option" => TypeCategory::Opt,
        _ => TypeCategory::Primitive,
    }
}
