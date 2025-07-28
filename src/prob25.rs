pub fn prob25() -> f64 {
    let phi = 1.6180339887 as f64;
    let log_phi = phi.log10();
    let half_log_5 = 0.5 * (5 as f64).log10();

   ((1000 as f64 - 1.0 + half_log_5) / log_phi).ceil()
}