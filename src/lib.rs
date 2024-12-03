use anchor::metadata_anchor;
use block_math::metadata_block_math;
use block_quote::metadata_block_quote;
use bold::metadata_bold;
use brack_pdk_rs::metadata::Metadata;
use code_block::metadata_code_block;
use extism_pdk::{plugin_fn, FnResult, Json};
use heading::metadata_headings;
use image::metadata_image;
use inline_code::metadata_inline_code;
use inline_math::metadata_inline_math;
use inline_quote::metadata_inline_quote;
use italic::metadata_italic;
use ordered_list::metadata_ordered_list;
use spotify::metadata_spotify;
use stmt::metadata_stmt;
use strike::metadata_strike;
use unordered_list::metadata_unordered_list;

pub mod anchor;
pub mod block_math;
pub mod block_quote;
pub mod bold;
pub mod code_block;
pub mod heading;
pub mod image;
pub mod inline_code;
pub mod inline_math;
pub mod inline_quote;
pub mod italic;
pub mod ordered_list;
pub mod spotify;
pub mod stmt;
pub mod strike;
pub mod unordered_list;

#[plugin_fn]
pub fn get_metadata() -> FnResult<Json<Vec<Metadata>>> {
    let mut metadata = Vec::new();
    metadata.push(metadata_anchor());
    metadata.push(metadata_block_math());
    metadata.push(metadata_block_quote());
    metadata.push(metadata_bold());
    metadata.push(metadata_code_block());
    metadata.append(&mut metadata_headings());
    metadata.push(metadata_image());
    metadata.push(metadata_inline_code());
    metadata.push(metadata_inline_quote());
    metadata.push(metadata_inline_math());
    metadata.push(metadata_italic());
    metadata.push(metadata_ordered_list());
    metadata.push(metadata_spotify());
    metadata.push(metadata_stmt());
    metadata.push(metadata_strike());
    metadata.push(metadata_unordered_list());
    Ok(Json(metadata))
}
