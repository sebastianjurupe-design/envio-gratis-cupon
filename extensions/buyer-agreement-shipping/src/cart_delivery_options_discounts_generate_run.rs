use super::schema;
use shopify_function::prelude::*;
use shopify_function::{shopify_function, Result};

#[shopify_function]
fn cart_delivery_options_discounts_generate_run(
    input: schema::cart_delivery_options_discounts_generate_run::Input,
) -> Result<schema::CartDeliveryOptionsDiscountsGenerateRunResult> {
    let has_shipping_discount_class = input
        .discount()
        .discount_classes()
        .contains(&schema::DiscountClass::Shipping);

    if !has_shipping_discount_class {
        return Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult {
            operations: vec![],
        });
    }

    let delivery_type = input
        .cart()
        .delivery_type()
        .and_then(|attr| attr.value())
        .map(|v| v.as_str())
        .unwrap_or("");

    if delivery_type != "Acordar con el comprador" {
        return Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult {
            operations: vec![],
        });
    }

    let candidates: Vec<schema::DeliveryDiscountCandidate> = input
        .cart()
        .delivery_groups()
        .iter()
        .flat_map(|group| {
            group.delivery_options().iter().map(|option| {
                schema::DeliveryDiscountCandidate {
                    message: Some("Envío a coordinar".to_string()),
                    targets: vec![
                        schema::DeliveryDiscountCandidateTarget::DeliveryOption(
                            schema::DeliveryOptionTarget {
                                handle: option.handle().to_string(),
                            },
                        ),
                    ],
                    value: schema::DeliveryDiscountCandidateValue::Percentage(
                        schema::Percentage {
                            value: Decimal::from(100.0),
                        },
                    ),
                    associated_discount_code: None,
                }
            })
        })
        .collect();

    if candidates.is_empty() {
        return Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult {
            operations: vec![],
        });
    }

    Ok(schema::CartDeliveryOptionsDiscountsGenerateRunResult {
        operations: vec![schema::DeliveryOperation::DeliveryDiscountsAdd(
            schema::DeliveryDiscountsAddOperation {
                selection_strategy: schema::DeliveryDiscountSelectionStrategy::All,
                candidates,
            },
        )],
    })
}