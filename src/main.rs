mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

use rust_decimal_macros::dec;

fn main() {
    let buy_order_from_greed = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_zuki = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = Orderbook::new();
    orderbook.add_limit_order(dec!(4.4), buy_order_from_greed);
    orderbook.add_limit_order(dec!(4.4), buy_order_from_zuki);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_limit_order(dec!(20.0), sell_order);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine
        .place_limit_order(pair, dec!(10.000), buy_order)
        .unwrap();
}
