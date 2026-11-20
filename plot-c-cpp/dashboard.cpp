// A dashboard from C++: direct calls for lines, scatter and histograms, a
// JSON spec with binary arrays for any other chart type, and the grammar
// of graphics. Writes dashboard.png and dashboard.svg.
#include "dataviz.hpp"

#include <chrono>
#include <cmath>
#include <cstdio>
#include <random>
#include <string>
#include <vector>

int main() {
    std::mt19937 rng(42);
    std::normal_distribution<double> normal(0.0, 1.0);

    // A noisy signal and its moving average.
    const int n = 2000;
    std::vector<double> t(n), signal(n), smooth(n);
    for (int i = 0; i < n; i++) {
        t[i] = i * 0.01;
        signal[i] = std::sin(t[i]) + 0.35 * std::sin(t[i] * 7.3) + 0.15 * normal(rng);
    }
    for (int i = 0; i < n; i++) {
        double s = 0;
        int k = 0;
        for (int j = std::max(0, i - 40); j <= std::min(n - 1, i + 40); j++, k++) s += signal[j];
        smooth[i] = s / k;
    }
    // 200 000 correlated points, colored by their product.
    const int m = 200000;
    std::vector<double> x(m), y(m), c(m);
    for (int i = 0; i < m; i++) {
        x[i] = normal(rng);
        y[i] = 0.6 * x[i] + 0.8 * normal(rng);
        c[i] = x[i] * y[i];
    }
    // Response times of three services.
    std::vector<double> a(5000), b(5000), d(5000);
    for (int i = 0; i < 5000; i++) {
        a[i] = std::exp(3.0 + 0.30 * normal(rng));
        b[i] = std::exp(3.4 + 0.25 * normal(rng));
        d[i] = std::exp(2.7 + 0.45 * normal(rng));
    }

    auto start = std::chrono::steady_clock::now();
    dataviz::Figure fig(1200, 800);
    fig.title("dataviz from C++");

    auto lines = fig.axes(0, 0);
    lines.line(t, signal, dataviz::Style().label("signal").width(1.0).color(0x1f77b480));
    lines.line(t, smooth, dataviz::Style().label("moving average").width(2.5));
    lines.title("Signal").xlabel("time (s)").legend("upper_right");

    auto points = fig.axes(0, 1);
    points.scatter(x, y, c, dataviz::Style().colormap("viridis").size(2));
    points.title("200 000 points");

    // Any chart type of the spec; the arrays stay binary ("@a", "@b", …).
    fig.axes(1, 0).plot(
        R"json({"type": "violin", "groups": {"auth": "@a", "search": "@b", "cache": "@c"},
            "title": "Response time (ms)"})json",
        {{"a", a}, {"b", b}, {"c", d}});

    // Grammar of graphics: mean response per service and hour.
    // Numbers go over as binary arrays, the service names as JSON text.
    const char *names[] = {"auth", "search", "cache"};
    std::vector<double> hour, ms;
    std::string service = "[";
    for (int h = 0; h < 24; h++)
        for (int s = 0; s < 3; s++)
            for (int k = 0; k < 20; k++) {
                hour.push_back(h);
                service += std::string(service.size() > 1 ? "," : "") + "\"" + names[s] + "\"";
                double base = (s == 0 ? 20 : s == 1 ? 30 : 15) * (1 + 0.6 * std::exp(-std::pow(h - 14, 2) / 18.0));
                ms.push_back(base * std::exp(0.2 * normal(rng)));
            }
    service += "]";
    fig.axes(1, 1).plot(
        R"json({"data": {"hour": "@h", "service": )json" + service + R"json(, "ms": "@m"}, "mark": "line",
            "encoding": {"x": "hour", "y": {"field": "ms", "aggregate": "mean"}, "color": "service"},
            "title": "Mean response by hour (grammar of graphics)"})json",
        {{"h", hour}, {"m", ms}});

    fig.save("dashboard.png");
    fig.save("dashboard.svg");
    double ms_total = std::chrono::duration<double, std::milli>(std::chrono::steady_clock::now() - start).count();
    std::printf("dataviz %s: dashboard built and saved (PNG + SVG) in %.0f ms\n",
                dataviz::version().c_str(), ms_total);
    return 0;
}
