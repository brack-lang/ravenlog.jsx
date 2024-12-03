use bold::metadata_bold;
use heading::metadata_headings;
use brack_pdk_rs::metadata::Metadata;
use extism_pdk::{plugin_fn, FnResult, Json};

pub mod bold;
pub mod heading;

#[plugin_fn]
pub fn get_metadata() -> FnResult<Json<Vec<Metadata>>> {
    let mut metadata = Vec::new();
    metadata.push(metadata_bold());
    metadata.append(&mut metadata_headings());
    Ok(Json(metadata))
}
