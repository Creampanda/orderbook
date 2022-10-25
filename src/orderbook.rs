use indexmap::IndexMap;

use crate::order::{Order, OrderDirection, OrderPair};
struct Cup {
    order_list: IndexMap<u32, Vec<Order>>,
}

impl Cup {
    fn create_list() -> Self {
        Self {
            order_list: IndexMap::new(),
        }
    }
    fn add_order(&mut self, order: Order) {
        let order_price = order.get_price();
        if let Some(orders) = self.order_list.get_mut(&order_price) {
            orders.push(order);
        } else {
            self.order_list.insert(order_price, vec![order]);
        }
        self.order_list.sort_by(|k1, _, k2, _| k2.cmp(k1));
        // match order_direction {
        //     OrderDirection::SELL => self.order_list.sort_keys(),
        //     OrderDirection::BUY => self.order_list.sort_by(|k1, v1, k2, v2| k2.cmp(k1)),
        // }
    }

    fn match_orders(&mut self, incoming_order: &mut Order) -> Option<Order> {
        let incoming_price = incoming_order.get_price();
        let incoming_direction = incoming_order.get_direction();
        // let order_list =  match incoming_direction {
        //     OrderDirection::BUY => ,
        //     OrderDirection::SELL => self.order_list.into_iter().rev(),
        // };
        if matches!(incoming_direction, OrderDirection::SELL) {
            for (counter_orders_price, counter_orders) in self.order_list.iter_mut() {
                let incoming_qtty = incoming_order.get_quantity();
                if incoming_qtty == 0 {
                    break;
                }
                if *counter_orders_price >= incoming_price {
                    for counter_order in counter_orders.iter_mut() {
                        let counter_order_qtty = counter_order.get_quantity();
                        if counter_order_qtty > incoming_qtty {
                            let execute_quantity = counter_order_qtty - incoming_qtty;
                            counter_order.execute_part(execute_quantity);
                            incoming_order.execute_full();
                            break;
                        } else if counter_order_qtty == incoming_qtty {
                            incoming_order.execute_full();
                            counter_order.execute_full();
                            break;
                        } else {
                            let execute_quantity = incoming_qtty - counter_order_qtty;
                            counter_order.execute_full();
                            incoming_order.execute_part(execute_quantity)
                        }
                    }
                    counter_orders.retain(|order| order.get_quantity() > 0);
                }
            }
        } else {
            for (counter_orders_price, counter_orders) in self.order_list.iter_mut().rev() {
                let incoming_qtty = incoming_order.get_quantity();
                if incoming_qtty == 0 {
                    break;
                }
                if *counter_orders_price <= incoming_price {
                    for counter_order in counter_orders.iter_mut() {
                        let counter_order_qtty = counter_order.get_quantity();
                        if counter_order_qtty > incoming_qtty {
                            let execute_quantity = counter_order_qtty - incoming_qtty;
                            counter_order.execute_part(execute_quantity);
                            incoming_order.execute_full();
                            break;
                        } else if counter_order_qtty == incoming_qtty {
                            incoming_order.execute_full();
                            counter_order.execute_full();
                            break;
                        } else {
                            let execute_quantity = incoming_qtty - counter_order_qtty;
                            counter_order.execute_full();
                            incoming_order.execute_part(execute_quantity)
                        }
                    }
                    counter_orders.retain(|order| order.get_quantity() > 0);
                }
            }
        }

        self.order_list.retain(|k, v| !v.is_empty());
        let incoming_qtty = incoming_order.get_quantity();
        if incoming_qtty == 0 {
            return None;
        } else {
            Some(incoming_order.clone())
        }
    }

    fn print_cup(&self, direction: String) {
        for (price, order_list) in self.order_list.iter() {
            let mut total_quantity = 0;
            let mut clients = String::new();
            for order in order_list {
                if total_quantity > 0 {
                    clients += &format!(", {}", order.get_client_name());
                } else {
                    clients += &format!("{}", order.get_client_name());
                }

                total_quantity += order.get_quantity();
            }
            print!(
                "{:>7} | {:>7} | {:>7} | {}",
                price, total_quantity, direction, clients
            );
            println!()
        }
        println!()
    }
}
pub struct Orderbook {
    _instrument_id: OrderPair,
    bids: Cup,
    asks: Cup,
}

impl Orderbook {
    pub fn create_orderbook(_instrument_id: OrderPair) -> Self {
        Self {
            _instrument_id,
            bids: Cup::create_list(),
            asks: Cup::create_list(),
        }
    }

    pub fn add_order(&mut self, mut order: Order) {
        let direction = order.get_direction();
        println!(
            "\n\nIncoming order: {} {} {} by {}\n",
            order.get_client_name(),
            order.get_direction_str(),
            order.get_quantity(),
            order.get_price(),
        );
        match direction {
            OrderDirection::BUY => {
                let order_match_result = self.asks.match_orders(&mut order);
                if let Some(order_after_matching) = order_match_result {
                    self.bids.add_order(order_after_matching);
                }
            }
            OrderDirection::SELL => {
                let order_match_result = self.bids.match_orders(&mut order);
                if let Some(order_after_matching) = order_match_result {
                    self.asks.add_order(order_after_matching);
                }
            }
        }
    }

    pub fn print_orderbook(&self) {
        println!("{:>7} | {:>7} | {:>7} | {}", "PRICE", "QTTY", "DIR", "IDs",);
        println!();

        self.asks.print_cup(String::from("SELL"));
        println!("---------------------------------");
        self.bids.print_cup(String::from("BUY"));
    }

    // pub fn match_order(&mut self, order: Order) {
    //     //find if exist match
    //     let direction = order.get_direction();
    //     match direction {
    //         OrderDirection::BUY => self.bids.add_order(order),
    //         OrderDirection::SELL => self.asks.add_order(order),
    //     }
    //     //Match
    //     //execute
    // }
}
