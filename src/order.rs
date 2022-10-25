use crate::wallet::CurrencyName;

#[derive(Clone, Debug)]
pub struct OrderPair {
    foreign: CurrencyName,
    domestic: CurrencyName,
}

impl OrderPair {
    pub fn create_orderpair(foreign: CurrencyName, domestic: CurrencyName) -> Self {
        Self { foreign, domestic }
    }
}

#[derive(Clone, Debug)]
pub enum OrderDirection {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]

pub struct Order {
    client_id: u32,
    client_name: String,
    instrument_id: OrderPair,
    direction: OrderDirection,
    // time_in_force: datetime,
    // order_type: OrderType,
    quantity: u32,
    price: u32,
    // timestamp:
}

impl Order {
    pub fn create_order(
        client_id: u32,
        client_name: String,
        instrument_id: OrderPair,
        direction: OrderDirection,
        quantity: u32,
        price: u32,
    ) -> Self {
        Self {
            client_id,
            client_name,
            instrument_id,
            direction,
            quantity,
            price,
        }
    }

    pub fn get_price(&self) -> u32 {
        self.price
    }

    pub fn get_direction(&self) -> OrderDirection {
        self.direction.clone()
    }

    pub fn get_direction_str(&self) -> &str {
        match self.direction {
            OrderDirection::BUY => "BUY",
            OrderDirection::SELL => "SELL",
        }
    }

    pub fn get_client_name(&self) -> String {
        self.client_name.clone()
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        self.quantity = new_quantity;
    }

    pub fn execute_full(&mut self) {
        self.set_quantity(0);
        match self.direction {
            OrderDirection::BUY => (),
            OrderDirection::SELL => (),
        }
    }
    pub fn execute_part(&mut self, executy_quantity: u32) {
        self.set_quantity(executy_quantity)
    }
}
