use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_headings() -> Vec<Metadata> {
    let mut metadata_vec = Vec::new();
    for i in 1..=6 {
        metadata_vec.push(Metadata {
            command_name: "*".repeat(i),
            call_name: format!("heading_level_{}", i),
            argument_types: vec![("text".to_string(), Type::TInline)],
            return_type: Type::TBlock,
        });
    }
    return metadata_vec;
}

#[plugin_fn]
pub fn heading_level_1(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.* text}}"),
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
    Ok(format!("<Heading1>{}</Heading1>", text))
}

#[plugin_fn]
pub fn heading_level_2(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.** text}}"),
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
    Ok(format!("<Heading2>{}</Heading2>", text))
}

#[plugin_fn]
pub fn heading_level_3(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.*** text}}"),
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
    Ok(format!("<Heading3>{}</Heading3>", text))
}

#[plugin_fn]
pub fn heading_level_4(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.**** text}}"),
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
    Ok(format!("<Heading4>{}</Heading4>", text))
}

#[plugin_fn]
pub fn heading_level_5(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.***** text}}"),
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
    Ok(format!("<Heading5>{}</Heading5>", text))
}

#[plugin_fn]
pub fn heading_level_6(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.****** text}}"),
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
    Ok(format!("<Heading6>{}</Heading6>", text))
}
