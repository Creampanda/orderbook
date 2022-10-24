use crate::order::{Order, OrderDirection, OrderPair};

#[derive(Debug)]
pub enum CurrencyBalance {
    XTZ { available: u32, blocked: u32 },
    USDT { available: u32, blocked: u32 },
}

impl CurrencyBalance {
    pub fn get_available(&self) -> &u32 {
        match self {
            Self::XTZ { available, blocked } => available,
            Self::USDT { available, blocked } => available,
        }
    }

    pub fn get_blocked(&self) -> &u32 {
        match self {
            Self::XTZ { available, blocked } => blocked,
            Self::USDT { available, blocked } => blocked,
        }
    }

    pub fn block_some(&self, amount: u32) -> Self {
        match self {
            Self::XTZ { available, blocked } => Self::XTZ {
                available: available - amount,
                blocked: blocked + amount,
            },
            Self::USDT { available, blocked } => Self::USDT {
                available: available - amount,
                blocked: blocked + amount,
            },
        }
    }
    pub fn unblock_some(&self, amount: u32) -> Self {
        match self {
            Self::XTZ { available, blocked } => Self::XTZ {
                available: available + amount,
                blocked: blocked - amount,
            },
            Self::USDT { available, blocked } => Self::USDT {
                available: available + amount,
                blocked: blocked - amount,
            },
        }
    }
    pub fn add_some(&self, amount: u32) -> Self {
        match self {
            Self::XTZ { available, blocked } => Self::XTZ {
                available: available + amount,
                blocked: *blocked,
            },
            Self::USDT { available, blocked } => Self::USDT {
                available: available + amount,
                blocked: *blocked,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum CurrencyName {
    XTZ,
    USDT,
}

#[derive(Debug)]
pub struct Wallet {
    id: u32,
    name: String,
    pubkey: String,
    pub xtz: CurrencyBalance,
    pub usdt: CurrencyBalance,
}

impl Wallet {
    pub fn create_wallet(id: u32, name: String, pubkey: String) -> Self {
        Self {
            id,
            name,
            pubkey,
            xtz: CurrencyBalance::XTZ {
                available: (0),
                blocked: (0),
            },
            usdt: CurrencyBalance::USDT {
                available: (0),
                blocked: (0),
            },
        }
    }

    pub fn deposite(&mut self, currency_name: CurrencyName, amount: u32) {
        match currency_name {
            CurrencyName::USDT => {
                let new = self.usdt.add_some(amount);
                self.usdt = new;
            }
            CurrencyName::XTZ => {
                let new = self.xtz.add_some(amount);
                self.xtz = new;
            }
        }
    }

    pub fn block(&mut self, currency_name: CurrencyName, amount: u32) {
        match currency_name {
            CurrencyName::USDT => {
                if amount <= *self.usdt.get_available() {
                    let new = self.usdt.block_some(amount);
                    self.usdt = new;
                } else {
                    println!("Insufficient funds");
                };
            }
            CurrencyName::XTZ => {
                if amount <= *self.xtz.get_available() {
                    let new = self.xtz.block_some(amount);
                    self.xtz = new;
                } else {
                    println!("Insufficient funds")
                }
            }
        }
    }

    pub fn unblock_xtz(&mut self, currency_name: CurrencyName, amount: u32) {
        match currency_name {
            CurrencyName::USDT => {
                let new = self.usdt.unblock_some(amount);
                self.usdt = new;
            }
            CurrencyName::XTZ => {
                let new = self.xtz.unblock_some(amount);
                self.xtz = new;
            }
        }
    }

    pub fn create_order(
        &self,
        direction: OrderDirection,
        // foreign_currency: CurrencyName,
        // domestic_currency: CurrencyName,
        quantity: u32,
        price: u32,
    ) -> Option<Order> {
        match direction {
            OrderDirection::BUY => {
                let available_usdt = self.usdt.get_available();
                if quantity * price > *available_usdt {
                    println!("Insufficient funds");
                    return None;
                } else {
                    self.usdt.block_some(quantity * price);
                    Some(Order::create_order(
                        self.name.clone(),
                        OrderPair::create_orderpair(CurrencyName::XTZ, CurrencyName::USDT),
                        direction,
                        quantity,
                        price,
                    ))
                }
            }
            OrderDirection::SELL => {
                let available_xtz = self.xtz.get_available();
                if quantity * price > *available_xtz {
                    println!("Insufficient funds");
                    return None;
                } else {
                    self.xtz.block_some(quantity * price);
                    Some(Order::create_order(
                        self.name.clone(),
                        OrderPair::create_orderpair(CurrencyName::XTZ, CurrencyName::USDT),
                        direction,
                        quantity,
                        price,
                    ))
                }
            }
        }
    }
}
