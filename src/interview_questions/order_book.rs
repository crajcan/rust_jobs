use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(Clone)]
enum Side {
    Buy,
    Sell
}

#[derive(Clone)]
struct Order {
  id: usize,
  price: i32,
  size: usize,
  side: Side,
}

#[derive(PartialEq, Debug, Clone)]
struct PriceLevel {
    size: usize,
    price: i32,
    order_ids: VecDeque<usize>,
}

struct OrderBook {
    buy_orders: HashMap<usize, Order>,
    buy_price_levels: BTreeMap<i32, PriceLevel>,
    sell_orders: HashMap<usize, Order>,
    sell_price_levels: BTreeMap<i32, PriceLevel>,
}

struct Trade {
    bought: Vec<Order>,
    sold: Vec<Order>,
    remainder: Option<Order>
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            buy_price_levels: BTreeMap::new(),
            sell_orders: HashMap::new(),
            sell_price_levels: BTreeMap::new(),
        }
    }

    fn trade(&mut self, other_order: usize, order: Order) -> (Order, Order) {
         
    }


    fn try_trade(&mut self, mut order: Order) -> Trade {
        let mut bought = vec![];
        let mut sold = vec![];

        let order = match order.side {
            Side::Buy => {
                
                loop {
                    let best_ask = self.best_ask();
                    if best_ask.is_none() {
                        break;
                    }

                    if best_ask.unwrap().price <= order.price {
                        break;
                    }

                    match self.sell_price_levels.get_mut(&order.price) {
                        Some(price_level) => {
                            let to_trade = price_level.order_ids.pop_front();

                            match to_trade {
                                None => break,
                                Some(other_order_id) => {
                                    (order, other_trade) = self.trade(other_order_id, order);
                                } 
                            }
                        },
                        None => break,
                    }
                    
                        
                }

                order
            },
            Side::Sell => {

            },
        };

        Trade {
            bought,
            sold,
            remainder: order,
        }
    }

    fn add_order(&mut self, order: Order) -> Trade { 
        let trade = self.try_trade(order);

        match trade.remainder {
            Some(order) => self.insert_order_to_book(order.clone()),
            None => (),
        }

        trade 
    }

    fn insert_order_to_book(&mut self, order: Order) {
        let order_size = order.size;
        let order_price = order.price;
        let order_id = order.id;

        match order.side {
            Side::Buy => {
                self.buy_orders.insert(order.id, order);

                self.buy_price_levels.entry(order_price)
                    .and_modify(|price_level| {
                        price_level.size += order_size;
                        price_level.order_ids.push_back(order_id);
                    })
                    .or_insert({
                        let mut order_ids = VecDeque::new();
                        order_ids.push_back(order_id);

                        PriceLevel { size: order_size, price: order_price, order_ids }
                    });
            },
            Side::Sell => {
                self.sell_orders.insert(order.id, order);
                self.sell_price_levels.entry(order_price)
                    .and_modify(|price_level| {
                        price_level.size += order_size;
                        price_level.order_ids.push_back(order_id);
                    })
                    .or_insert({
                        let mut order_ids =  VecDeque::new();
                        order_ids.push_back(order_id);

                        PriceLevel { size: order_size, price: order_price, order_ids }
                    });
            },
        }
    }

    fn delete_order(&mut self, order_id: usize) {
        let removed = self.buy_orders.remove(&order_id);

        if let Some(removed) = removed {
            self.buy_price_levels.entry(removed.price).and_modify(|price_level| price_level.size -= removed.size);

            if self.buy_price_levels.get(&removed.price).unwrap().size == 0 {
                self.buy_price_levels.remove(&removed.price);
            }

            return;
        } 

        let removed = self.sell_orders.remove(&order_id);
        if let Some(removed) = removed {
            self.sell_price_levels.entry(removed.price).and_modify(|price_level| price_level.size -= removed.size);

            if self.sell_price_levels.get(&removed.price).unwrap().size == 0 {
                self.sell_price_levels.remove(&removed.price);
            }
        } else {
            print!("id not found");
        }
    }

    fn best_bid(&self) -> Option<PriceLevel> {
        match self.buy_price_levels.last_key_value() {
            None => None,
            Some((_k,v)) => Some(v.clone())
        }
    }

    fn best_ask(&self) -> Option<PriceLevel> {
        match self.sell_price_levels.first_key_value() {
            None => None,
            Some((_k,v)) => Some(v.clone())
        }
    }
}

fn orders() -> Vec<Order> {
    vec![
        Order { id: 1, price: 99, size: 1, side: Side::Buy },
        Order { id: 2, price: 99, size: 3, side: Side::Buy },
        Order { id: 3, price: 98, size: 4, side: Side::Buy },
        Order { id: 4, price: 100, size: 3, side: Side::Sell },
        Order { id: 5, price: 101, size: 2, side: Side::Sell },
        Order { id: 6, price: 101, size: 5, side: Side::Sell },
    ]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_order_book() {

        let mut book = OrderBook::new();

        for order in orders() {
            book.add_order(order);
        }

        // book.delete_order(4);

        assert_eq!(book.best_bid().unwrap().price, 99);
        assert_eq!(book.best_bid().unwrap().size, 4);
        assert_eq!(book.best_ask().unwrap().price, 100);
        assert_eq!(book.best_ask().unwrap().size, 3);
        println!("tests passed");
    }
}