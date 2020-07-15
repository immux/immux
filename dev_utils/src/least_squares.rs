fn average(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    return sum / (data.len() as f64);
}

// for y = kx+b
// data: [(x1, y1), (x2, y2)...] -> (k, b)
pub fn solve(data: &[(f64, f64)]) -> (f64, f64) {
    let xs: Vec<f64> = data.iter().map(|pair| pair.0).collect();
    let ys: Vec<f64> = data.iter().map(|pair| pair.1).collect();
    let x_average = average(&xs);
    let y_average = average(&ys);

    let slope: f64 = {
        let numerator: f64 = {
            let mut sum: f64 = 0.0;
            for i in 0..data.len() {
                sum += (xs[i] - x_average) * (ys[i] - y_average)
            }
            sum
        };
        let denominator: f64 = {
            let mut sum: f64 = 0.0;
            for i in 0..data.len() {
                sum += (xs[i] - x_average).powi(2)
            }
            sum
        };
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    };

    let intercept = y_average - slope * x_average;
    return (slope, intercept);
}
