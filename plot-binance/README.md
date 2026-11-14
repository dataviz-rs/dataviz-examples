# Live Binance market dashboard

Live BTC/USDT (or any spot symbol) from Binance's public market-data API,
redrawn about once a second (`Figure::show_live`, dark theme):

- 1-minute candlesticks with MA(7) and MA(25) and the last price,
- volume per minute (total and taker buys),
- every trade of the last two minutes (buys green, sells red, marker size by amount),
- order-book depth (cumulative bids and asks, 500 levels),
- taker buy minus sell volume per 10 seconds,
- 24 h change, high, low and volume in the title.

<img src="./screenshots/binance.png" alt="Live Binance dashboard" width="900">

A background thread polls `https://data-api.binance.vision/api/v3`
(`trades`, `depth`, `klines`, `ticker/24hr`) with the system's `curl`, so
there is no HTTP or JSON dependency and no API key; the window only redraws
when new data arrived. Times are shown in the local time zone.

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-binance
cargo run --release                 # BTCUSDT
cargo run --release -- ETHUSDT      # any spot symbol
cargo run --release -- BTCUSDT --png binance.png --seconds 10   # snapshot, no window
```

Needs `curl` and network access to `data-api.binance.vision`. Market data
only, for illustration; not financial advice.

## Window controls

Mouse wheel zooms, left-drag pans, right-drag zooms to a box; the candles
and the volume bars move together and share a crosshair (`C` toggles it)
that shows the time, price and every series' value under the cursor.
Clicking a legend row (e.g. MA(25)) hides or shows that series. A zoomed or
panned plot keeps its view while new data arrives; auto scale (on, `A`
toggles it) fits the y-axis to the visible data, so you move along time and
the prices stay in view. `R` follows the live data again, `S` saves a PNG,
`H` toggles hover tooltips, `Esc` closes. The window has a normal title bar
(X11/XWayland); `DATAVIZ_WAYLAND=1` uses Wayland directly.

## Recording (`--log DIR`)

`cargo run --release -- BTCUSDT --log binance-log` also appends the data to
CSV files (times in UTC, `● REC` in the title):

| File | Rows |
|---|---|
| `trades.csv` | every trade: time, price, qty, side (buy/sell) |
| `book.csv` | once a second: best bid, best ask, spread, bid and ask depth (500 levels) |
| `candles.csv` | every closed 1-minute candle (on the first run also the 89 before it) |

Files are written every second and appended to: running again continues the
log (candles are not repeated). They load back with
`dataviz::plot::data::Table::read`.

## License

MIT
