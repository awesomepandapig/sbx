use crate::messages::ExecutionReportMessage;
use crate::orderbook::OrderBook;
use crate::processors::level2::format_l2_update_json;

use std::string::ToString;

use sbe::exec_type_enum::ExecTypeEnum;
use sbe::side_enum::SideEnum;

use chrono::{DateTime, SecondsFormat, TimeZone, Utc};

use tracing::warn;

pub fn process_execution_report(
    book: &mut OrderBook,
    report: &ExecutionReportMessage,
) -> Option<String> {
    let mut update_to_send: Option<String> = None;
    match report.exec_type {
        ExecTypeEnum::New => {
            assert_eq!(
                report.order_id,
                book.last_seen_id + 1,
                "Error: Recieved id {} expected {}",
                report.order_id,
                book.last_seen_id + 1
            );
            book.last_seen_id += 1;

            if report.price != i64::MIN {
                let new_quantity = book.add_order(&report);
                update_to_send = Some(
                    format_l2_update_json(
                        report.side.to_string().to_lowercase(),
                        format_nanosecond_timestamp(&report.transact_time),
                        format_decimal_with_exponent_neg8(&report.price),
                        format_decimal_with_exponent_neg8(&new_quantity),
                        format_symbol(&report.symbol),
                    )
                    .expect("Failed to serialize L2Update"),
                ); // TODO: NO EXPECTS
            }
        }
        ExecTypeEnum::Trade => {
            if report.price != i64::MIN {
                let price_level_exists = match report.side {
                    SideEnum::Buy => book.bids.contains_key(&report.price),
                    SideEnum::Sell => book.asks.contains_key(&report.price),
                    _ => false,
                };

                assert!(
                    price_level_exists,
                    "Assumption Violation: fill_order called for non-existent price level {} on side {:?}",
                    report.price, report.side
                );

                let new_quantity = book.fill_order(&report);

                update_to_send = Some(
                    format_l2_update_json(
                        report.side.to_string().to_lowercase(),
                        format_nanosecond_timestamp(&report.transact_time),
                        format_decimal_with_exponent_neg8(&report.price),
                        format_decimal_with_exponent_neg8(&new_quantity),
                        format_symbol(&report.symbol),
                    )
                    .expect("Failed to serialize L2Update"),
                ); // TODO: NO EXPECTS
            }
        }
        ExecTypeEnum::Canceled => {
            if report.price != i64::MIN {
                let new_quantity = book.remove_order(&report);

                update_to_send = Some(
                    format_l2_update_json(
                        report.side.to_string().to_lowercase(),
                        format_nanosecond_timestamp(&report.transact_time),
                        format_decimal_with_exponent_neg8(&report.price),
                        format_decimal_with_exponent_neg8(&new_quantity),
                        format_symbol(&report.symbol),
                    )
                    .expect("Failed to serialize L2Update"),
                ); // TODO: NO EXPECTS
            }
        }
        ExecTypeEnum::Rejected => {
            assert_eq!(
                report.order_id,
                book.last_seen_id + 1,
                "Error: Recieved id {} expected {}",
                report.order_id,
                book.last_seen_id + 1
            );
            book.last_seen_id += 1;
            warn!("Rejected order: {}", report.order_id);
        }
        _ => {
            // Handle other execution types or ignore
        }
    }
    update_to_send
}

fn format_decimal_with_exponent_neg8(mantissa: &i64) -> String {
    const SCALE: i64 = 100_000_000;

    let integral = mantissa / SCALE;
    let fractional_part = (mantissa % SCALE).abs();

    if fractional_part == 0 {
        return integral.to_string();
    }

    let mut fractional_str = format!("{:08}", fractional_part);
    while fractional_str.ends_with('0') {
        fractional_str.pop();
    }

    if fractional_str.is_empty() {
        integral.to_string()
    } else {
        format!("{}.{}", integral, fractional_str)
    }
}

fn format_nanosecond_timestamp(timestamp: &u64) -> String {
    let secs = timestamp / 1_000_000_000;
    let nanos = (timestamp % 1_000_000_000) as u32;
    let datetime: DateTime<Utc> = Utc.timestamp_opt(secs.try_into().unwrap(), nanos).unwrap(); // TODO: DONT UNWRAP IN PROD
    datetime.to_rfc3339_opts(SecondsFormat::Nanos, true)
}

fn format_symbol(symbol_bytes: &[u8; 6]) -> String {
    String::from_utf8_lossy(symbol_bytes)
        .trim_end_matches(['\0', ' ']) // Trim both null characters and spaces.
        .to_string()
}
