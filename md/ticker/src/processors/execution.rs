use crate::messages::ExecutionReportMessage;
use crate::orderbook::OrderBook;
use crate::processors::ticker::TickerState;

use sbe::exec_type_enum::ExecTypeEnum;
use sbe::side_enum::SideEnum;

use tracing::info;

pub fn process_execution_report(
    book: &mut OrderBook,
    ticker_state: &mut TickerState,
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
                book.add_order(&report);
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

                book.fill_order(&report);
                ticker_state.update_on_match(&report);

                let ticker = ticker_state.create_ticker(book, report.transact_time);

                update_to_send = Some(
                    serde_json::to_string(&ticker)
                    .expect("Failed to serialize L2Update"), // TODO: NO EXPECTS
                );
            }
        }
        ExecTypeEnum::Canceled => {
            if report.price != i64::MIN {
                book.remove_order(&report);
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
        }
        _ => {
            // Handle other execution types or ignore
        }
    }
    update_to_send
}