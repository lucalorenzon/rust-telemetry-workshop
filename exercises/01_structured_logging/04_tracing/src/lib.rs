//! # Exercise
//!
//! We'll ignore the consumer side for now and focus on the instrumentation side.
//! We'll redo the previous exercise using the `tracing` crate in order to get familiar with the
//! basics.
mod subscriber;

pub use subscriber::init_test_subscriber;
use tracing::info_span;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Wrap `get_total` and `get_order_details`, our two units of work, in a `tracing::Span`.
/// We don't care about capturing the outcome of each unit of work (for now).
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    // Tip: use `tracing::info_span!` to create a new span.
    // You'll have to learn about the *RAII guard* pattern!
    let span = info_span!("process total price");
    span.in_scope(|| {
        let mut sum = 0;
        for oid in order_numbers {
            let order = get_order_details(*oid)?;
            sum += order.price;
        }
        Ok(sum)
    })
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    info_span!("retrieve order").in_scope(|| {
        if order_number % 4 == 0 {
            Err(anyhow::anyhow!("Failed to talk to the database"))
        } else {
            let prices = vec![999, 1089, 1029];
            Ok(OrderDetails {
                order_number,
                price: prices[order_number as usize % prices.len()],
            })
        }
    })
}
