//! Shared, deterministic benchmark data.
#![allow(dead_code)]

pub const W: u32 = 800;
pub const H: u32 = 600;

pub struct Rng(pub u64);
impl Rng {
    pub fn uniform(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn normal(&mut self) -> f64 {
        (0..12).map(|_| self.uniform()).sum::<f64>() - 6.0
    }
}

/// x = 0..n, y = noisy sine.
pub fn line(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut r = Rng(1);
    let x: Vec<f64> = (0..n).map(|i| i as f64 / n as f64 * 100.0).collect();
    let y = x.iter().map(|v| (v * 0.3).sin() * 10.0 + r.uniform()).collect();
    (x, y)
}

/// Correlated normal points.
pub fn scatter(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut r = Rng(2);
    let x: Vec<f64> = (0..n).map(|_| r.normal()).collect();
    let y = x.iter().map(|v| v * 0.6 + r.normal() * 0.5).collect();
    (x, y)
}

/// `n` categories with values.
pub fn bars(n: usize) -> (Vec<String>, Vec<f64>) {
    ((0..n).map(|i| format!("c{i}")).collect(), (0..n).map(|i| 10.0 + ((i * 37) % 23) as f64).collect())
}

/// Normal samples.
pub fn hist(n: usize) -> Vec<f64> {
    let mut r = Rng(3);
    (0..n).map(|_| r.normal() * 2.0 + 5.0).collect()
}

pub const BINS: usize = 50;

/// (scenario, n, format, output path) from the command line.
pub fn args() -> (String, usize, String, String) {
    let a: Vec<String> = std::env::args().skip(1).collect();
    (a[0].clone(), a[1].parse().unwrap(), a[2].clone(), a[3].clone())
}

/// Process CPU time (user + system) in milliseconds.
pub fn cpu_ms() -> f64 {
    let mut ru = std::mem::MaybeUninit::<libc_rusage>::zeroed();
    // SAFETY: getrusage fills the struct; RUSAGE_SELF = 0.
    unsafe { getrusage(0, ru.as_mut_ptr()) };
    let ru = unsafe { ru.assume_init() };
    let t = |s: i64, us: i64| s as f64 * 1000.0 + us as f64 / 1000.0;
    t(ru.utime_s, ru.utime_us) + t(ru.stime_s, ru.stime_us)
}

#[repr(C)]
pub struct libc_rusage {
    utime_s: i64,
    utime_us: i64,
    stime_s: i64,
    stime_us: i64,
    rest: [i64; 14],
}

extern "C" {
    fn getrusage(who: i32, usage: *mut libc_rusage) -> i32;
}

/// Runs `f` `BENCH_REPS` times (default 1) and prints the first (cold)
/// time, the median warm time and the median warm CPU time in ms.
pub fn timed(mut f: impl FnMut()) {
    timed_with(|| (), |()| f());
}

/// Like [`timed`], with an untimed `setup` whose result `f` consumes
/// (owned data handed to the plot, as idiomatic for retained-mode APIs).
pub fn timed_with<T>(mut setup: impl FnMut() -> T, mut f: impl FnMut(T)) {
    let reps: usize = std::env::var("BENCH_REPS").ok().and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let (mut t, mut c) = (Vec::with_capacity(reps), Vec::with_capacity(reps));
    for _ in 0..reps {
        let input = setup();
        let (s, c0) = (std::time::Instant::now(), cpu_ms());
        f(input);
        t.push(s.elapsed().as_secs_f64() * 1000.0);
        c.push(cpu_ms() - c0);
    }
    let med = |v: &[f64]| {
        let mut w = v.to_vec();
        w.sort_by(f64::total_cmp);
        if w.is_empty() { 0.0 } else { w[w.len() / 2] }
    };
    let (first, warm, cpu) = (t[0], med(&t[1.min(reps - 1)..]), med(&c[1.min(reps - 1)..]));
    println!("{first:.3} {warm:.3} {cpu:.3}");
}
