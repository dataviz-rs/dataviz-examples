//! Text in any language: characters the theme font lacks come from system
//! fonts, Arabic and Hebrew are laid out right to left and shaped, emoji are
//! drawn in color.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 600);
    fig.title("World sales · 全球销售 · المبيعات العالمية · 🌍");
    let ax = fig.axes();
    let countries = [
        "中国", "日本", "한국", "مصر", "ישראל", "भारत", "ไทย", "Ελλάδα", "Türkiye", "Україна",
    ];
    let sales = [42.0, 31.0, 27.0, 18.0, 12.0, 35.0, 15.0, 9.0, 22.0, 11.0];
    ax.bar(countries, sales)
        .colors(Palette::Tab10.colors())
        .value_labels(LabelPosition::Outside)
        .label("2025 📈");
    ax.x_label("国家 · البلد · देश · country")
        .y_label("Umsatz (Mio. €)");
    ax.text(-0.3, 54.0, "مرحبا بالعالم — שלום עולם — नमस्ते दुनिया — สวัสดีชาวโลก").size(15.0);
    ax.text(-0.3, 49.5, "Top market 🚀 · fastest growth ✅ · ❤️").size(15.0);
    ax.y_limits(0.0, 58.0);
    ax.legend(LegendPosition::UpperRight);

    fig.save("international.png")?;
    fig.save("international.svg")?;
    Ok(())
}
