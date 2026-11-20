/* Live data in a window from C: a sensor value arrives every frame and the
 * last 600 points are shown. Zoom with the wheel, pan by dragging, double
 * click to reset; Escape closes the window. */
#include "dataviz.h"

#include <math.h>
#include <stdio.h>

typedef struct {
    dv_series raw, filtered;
    double t, f;
} state;

static int update(dv_figure *fig, void *user) {
    state *s = (state *)user;
    (void)fig;
    for (int i = 0; i < 3; i++) {
        s->t += 0.02;
        double v = sin(s->t) + 0.4 * sin(s->t * 5.7) + 0.2 * sin(s->t * 31.0);
        s->f += (v - s->f) * 0.08; /* low-pass filter */
        dv_series_push(s->raw, s->t, v, 600);
        dv_series_push(s->filtered, s->t, s->f, 600);
    }
    return 1; /* redraw */
}

int main(void) {
    dv_figure *fig = dv_figure_new(900, 480);
    dv_axes ax = dv_figure_axes(fig, 0, 0);
    dv_axes_title(ax, "Sensor (live)");
    dv_style raw = {0}, filtered = {0};
    raw.label = "raw";
    raw.width = 1.0;
    filtered.label = "filtered";
    filtered.width = 2.5;
    state s = {dv_line(ax, NULL, NULL, 0, &raw), dv_line(ax, NULL, NULL, 0, &filtered), 0, 0};
    dv_axes_legend(ax, "upper_left");
    if (!dv_figure_show_live(fig, "dataviz live (C)", 60, update, &s))
        fprintf(stderr, "no window: %s\n", dv_last_error());
    dv_figure_free(fig);
    return 0;
}
