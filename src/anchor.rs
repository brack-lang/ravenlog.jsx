use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_anchor() -> Metadata {
    Metadata {
        command_name: "@".to_string(),
        call_name: "anchor".to_string(),
        argument_types: vec![
            ("href".to_string(), Type::TInline),
            ("text".to_string(), Type::TInline),
        ],
        return_type: Type::TInline,
    }
}

#[plugin_fn]
pub fn anchor(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: [ravenlog.@ href, text]"),
            1,
        ));
    }
    let href = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("href must be Value::Text"),
                1,
            ))
        }
    };
    let text = match &args[1] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::Text"),
                1,
            ))
        }
    };
    Ok(format!("<Anchor href=\"{}\">{}</Anchor>", href, text))
}
