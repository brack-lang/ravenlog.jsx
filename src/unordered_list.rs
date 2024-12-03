use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_unordered_list() -> Metadata {
    Metadata {
        command_name: "-".to_string(),
        call_name: "unordered_list".to_string(),
        argument_types: vec![("items".to_string(), Type::TArray(Box::new(Type::TInline)))],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn unordered_list(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{ravenlog.- item1, item2}}"),
            1,
        ));
    }
    let items = match &args[0] {
        Value::TextArray(items) => items,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("items must be Value::TextArray"),
                1,
            ))
        }
    };
    let mut items_str = String::new();
    for item in items {
        if item.starts_with("<UnorderedList>") || item.starts_with("<OrderedList>") {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("items cannot contain another list"),
                1,
            ));
        } else {
            items_str.push_str(&format!("<ListItem>{}</ListItem>", item));
        }
    }
    Ok(format!("<UnorderedList>{}</UnorderedList>", items_str))
}
