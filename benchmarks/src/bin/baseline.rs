//! Generates the same data as the library binaries and writes nothing:
//! the memory/CPU floor every library starts from.
#[path = "../data.rs"]
mod data;
use data::*;

fn main() {
    let (scenario, n, _fmt, _out) = args();
    let sink = match scenario.as_str() {
        "line" => line(n).1.iter().sum::<f64>(),
        "scatter" => scatter(n).1.iter().sum::<f64>(),
        "hist" => hist(n).iter().sum::<f64>(),
        _ => 0.0,
    };
    timed(|| {
        std::hint::black_box(sink);
    });
}
