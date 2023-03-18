use std::f64::consts::E;
use image::{RgbImage, Rgb, imageops};

fn main() {
    // let f = |x| x * x - 3.0;
    // let d = |x| 2.0 * x;
    // let f = |x| (x - 1.0) * (x + 2.0) * (x + 0.4) * (x - 0.2) * (x + 0.8);
    // let d = |x| (5.0*x*x*x*x) + (8.0*x*x*x) - ((69.0*x*x)/25.0) - ((496.0*x)/125.0) - 0.224;

    // let zero = newton(f, d, -1.0, 10);
    // println!("{zero}");

    let mut input = String::new();

    // println!("Enter a number to take the square root of:");
    // std::io::stdin().read_line(&mut input).unwrap();
    // let a: f64 = input.trim().parse().expect("Failed to parse input");
    // input.clear();

    println!("Enter a base for exponentiation:");
    std::io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().expect("Failed to parse input");
    input.clear();

    println!("Enter an exponent for exponentiation: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let e: f64 = input.trim().parse().expect("Failed to parse input");
    input.clear();

    // println!("std sqrt: {}, custom sqrt: {}", a.sqrt(), sqrt(a));
    println!("std pow: {}, custom pow: {}", b.powf(e), pow(b, e));

    let mut img = RgbImage::new(2000, 2000);
    draw_axes(&mut img);

    let f1 = |x| pow(2.5, x);
    let f2 = |x| sqrt(x);
    let f3 = |x| (x - 1.0) * (x + 2.0) * (x + 0.4) * (x - 0.2) * (x + 0.8);
    let f4 = |x| pow(x, x);
    let f5 = |x| 1.0 / x;
    let f6 = |x| ln(x);

    let scale = 5.0 / 2000.0;

    graph(f1, &mut img, scale, Rgb([255, 0, 0]));
    graph(f2, &mut img, scale, Rgb([0, 255, 0]));
    graph(f3, &mut img, scale, Rgb([0, 0, 255]));
    graph(f4, &mut img, scale, Rgb([255, 0, 255]));
    graph(f5, &mut img, scale, Rgb([0, 255, 255]));
    graph(f6, &mut img, scale, Rgb([255, 255, 0]));

    imageops::flip_vertical_in_place(&mut img);
    img.save("out.png").expect("Failed ot save image");
}

fn draw_axes(buf: &mut RgbImage) {
    for i in 0..buf.width() {
        buf.put_pixel(i, buf.height() / 2, Rgb([255; 3]));
    }

    for i in 0..buf.height() {
        buf.put_pixel(buf.width() / 2, i, Rgb([255; 3]));
    }
}

fn graph<F: Fn(f64) -> f64>(f: F, buf: &mut RgbImage, scale: f64, color: Rgb<u8>) {
    for i in 0..buf.width() {
        let x1 = (i as f64 - buf.width() as f64 / 2.0) * scale;
        let y1 = f(x1) / scale;

        let x2 = ((i + 1) as f64 - buf.width() as f64 / 2.0) * scale;
        let y2 = f(x2) / scale;

        if y1.is_nan() || y2.is_nan() {
            continue;
        }

        let y1 = y1 + buf.height() as f64 / 2.0;
        let y2 = y2 + buf.height() as f64 / 2.0;

        if y1 >= buf.height() as f64 || y1 < 0.0 || y2 >= buf.height() as f64 || y2 < 0.0 {
            continue;
        }

        draw_line(buf, (x1, y1), (x2, y2), color);
    }
}

fn draw_line(buf: &mut RgbImage, mut p1: (f64, f64), mut p2: (f64, f64), color: Rgb<u8>) {
    if p1.0 > p2.0 {
        std::mem::swap(&mut p1, &mut p2);
    }

    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let mut current_x = x1;
    let mut current_y = y1;

    let slope = (y2 - y1) / (x2 - x1);

    while current_x <= x2 {
        buf.put_pixel(current_x as u32, current_y as u32, color);
        current_x += 1.0;
        buf.put_pixel(current_x as u32, current_y as u32, color);
        current_y += slope;
    }
}

fn sqrt(a: f64) -> f64 {
    if a < 0.0 {
        return std::f64::NAN;
    }

    let a_bits = a.to_bits();
    // U = 0.043 (best value to offset the approximation log2(x + 1) = x for lowest error on average)
    // 0x1ff7a7ef9db22d0e + (a_bits >> 1) = 2^23((BITS_OF_A / 2^23 + U - 127) / 2 - U + 127)
    let approx_bits = 0x1ff7a7ef9db22d0e + (a_bits >> 1);
    let approx = f64::from_bits(approx_bits);

    let f = |x| x * x - a;
    let d = |x| 2.0 * x;

    newton(f, d, approx, 5)
}

fn newton<F, D>(f: F, d: D, initial_guess: f64, iters: u32) -> f64
    where F: Fn(f64) -> f64,
        D: Fn(f64) -> f64
{
    let mut zero = initial_guess;

    for _ in 0..iters {
        zero = zero - f(zero) / d(zero);
    }

    zero
}

fn pow(b: f64, e: f64) -> f64 {
    exp(e * ln(b))
}

fn ln(a: f64) -> f64 {
    // ln(2)
    const LN2: f64 = 0.69314718055994530941723;
    // ln(1 + x) = ln(2)x-ln(2) approximation correction value
    const U: f64 = 0.03;
    // maximum mantissa value (2^52)
    const MANT: f64 = 4503599627370496.0;

    let a_bits = a.to_bits() as f64;
    // this is just an approximate transformation of the expression ln((1 + M/2^52) * 2^(E-1023))
    // in this expression the stuff inside the log is just the representation of an f64
    // M and E are the mantissa and exponent respectively
    let approx = (a_bits * LN2) / MANT + U - 1023.0 * LN2;

    let f = |x| exp(x) - a;
    let d = |x| exp(x);

    newton(f, d, approx, 5)
}

// e^x
fn exp(x: f64) -> f64 {
    let i = x.trunc() as i64;
    let f = x.fract();

    powi(E, i) * exp_limited(f)
}

// e^x
// only guaranteed to be accurate from -1 to 1
fn exp_limited(x: f64) -> f64 {
    1.0 + x
        + powi(x, 2) / f(2)
        + powi(x, 3) / f(3)
        + powi(x, 4) / f(4)
        + powi(x, 5) / f(5)
        + powi(x, 6) / f(6)
        + powi(x, 7) / f(7)
        + powi(x, 8) / f(8)
        + powi(x, 9) / f(9)
        + powi(x, 10) / f(10)
        + powi(x, 11) / f(11)
        + powi(x, 12) / f(12)
        + powi(x, 13) / f(13)
        + powi(x, 14) / f(14)
        + powi(x, 15) / f(15)
        + powi(x, 16) / f(16)
        + powi(x, 17) / f(17)
        + powi(x, 18) / f(18)
        + powi(x, 19) / f(19)
        + powi(x, 20) / f(20)
}

// exponentiation but limited to an integer exponent
fn powi(b: f64, e: i64) -> f64 {
    let mut result = 1.0;

    for _ in 0..e.abs() {
        result *= b;
    }

    if e < 0 {
        1.0 / result
    } else {
        result
    }
}

// factorial
fn f(i: u64) -> f64 {
    let mut result = 1.0;

    for i in 2..=i {
        result *= i as f64;
    }

    result
}