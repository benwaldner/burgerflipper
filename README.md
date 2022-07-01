# Burgerfilpper🍔

## Summary
This repo is an full Rust impelement of [J Fernandez Tapia · 2015.Modeling, optimization and estimation for the on-line control of trading algorithms in limit-order markets](https://www.theses.fr/2015PA066354.pdf) Avellaneda-Stoikov market making model extends version

only work for Binance USDT/BUSD PERP and OKX USDT PERP

api client is the [exrs](https://github.com/zhenpingfeng/exrs) 

## How to use
for binance
```rust
RUST_LOG=info cargo run --release --bin burgerfilpper config.json
```

for okx
```
RUST_LOG=info cargo run --release --bin as_okex okex_config.json
```

## Parameters explain
```json
{
    "api_key": "api_key", // ur key
    "secret_key": "secret_key", // ur dirty secret
    "base_asset": "1000SHIB",
    "quote_asset": "USDT",
    "order_qty": 1000000, // how many base_asset each time to bid
    "tick_size": 0.000001, // increment of spread, greater than or equal to tick size
    "n_spreads": 10, // number of spread increments, greater than 2
    "estimate_window": 600000, // estimate window for ak solver (milliseconds).
    "period": 2000, // bid and cancel order period (milliseconds). u can set it to 100 to reach binance's max rate limit. At activate market condition should around 2000-5000 milliseconds, boring market condition u can change it to 10000 milliseconds. when the both side can get fill, that would be a suitable parameter.
    "sigma_tick_period": 550, // how many tick u want to use for calc the volatility parameter. u can change it to use a different volatility indicator from the code.
    "gamma": 0.2, // gamma form the paper
    "sigma_multiplier": 1, // hack, when the sigma esitmate did't doing will, u use this hack the paramter new_sigma = sigma * sigma_multiplier
    "ema_tick_period": 200, // indicator param
    "stoploss": 0.00618, // market stoploss when current holding position loss > 0.618%, u can turn it off by set to 1.
    "stoploss_sleep": 300000, // after stop loss, how many milliseconds u want to stop trading.
    "stopprofit": 0.00618, // take profit when current holding profit > 0.6%, I personally like 2.6%
    "trailing_stop": 0.00382, // when profit fall back to 0.3%, send market order to take profit.
}
```

## Can it make money?
yes, sometimes it can make money smoothly, but when the informed trader come and have the information advantage, this bot finally will become a Burgerfilpper at McDonald's.
![alt text](./E42sI2lWUAAOHHo.jpeg)

## Sensitive to Rust unsafe keyword?
full safe rusty code, relax bruh.
