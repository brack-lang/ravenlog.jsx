use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_inline_quote() -> Metadata {
    Metadata {
        command_name: "quote".to_string(),
        call_name: "inline_quote".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TInline,
    }
}

#[plugin_fn]
pub fn inline_quote(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: [ravenlog.quote text]"),
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
    Ok(format!("<InlineQuote>{}</InlineQuote>", text))
}
