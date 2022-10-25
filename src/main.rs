mod order;
mod orderbook;
mod wallet;

use std::collections::HashMap;

use order::OrderDirection;
use orderbook::Orderbook;
use rand::Rng;
use wallet::{CurrencyName, Wallet};

fn main() {
    let mut rng = rand::thread_rng();
    let trading_pair = order::OrderPair::create_orderpair(CurrencyName::XTZ, CurrencyName::USDT);

    let mut new_orderbook = Orderbook::create_orderbook(trading_pair);
    let mut users: HashMap<u32, Wallet> = HashMap::new();
    let mut alice_wallet =
        Wallet::create_wallet(1, String::from("Alice"), String::from("alice_pub_key"));
    let mut bob_wallet = Wallet::create_wallet(2, String::from("Bob"), String::from("bob_pub_key"));
    let mut mark_wallet =
        Wallet::create_wallet(3, String::from("Mark"), String::from("mark_pub_key"));
    let mut ann_wallet = Wallet::create_wallet(4, String::from("Ann"), String::from("ann_pub_key"));
    users.insert(alice_wallet.get_id(), alice_wallet);
    users.insert(bob_wallet.get_id(), bob_wallet);
    users.insert(mark_wallet.get_id(), mark_wallet);
    users.insert(ann_wallet.get_id(), ann_wallet);
    for (id, user) in users.iter_mut() {
        user.deposite(CurrencyName::XTZ, 1000000);
        user.deposite(CurrencyName::USDT, 1000000);
        new_orderbook.add_order(
            user.create_order(
                OrderDirection::SELL,
                rng.gen_range(50..150),
                rng.gen_range(90..110),
            )
            .unwrap(),
        );
        new_orderbook.print_orderbook();
        new_orderbook.add_order(
            user.create_order(
                OrderDirection::BUY,
                rng.gen_range(50..150),
                rng.gen_range(90..110),
            )
            .unwrap(),
        );
        new_orderbook.print_orderbook();
    }
}
