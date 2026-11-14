//! A dataviz chart inside an egui application: the figure is rendered into
//! a texture every frame (`Figure::render_into`) and the mouse is passed on
//! with the window-free interaction API (`zoom_at`, `pan_by`, `hover`,
//! `click`, `reset_view`). The side panel changes the chart live.

use dataviz::plot::series::line::Line;
use dataviz::prelude::*;
use eframe::egui;

struct App {
    fig: Figure,
    px: Pixels,
    texture: Option<egui::TextureHandle>,
    t: f64,
    speed: f64,
    noise: f64,
    paused: bool,
    dark: bool,
}

/// The chart at a size, with the data of `from` (if any).
fn chart(w: u32, h: u32, dark: bool, from: Option<&mut Figure>) -> Figure {
    let mut fig = Figure::new(w, h);
    fig.theme(if dark { Theme::dark() } else { Theme::default() });
    let mut data = vec![(Vec::new(), Vec::new()); 2];
    if let Some(old) = from {
        for (k, d) in data.iter_mut().enumerate() {
            if let Some(l) = old.axes().series_mut::<Line>(k) {
                let (x, y) = l.data();
                *d = (x.to_vec(), y.to_vec());
            }
        }
    }
    let ax = fig.axes();
    ax.title("Signal (wheel: zoom, drag: pan, double-click: reset)");
    let [(x0, y0), (x1, y1)] = <[_; 2]>::try_from(data).expect("two series");
    ax.line(x0, y0).label("signal");
    ax.line(x1, y1).label("moving average");
    ax.x_label("time [s]");
    ax.legend(LegendPosition::UpperLeft);
    fig
}

impl App {
    fn new() -> Self {
        Self {
            fig: chart(800, 500, false, None),
            px: Pixels::default(),
            texture: None,
            t: 0.0,
            speed: 1.0,
            noise: 0.3,
            paused: false,
            dark: false,
        }
    }

    /// New samples for the elapsed time.
    fn step(&mut self, dt: f64) {
        let n = (dt * 200.0 * self.speed).round().max(1.0) as usize;
        for _ in 0..n {
            self.t += 0.005;
            let t = self.t;
            let mut r = (t * 1e4).sin() * 43758.5453;
            r -= r.floor();
            let v = (t * 2.0).sin() + 0.4 * (t * 0.37).sin() + (r - 0.5) * self.noise;
            let ax = self.fig.axes();
            let line = ax.series_mut::<Line>(0).expect("signal");
            line.push(t, v);
            line.keep_last(4000);
            let (x, y) = line.data();
            let k = y.len().min(50);
            let avg = y[y.len() - k..].iter().sum::<f64>() / k as f64;
            let last_x = x[x.len() - 1];
            let ma = ax.series_mut::<Line>(1).expect("average");
            ma.push(last_x, avg);
            ma.keep_last(4000);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let dt = ctx.input(|i| i.stable_dt as f64).min(0.1);
        if !self.paused {
            self.step(dt);
        }
        egui::Panel::left("controls").show(ui, |ui| {
            ui.heading("dataviz in egui");
            ui.add(egui::Slider::new(&mut self.speed, 0.1..=5.0).text("speed"));
            ui.add(egui::Slider::new(&mut self.noise, 0.0..=1.5).text("noise"));
            ui.checkbox(&mut self.paused, "paused");
            if ui.checkbox(&mut self.dark, "dark theme").changed() {
                let (w, h) = self.fig.size();
                self.fig = chart(w as u32, h as u32, self.dark, Some(&mut self.fig));
            }
            if ui.button("reset view").clicked() {
                self.fig.reset_view();
            }
            ui.separator();
            ui.label(format!("{:.0} samples", self.t / 0.005));
        });
        egui::CentralPanel::default().show(ui, |ui| {
            // Render at the panel's size in physical pixels (sharp on HiDPI).
            let ppp = ctx.pixels_per_point();
            let size = ui.available_size();
            let (w, h) = ((size.x * ppp) as u32, (size.y * ppp) as u32);
            if w < 50 || h < 50 {
                return;
            }
            if self.fig.size() != (w as f64, h as f64) {
                // A new figure at the panel's size, keeping the data.
                self.fig = chart(w, h, self.dark, Some(&mut self.fig));
            }
            if self.fig.render_into(&mut self.px).is_err() {
                return;
            }
            let image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &self.px.rgba);
            let texture = match &mut self.texture {
                Some(t) => {
                    t.set(image, egui::TextureOptions::NEAREST);
                    t.clone()
                }
                None => {
                    let t = ctx.load_texture("chart", image, egui::TextureOptions::NEAREST);
                    self.texture = Some(t.clone());
                    t
                }
            };
            let resp = ui.add(egui::Image::new(&texture).fit_to_exact_size(size).sense(egui::Sense::click_and_drag()));
            // Mouse position in figure pixels.
            let at = |p: egui::Pos2| (((p.x - resp.rect.min.x) * ppp) as f64, ((p.y - resp.rect.min.y) * ppp) as f64);
            if let Some(p) = resp.hover_pos() {
                let (x, y) = at(p);
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll != 0.0 {
                    self.fig.zoom_at(x, y, (-scroll as f64 * 0.003).exp());
                }
                if resp.dragged() {
                    let d = resp.drag_delta();
                    self.fig.pan_by(x, y, d.x as f64 * ppp as f64, d.y as f64 * ppp as f64);
                }
                if let Some(h) = self.fig.hover(x, y) {
                    resp.clone().on_hover_text_at_pointer(h.text);
                }
            }
            if resp.double_clicked() {
                self.fig.reset_view();
            } else if resp.clicked() {
                if let Some(p) = resp.interact_pointer_pos() {
                    let (x, y) = at(p);
                    self.fig.click(x, y);
                }
            }
        });
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result {
    // `--screenshot out.png`: render the chart once without a window (CI).
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--screenshot") {
        let mut app = App::new();
        app.step(10.0);
        app.fig = chart(900, 520, false, Some(&mut app.fig));
        let path = args.get(i + 1).cloned().unwrap_or_else(|| "egui.png".into());
        app.fig.save(&path).expect("save");
        return Ok(());
    }
    eframe::run_native(
        "dataviz in egui",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 650.0]),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
