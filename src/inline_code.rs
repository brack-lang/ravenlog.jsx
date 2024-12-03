use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_inline_code() -> Metadata {
    Metadata {
        command_name: "`".to_string(),
        call_name: "inline_code".to_string(),
        argument_types: vec![("code".to_string(), Type::TInline)],
        return_type: Type::TInline,
    }
}

#[plugin_fn]
pub fn inline_code(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: [ravenlog.` code]"),
            1,
        ));
    }
    let code = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("code must be Value::Text"),
                1,
            ))
        }
    };
    Ok(format!("<InlineCode>{}</InlineCode>", code))
}
