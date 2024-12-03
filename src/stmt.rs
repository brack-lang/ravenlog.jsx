use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_stmt() -> Metadata {
    Metadata {
        command_name: "stmt".to_string(),
        call_name: "stmt".to_string(),
        argument_types: vec![("stmt".to_string(), Type::TBlock)],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn stmt(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(anyhow::anyhow!("stmt failed"), 1));
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
    Ok(format!("<Stmt>{}</Stmt>", text))
}
