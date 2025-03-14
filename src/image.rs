use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_image() -> Metadata {
    Metadata {
        command_name: "img".to_string(),
        call_name: "image".to_string(),
        argument_types: vec![("src".to_string(), Type::TInline), ("alt".to_string(), Type::TInline), ("caption".to_string(), Type::TOption(Box::new(Type::TInline)))],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn image(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 3 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage:
    1. {{ravenlog.img src, alt}}
    2. {{ravenlog.img src, alt, caption}}"),
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
    let alt = match &args[1] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("alt must be Value::Text"),
                1,
            ))
        }
    };
    let caption = match &args[2] {
        Value::TextOption(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("caption must be Value::TextOption"),
                1,
            ))
        }
    };
    match caption {
        Some(caption) => Ok(format!("<RavenlogImage src=\"{}\" alt=\"{}\" caption=\"{}\"></RavenlogImage>", src, alt, caption)),
        None => Ok(format!("<RavenlogImage src=\"{}\" alt=\"{}\"></RavenlogImage>", src, alt)),
    }
}
