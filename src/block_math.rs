use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_block_math() ->Metadata {
    Metadata {
        command_name: "$".to_string(),
        call_name: "block_formula".to_string(),
        argument_types: vec![("formula".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn block_formula(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.$ text}}"),
            1,
        ));
    }
    let formula = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("formula must be Value::Text"),
                1,
            ))
        }
    };
    Ok(format!("<BlockMath>{}</BlockMath>", formula))
}
