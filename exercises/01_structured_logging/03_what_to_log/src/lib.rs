mod logger;

use std::time::Instant;

use log::{error, info};
pub use logger::TestLogger;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Add log statements to `get_total` and `get_order_details`, our two units of work, to capture
/// the data points we discussed:
/// - the start and end of each unit of work
/// - the duration of each unit of work
/// - the outcome of each unit of work
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    let start_time = Instant::now();
    let mut sum = 0;
    info!("START - process total price");
    for oid in order_numbers {
        let start_time = Instant::now();
        let order = get_order_details(*oid).map_err(|err| {
            error!(
                "END - process total price - ERROR - {:?}ms",
                start_time.elapsed().as_millis()
            );
            err
        })?;
        sum += order.price;
    }
    info!(
        "END - process total price - SUCCESS - {:?}ms",
        start_time.elapsed().as_millis()
    );
    // let sum = order_numbers
    //     .iter()
    //     .filter_map(|oid| get_order_details(*oid).map(|order| order.price).ok())
    //     .sum();
    Ok(sum)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    let start_time = Instant::now();
    info!("START - retrieve order");
    if order_number % 4 == 0 {
        error!(
            "END - retrieve order - ERROR - {}ms",
            start_time.elapsed().as_millis()
        );
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let prices = vec![999, 1089, 1029];
        info!(
            "END - retrieve order - SUCCESS - {}ms",
            start_time.elapsed().as_millis()
        );
        Ok(OrderDetails {
            order_number,
            price: prices[order_number as usize % prices.len()],
        })
    }
}
