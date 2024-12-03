use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_strike() -> Metadata {
    Metadata {
        command_name: "~".to_string(),
        call_name: "strike".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TInline,
    }
}

#[plugin_fn]
pub fn strike(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: [ravenlog.~ text]"),
            1,
        ));
    }
    let text = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::Text"),
                1,
            ))
        }
    };
    Ok(format!("<Strike>{}</Strike>", text))
}
