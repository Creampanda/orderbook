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

    new_orderbook.print_orderbook();

    let new_order_3 = order::Order::create_order(
        String::from("Mark"),
        trading_pair.clone(),
        order::OrderDirection::BUY,
        120,
        99,
    );
    new_orderbook.add_order(new_order_3);

    new_orderbook.print_orderbook();

    let alice_order = alice_wallet.create_order(order::OrderDirection::SELL, 100, 102);
    new_orderbook.add_order(alice_order.unwrap());

    new_orderbook.print_orderbook();

    let alice_order_2 = alice_wallet.create_order(order::OrderDirection::SELL, 50, 103);
    let alice_order_3 = alice_wallet.create_order(order::OrderDirection::SELL, 75, 101);
    new_orderbook.add_order(alice_order_2.unwrap());

    new_orderbook.print_orderbook();
    new_orderbook.add_order(alice_order_3.unwrap());

    new_orderbook.print_orderbook();
    let alice_order_4 = alice_wallet.create_order(order::OrderDirection::BUY, 33, 98);
    let alice_order_5 = alice_wallet.create_order(order::OrderDirection::BUY, 42, 97);
    new_orderbook.add_order(alice_order_4.unwrap());

    new_orderbook.print_orderbook();
    new_orderbook.add_order(alice_order_5.unwrap());

    new_orderbook.print_orderbook();

    let alice_order_6 = alice_wallet.create_order(order::OrderDirection::BUY, 111, 91);
    new_orderbook.add_order(alice_order_6.unwrap());
    new_orderbook.print_orderbook();

    let alice_order_7 = alice_wallet.create_order(order::OrderDirection::SELL, 100, 90);
    new_orderbook.add_order(alice_order_7.unwrap());

    new_orderbook.print_orderbook();

    let alice_order_8 = alice_wallet.create_order(order::OrderDirection::SELL, 100, 90);
    new_orderbook.add_order(alice_order_8.unwrap());

    new_orderbook.print_orderbook();

    let alice_order_9 = alice_wallet.create_order(order::OrderDirection::BUY, 110, 105);
    new_orderbook.add_order(alice_order_9.unwrap());

    new_orderbook.print_orderbook();
}
