use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_image() -> Metadata {
    Metadata {
        command_name: "img".to_string(),
        call_name: "image".to_string(),
        argument_types: vec![("src".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn image(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.img src}}"),
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
    Ok(format!("<RavenlogImage src=\"{}\"></RavenlogImage>", src))
}
