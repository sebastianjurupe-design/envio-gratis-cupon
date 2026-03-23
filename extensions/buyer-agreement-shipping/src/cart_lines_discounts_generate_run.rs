use super::schema;
use shopify_function::{shopify_function, Result};

#[shopify_function]
fn cart_lines_discounts_generate_run(
    _input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<schema::CartLinesDiscountsGenerateRunResult> {
    Ok(schema::CartLinesDiscountsGenerateRunResult {
        operations: vec![],
    })
}