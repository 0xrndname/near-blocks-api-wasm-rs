use futures::future::join;
use std::panic;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

mod near_blocks;
mod time;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    setup_refresh();
}

fn setup_refresh() {
    let update: Box<dyn Fn()> = {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let price_usd_elem = Rc::new(
            document
                .query_selector("#price_usd")
                .expect("query_selector failed")
                .expect("missing #price_usd"),
        );
        let price_btc_elem = Rc::new(
            document
                .query_selector("#price_btc")
                .expect("query_selector failed")
                .expect("missing #price_btc"),
        );
        let txs_elem = Rc::new(
            document
                .query_selector("#txs")
                .expect("query_selector failed")
                .expect("missing #txs"),
        );

        Box::new(move || {
            let price_usd_elem = price_usd_elem.clone();
            let price_btc_elem = price_btc_elem.clone();
            let txs_elem = txs_elem.clone();

            spawn_local(async move {
                join(
                    update_price(&price_usd_elem, &price_btc_elem),
                    update_txs(&txs_elem),
                )
                .await;
            })
        })
    };
    update();

    let window = web_sys::window().expect("no global `window` exists");
    let closure = Closure::wrap(update);
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        )
        .unwrap();
    closure.forget();
}

async fn update_price(usd_elem: &web_sys::Element, btc_elem: &web_sys::Element) {
    match near_blocks::get_price().await {
        Ok(p) => {
            usd_elem.set_text_content(Some(&format!("${}", p.usd)));
            btc_elem.set_text_content(Some(&format!("₿{}", p.btc)));
        }
        Err(_) => {
            usd_elem.set_text_content(Some("❌"));
            btc_elem.set_text_content(Some("❌"));
        }
    };
}

async fn update_txs(txs_elem: &web_sys::Element) {
    match near_blocks::get_transactions().await {
        Ok(txs) => {
            let txs_list = txs
                .txns
                .iter()
                .map(|tx| {
                    let timestamp_str = time::unixts_to_string(tx.block_timestamp);
                    let dep_val: f32 = lexical::parse(tx.deposit_value.as_bytes()).unwrap();
                    let fee: f32 = lexical::parse(tx.transaction_fee.as_bytes()).unwrap();
                    format!(
                        "{} {}... \"{}\" {}({})Ⓝ {} -> {}",
                        timestamp_str,
                        &tx.transaction_hash[0..7],
                        tx.tx_type,
                        dep_val / 1000000000000000000000000.0,
                        fee / 1000000000000000000000000.0,
                        tx.from,
                        tx.to
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            txs_elem.set_text_content(Some(&txs_list));
        }
        Err(e) => {
            log::error!("{}", e)
        }
    }
}
