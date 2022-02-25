use surf::Url;

const PARSE_FAILURE_MESSAGE: &str = "failed to parse hardcoded url";

#[cfg(not(test))]
macro_rules! base {
    () => {
        "https://api.btcturk.com/"
    };
}

#[cfg(test)]
macro_rules! base {
    () => {
        "https://api-dev.btcturk.com/"
    };
}

macro_rules! endpoint {
    ($endpoint:literal) => {
        Url::parse(concat!(base!(), $endpoint)).expect(PARSE_FAILURE_MESSAGE)
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UrlCache {
    ticker: Url,
    currency: Url,
    order_book: Url,
    trades: Url,
    ohlc: Url,
    account_balance: Url,
    trade_transactions: Url,
    crypto_transactions: Url,
    fiat_transactions: Url,
    open_orders: Url,
    all_orders: Url,
    submit_cancel_order: Url,
    exchange_info: Url,
}

impl Default for UrlCache {
    fn default() -> Self {
        Self {
            ticker: endpoint!("api/v2/ticker"),
            currency: endpoint!("api/v2/ticker/currency"),
            order_book: endpoint!("api/v2/orderbook"),
            trades: endpoint!("api/v2/trades"),
            ohlc: Url::parse("https://graph-api.btcturk.com/v1/ohlcs")
                .expect(PARSE_FAILURE_MESSAGE),
            account_balance: endpoint!("api/v1/users/balances"),
            trade_transactions: endpoint!("api/v1/users/transactions/trade"),
            crypto_transactions: endpoint!("api/v1/users/transactions/crypto"),
            fiat_transactions: endpoint!("api/v1/users/transactions/fiat"),
            open_orders: endpoint!("api/v1/openOrders"),
            all_orders: endpoint!("api/v1/allOrders"),
            submit_cancel_order: endpoint!("api/v1/order"),
            exchange_info: endpoint!("api/v2/server/exchangeinfo"),
        }
    }
}

impl UrlCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn ticker(&self) -> &Url {
        &self.ticker
    }

    pub const fn currency(&self) -> &Url {
        &self.currency
    }

    pub const fn order_book(&self) -> &Url {
        &self.order_book
    }

    pub const fn trades(&self) -> &Url {
        &self.trades
    }

    pub const fn ohlc(&self) -> &Url {
        &self.ohlc
    }

    pub const fn account_balance(&self) -> &Url {
        &self.account_balance
    }

    pub const fn trade_transactions(&self) -> &Url {
        &self.trade_transactions
    }

    pub const fn crypto_transactions(&self) -> &Url {
        &self.crypto_transactions
    }

    pub const fn fiat_transactions(&self) -> &Url {
        &self.fiat_transactions
    }

    pub const fn open_orders(&self) -> &Url {
        &self.open_orders
    }

    pub const fn all_orders(&self) -> &Url {
        &self.all_orders
    }

    pub const fn submit_cancel_order(&self) -> &Url {
        &self.submit_cancel_order
    }

    pub const fn exchange_info(&self) -> &Url {
        &self.exchange_info
    }
}
