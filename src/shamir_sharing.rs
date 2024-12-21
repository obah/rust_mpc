//shamir secret sharing technique
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct PointsTrace {
    x: Vec<i32>,
    y: Vec<i32>,
    mode: String,
    trace_type: String,
    name: String,
    text: Vec<String>,
}

#[derive(Debug)]
struct PolyTrace {
    x: Vec<i32>,
    y: Vec<i32>,
    mode: String,
    trace_type: String,
    name: String,
}

#[derive(Debug)]
enum Trace {
    Points(PointsTrace),
    Polynomial(PolyTrace),
}

const PRIME: u32 = 127;

fn calc_mod(mut v: i32) -> i32 {
    let prime = PRIME as i32;

    while v < 0 {
        v += prime;
    }
    v % prime
}

fn polynomial_print(data: &[i32]) -> String {
    let mut polynomial = String::from("");

    for (i, &coeff) in data.iter().enumerate() {
        polynomial.push_str(&coeff.to_string());

        if i > 0 {
            polynomial.push_str(&format!(" x^{}", i));
        }

        if i < data.len() - 1 {
            polynomial.push_str(" + ");
        }
    }

    polynomial
}

fn polynomial_eval(data: &[i32], x: i32) -> i32 {
    let mut power = 1;
    let mut y = 0;

    for i in 0..data.len() {
        y += data[i] * power;
        power *= x;
    }

    y
}

fn polynomial_multiply(data1: &[i32], data2: &[i32]) -> Vec<i32> {
    let mut polynomial = Vec::new();

    let len1 = data1.len() + data2.len() - 1;

    for _ in 0..len1 {
        polynomial.push(0);
    }

    for i in 0..data1.len() {
        for j in 0..data2.len() {
            let v = data1[i] + data2[j];
            let pow = i + j;
            polynomial[pow] += v;
        }
    }

    polynomial
}

fn plot_point(points: &[i32], coefficients: &[i32], index: usize) -> (Trace, Trace) {
    let mut points_trace = PointsTrace {
        x: vec![0],
        y: vec![coefficients[0]],
        mode: "markers".to_string(),
        trace_type: "scatter".to_string(),
        name: format!("Shares {}", index + 1),
        text: vec!["Value".to_string()],
    };

    let mut poly_trace = PolyTrace {
        x: Vec::new(),
        y: Vec::new(),
        mode: "lines".to_string(),
        trace_type: "scatter".to_string(),
        name: format!("Polynomial {}", index + 1),
    };

    // Populate `points_trace`
    for (i, &point) in points.iter().enumerate() {
        points_trace.x.push((i + 1) as i32);
        points_trace.y.push(point);
        points_trace.text.push(format!("Share {}", i + 1));
    }

    // Populate `poly_trace`
    let range = -6..6;
    let step = 0.1;

    for i in (range.start as i32..range.end as i32).step_by((step * 10.0) as usize) {
        poly_trace.x.push(i);
        let poly_value = polynomial_eval(coefficients, i);
        poly_trace.y.push(calc_mod(poly_value));
    }

    (Trace::Points(points_trace), Trace::Polynomial(poly_trace))
}

fn plot(points_arr: &[Vec<i32>], coefficients_arr: &[Vec<i32>], id: &str) {
    let mut traces = Vec::new();

    for (i, (points, coefficients)) in points_arr.iter().zip(coefficients_arr.iter()).enumerate() {
        let (poly_trace, points_trace) = plot_point(points, coefficients, i);
        traces.push(poly_trace);
        traces.push(points_trace);
    }

    // Simulate rendering (or replace with actual plotting library for Rust)
    println!("Plot ID: {}", id);
    println!("Traces: {:#?}", traces);
}

fn share(x: i32, n: usize) -> (Vec<i32>, Vec<i32>) {
    let mut rng = thread_rng();
    let prime = PRIME as i32;

    // Generate the polynomial coefficients
    let mut polynomial = vec![x];
    for _ in 0..n - 1 {
        let r = rng.gen_range(0..prime);
        polynomial.push(r);
    }

    // Generate the shares
    let shares = (1..=n)
        .map(|k| calc_mod(polynomial_eval(&polynomial, k.try_into().unwrap())))
        .collect::<Vec<_>>();

    (shares, polynomial)
}

pub fn shamir_sharing() {
    let result = share(5, 3);

    let shares = result.0;
    let polynomial = result.1;

    println!("Polynomial: {}", polynomial_print(&polynomial));
    plot(&[shares], &[polynomial], "plot2");
}
