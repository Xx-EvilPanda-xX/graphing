use std::f64::consts::{E, LN_2};
use image::{RgbImage, Rgb, imageops};

fn main() {
    // let f = |x| x * x - 3.0;
    // let d = |x| 2.0 * x;
    // let f = |x| (x - 1.0) * (x + 2.0) * (x + 0.4) * (x - 0.2) * (x + 0.8);
    // let d = |x| (5.0*x*x*x*x) + (8.0*x*x*x) - ((69.0*x*x)/25.0) - ((496.0*x)/125.0) - 0.224;

    // let zero = newton(f, d, -1.0, 10);
    // println!("{zero}");

    let mut input = String::new();

    println!("Enter a base for exponentiation:");
    std::io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().expect("Failed to parse input");
    input.clear();

    println!("Enter an exponent for exponentiation: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let e: f64 = input.trim().parse().expect("Failed to parse input");
    input.clear();

    println!("std pow: {}, custom pow: {}", b.powf(e), pow(b, e));

    const SIZE: u32 = 5000;
    let mut img = RgbImage::new(SIZE, SIZE);
    draw_axes(&mut img);

    let f1 = |x| pow(2.22222, x);
    let f2 = |x| sqrt(x);
    let f3 = |x| (x - 1.0) * (x + 2.0) * (x + 0.4) * (x - 0.2) * (x + 0.8);
    let f4 = |x| pow(x, x);
    let f5 = |x| 1.0 / x;
    let f6 = |x| ln(x);
    let f7 = |x| LN_2 * x - LN_2;

    let scale = 10.0 / SIZE as f64;

    println!("Graphing f1!");
    graph(f1, &mut img, scale, Rgb([255, 0, 0]));
    println!("Graphing f2!");
    graph(f2, &mut img, scale, Rgb([0, 255, 0]));
    println!("Graphing f3!");
    graph(f3, &mut img, scale, Rgb([0, 0, 255]));
    println!("Graphing f4!");
    graph(f4, &mut img, scale, Rgb([255, 0, 255]));
    println!("Graphing f5!");
    graph(f5, &mut img, scale, Rgb([0, 255, 255]));
    println!("Graphing f6!");
    graph(f6, &mut img, scale, Rgb([255, 255, 0]));
    println!("Graphing f7!");
    graph(f7, &mut img, scale, Rgb([128, 128, 128]));

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
        // divide by scale to bring output back to screen coords
        let y1 = f(x1) / scale;

        let x2 = ((i + 1) as f64 - buf.width() as f64 / 2.0) * scale;
        let y2 = f(x2) / scale;

        if y1.is_nan() || y2.is_nan() {
            continue;
        }

        let y1 = y1 + buf.height() as f64 / 2.0;
        let y2 = y2 + buf.height() as f64 / 2.0;

        draw_line(buf, (i as f64, y1), ((i + 1) as f64, y2), color);
    }
}

fn draw_line(buf: &mut RgbImage, mut p1: (f64, f64), mut p2: (f64, f64), color: Rgb<u8>) {
    let w = buf.width() as i32;
    let h = buf.height() as i32;

    // ensure our starting point is not outside the image (if the ending point isn't)
    if p1.0 >= w as f64 || p1.0 < 0.0 || p1.1 >= h as f64 || p1.1 < 0.0 {
        std::mem::swap(&mut p1, &mut p2);
    }

    // direction vector (p1 -> p2)
    let mut dx = p2.0 - p1.0;
    let mut dy = p2.1 - p1.1;

    const STEP_VEC_LEN: f64 = 0.1;

    // normalize vector to length STEP_VEC_LEN
    let len = sqrt(dx * dx + dy * dy);
    dx *= STEP_VEC_LEN / len;
    dy *= STEP_VEC_LEN / len;

    let mut current_x = p1.0;
    let mut current_y = p1.1;

    let mut try_put_pixel = |x: i32, y: i32, color| {
        if x >= w || x < 0 || y >= h || y < 0 {
            return true;
        }

        buf.put_pixel(x as u32, y as u32, color);
        false
    };

    // if (dx, dy) as length 1, we must iterate as many times as that will fit into the whole path
    for _ in 0..(len / STEP_VEC_LEN) as u32 {
        let (x, y) = (current_x as i32, current_y as i32);
        let mut quit = false;
        quit |= try_put_pixel(x, y, color);
        quit |= try_put_pixel(x + 1, y, color);
        quit |= try_put_pixel(x, y + 1, color);
        quit |= try_put_pixel(x - 1, y, color);
        quit |= try_put_pixel(x, y - 1, color);

        if quit {
            return;
        }

        // step forward
        current_x += dx;
        current_y += dy;
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
    // explicit cases (most work fine but not all like 0^x)
    if e == 1.0 {
        return b;
    }

    if b == 1.0 {
        return 1.0;
    }

    if e == 0.0 {
        return 1.0;
    }

    if b == 0.0 {
        return 0.0;
    }

    powi(b, e.trunc() as i64) * exp(e.fract() * ln(b))
}

fn ln(a: f64) -> f64 {
    // ln(1 + x) = ln(2)x-ln(2) approximation correction value
    const U: f64 = 0.03;
    // maximum mantissa value (2^52)
    const MANT: f64 = 4503599627370496.0;

    let a_bits = a.to_bits() as f64;
    // this is just an approximate transformation of the expression ln((1 + M/2^52) * 2^(E-1023))
    // in this expression the stuff inside the log is just the representation of an f64
    // M and E are the mantissa and exponent respectively
    let approx = (a_bits * LN_2) / MANT + U - 1023.0 * LN_2;

    let f = |x| exp(x) - a;
    let d = |x| exp(x);

    newton(f, d, approx, 5)
}

// e^x
fn exp(x: f64) -> f64 {
    let i = x.trunc() as i64;
    let f = x.fract();

    powi(E, i) * exp_taylor(f)
}

// e^x approximated as a taylor series
// only guaranteed to be accurate from -1 to 1
fn exp_taylor(x: f64) -> f64 {
    const TERMS: i64 = 20;
    let mut result = 0.0;

    for i in 0..TERMS {
        result += powi(x, i) / f(i as u64);
    }

    result
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