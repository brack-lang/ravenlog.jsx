use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_code_block() -> Metadata {
    Metadata {
        command_name: "`".to_string(),
        call_name: "code_block".to_string(),
        argument_types: vec![("src".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn code_block(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.` src}}"),
            1,
        ));
    }
    let src = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("src must be Value::Text"),
                1,
            ))
        }
    };
    Ok(format!("<CodeBlock src=\"{}\"></CodeBlock>", src))
}
