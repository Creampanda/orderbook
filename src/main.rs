mod order;
mod orderbook;
mod wallet;

use orderbook::Orderbook;
use wallet::CurrencyName;

fn main() {
    let mut alice_wallet =
        wallet::Wallet::create_wallet(1, String::from("Alice"), String::from("aaa"));
    alice_wallet.deposite(CurrencyName::USDT, 10000000);
    alice_wallet.deposite(CurrencyName::XTZ, 1000100000);
    alice_wallet.block(CurrencyName::USDT, 500);
    // println!("{:?}", alice_wallet);

    let trading_pair = order::OrderPair::create_orderpair(CurrencyName::XTZ, CurrencyName::USDT);
    let new_order = order::Order::create_order(
        String::from("Alice"),
        trading_pair.clone(),
        order::OrderDirection::BUY,
        100,
        1000,
    );

    let mut new_orderbook = Orderbook::create_orderbook(trading_pair.clone());
    new_orderbook.add_order(new_order);
    println!("\n\nAdded Alice BUY 1000 by 100");
    new_orderbook.print_orderbook();

    let new_order_2 = order::Order::create_order(
        String::from("Bob"),
        trading_pair.clone(),
        order::OrderDirection::SELL,
        100,
        1000,
    );
    new_orderbook.add_order(new_order_2);
    // new_orderbook.print_orderbook();

    println!("\n\nAdded Bob SELL 1000 by 100");
    new_orderbook.print_orderbook();

    let new_order_3 = order::Order::create_order(
        String::from("Mark"),
        trading_pair.clone(),
        order::OrderDirection::SELL,
        120,
        1000,
    );
    new_orderbook.add_order(new_order_3);

    println!("\n\nAdded Mark SELL 120 by 1000");
    new_orderbook.print_orderbook();

    let alice_order = alice_wallet.create_order(order::OrderDirection::SELL, 100, 100);
    new_orderbook.add_order(alice_order.unwrap());

    println!("\n\nAdded Alice SELL 100 by 100");
    new_orderbook.print_orderbook();

    let alice_order_2 = alice_wallet.create_order(order::OrderDirection::SELL, 50, 83);
    let alice_order_3 = alice_wallet.create_order(order::OrderDirection::SELL, 75, 90);
    new_orderbook.add_order(alice_order_2.unwrap());
    println!("\n\nAdded Alice SELL 50 by 83");
    new_orderbook.print_orderbook();
    new_orderbook.add_order(alice_order_3.unwrap());
    println!("\n\nAdded Alice SELL 75 by 90");
    new_orderbook.print_orderbook();
    let alice_order_4 = alice_wallet.create_order(order::OrderDirection::BUY, 33, 195);
    let alice_order_5 = alice_wallet.create_order(order::OrderDirection::BUY, 42, 199);
    new_orderbook.add_order(alice_order_4.unwrap());
    println!("\n\nAdded Alice BUY 33 by 195");
    new_orderbook.print_orderbook();
    new_orderbook.add_order(alice_order_5.unwrap());
    println!("\n\nAdded Alice BUY 42 by 199");
    new_orderbook.print_orderbook();
}
