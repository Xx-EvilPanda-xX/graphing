use std::{f64::consts::{E, LN_2, PI}, fmt::Debug, str::FromStr};
use image::{RgbImage, Rgb, imageops};

fn main() {
    // let f = |x| (x - 1.0) * (x + 2.0) * (x + 0.4) * (x - 0.2) * (x + 0.8);
    // let d = |x| (5.0*x*x*x*x) + (8.0*x*x*x) - ((69.0*x*x)/25.0) - ((496.0*x)/125.0) - 0.224;

    // area under tan(x) from 0 to 1
    println!("{}", -ln(cos(1.0)));

    let b: f64 = input("Enter a base for exponentiation:");
    let e: f64 = input("Enter an exponent for exponentiation:");

    let (std, custom) = (b.powf(e), pow(b, e));
    println!("std pow: {}, custom pow: {}", std, custom);
    println!("std sin: {}, custom sin: {}", std.sin(), sin(custom));
    println!("std cos: {}, custom cos: {}", std.cos(), cos(custom));
    println!("std tan: {}, custom tan: {}", std.tan(), tan(custom));

    let size = input("Enter a size in pixels for the graph:");
    let thickness = input("Enter a line thickness for graphing:");
    let bound: f64 = input("Enter a bound for graphing:");

    let mut img = RgbImage::new(size, size);
    draw_axes(&mut img, thickness);

    let f1 = |x| pow(2.22222, x);
    let f2 = |x| sqrt(x);
    let f3 = |x| (x - 1.0) * (x + 2.0) * (x + 0.4) * (x - 0.2) * (x + 0.8);
    let f4 = |x| pow(x, x);
    let f5 = |x| 1.0 / x;
    let f6 = |x| log(2.0, x);
    let f7 = |x| tan(x);
    let f8 = |x| sin(cos(x));
    let f9 = |x| LN_2 * x - LN_2;

    let scale = bound / size as f64;

    println!("Graphing f1!");
    graph(f1, &mut img, scale, Rgb([255, 0, 0]), thickness);
    println!("Graphing f2!");
    graph(f2, &mut img, scale, Rgb([0, 255, 0]), thickness);
    println!("Graphing f3!");
    graph(f3, &mut img, scale, Rgb([0, 0, 255]), thickness);
    println!("Graphing f4!");
    graph(f4, &mut img, scale, Rgb([255, 0, 255]), thickness);
    println!("Graphing f5!");
    graph(f5, &mut img, scale, Rgb([0, 255, 255]), thickness);
    println!("Graphing f6!");
    graph(f6, &mut img, scale, Rgb([255, 255, 0]), thickness);
    println!("Graphing f7!");
    graph(f7, &mut img, scale, Rgb([0, 255, 128]), thickness);
    println!("Graphing f8!");
    graph(f8, &mut img, scale, Rgb([128, 128, 128]), thickness);
    println!("Graphing f9!");
    graph(f9, &mut img, scale, Rgb([255, 128, 0]), thickness);

    imageops::flip_vertical_in_place(&mut img);
    img.save("out.png").expect("Failed to save image");
}

fn input<T>(prompt: &str) -> T
    where T: FromStr,
        T::Err: Debug
{
    println!("{prompt}");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().parse().expect("Failed to parse input")
}

fn draw_axes(buf: &mut RgbImage, thickness: i32) {
    let half_w = buf.width() as f64 / 2.0;
    let half_h = buf.height() as f64 / 2.0;

    draw_line(buf, (0.0, half_h), (half_w * 2.0, half_h), Rgb([255; 3]), thickness, false);
    draw_line(buf, (half_w, 0.0), (half_w, half_h * 2.0), Rgb([255; 3]), thickness, false);
}

fn graph<F: Fn(f64) -> f64>(f: F, buf: &mut RgbImage, scale: f64, color: Rgb<u8>, thickness: i32) {
    let half_w = buf.width() as f64 / 2.0;
    let half_h = buf.height() as f64 / 2.0;

    for x in 0..buf.width() {
        let x1 = (x as f64 - half_w) * scale;
        // divide by scale to bring output back to screen coords, clamp to i32 range to avoid overflow later
        let y1 = f(x1) / scale + half_h;

        let x2 = ((x + 1) as f64 - half_w) * scale;
        let y2 = f(x2) / scale + half_h;

        if y1.is_nan() || y2.is_nan() {
            continue;
        }

        draw_line(buf, (x as f64, y1), ((x + 1) as f64, y2), color, thickness, true);
    }
}

fn draw_line(buf: &mut RgbImage, mut p1: (f64, f64), mut p2: (f64, f64), color: Rgb<u8>, thickness: i32, can_quit: bool) {
    let w = buf.width() as i32;
    let h = buf.height() as i32;

    // ensure our starting point isn't gonna immediately trigger a quit from trying to render outside the image
    let t = thickness as f64;
    if p1.0 + t >= w as f64 || p1.0 - t < 0.0 || p1.1 + t >= h as f64 || p1.1 - t < 0.0 {
        std::mem::swap(&mut p1, &mut p2);
    }

    // direction vector (p1 -> p2)
    let mut vx = p2.0 - p1.0;
    let mut vy = p2.1 - p1.1;

    // how big should each step be? (smaller = nicer line)
    const STEP_VEC_LEN: f64 = 0.25;

    // normalize vector to length STEP_VEC_LEN
    let len = sqrt(vx * vx + vy * vy);
    vx *= STEP_VEC_LEN / len;
    vy *= STEP_VEC_LEN / len;

    let mut current_x = p1.0;
    let mut current_y = p1.1;

    let mut try_put_pixel = |x: i32, y: i32, color| {
        if x >= w || x < 0 || y >= h || y < 0 {
            return true;
        }

        buf.put_pixel(x as u32, y as u32, color);
        false
    };

    // if (dx, dy) has length `STEP_VEC_LEN`, we must iterate as many times as that will fit into the whole path from p1 to p2
    for _ in 0..(len / STEP_VEC_LEN) as u32 {
        if current_x + t > i32::MAX as f64 || current_x - t < i32::MIN as f64 || current_y + t > i32::MAX as f64 || current_y - t < i32::MIN as f64 {
            return;
        }

        let (x, y) = (current_x as i32, current_y as i32);
        let mut quit = false;

        // color only the pixels within a radius `thickness` of our pixel
        for dx in x - thickness..x + thickness {
            for dy in y - thickness..y + thickness {
                if (dx - x) * (dx - x) + (dy - y) * (dy - y) < thickness * thickness {
                    quit |= try_put_pixel(dx, dy, color);
                }
            }
        }

        if quit && can_quit {
            return;
        }

        // step forward
        current_x += vx;
        current_y += vy;
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

// performs `iters` newton iterations on the function `f` with derivative `d`
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

// calculates log_b(a)
fn log(b: f64, a: f64) -> f64 {
    ln(a) / ln(b)
}

// calculates b^e
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
    // ln(1 + x) = ln(2)x-ln(2) approximation correction value for smallest average error
    const U: f64 = 0.03972;
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

// calculates e^x
fn exp(x: f64) -> f64 {
    let i = x.trunc() as i64;
    let f = x.fract();

    powi(E, i) * exp_taylor(f)
}

// e^x approximated as a taylor polynomial
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

fn tan(x: f64) -> f64 {
    sin(x) / cos(x)
}

// sin and cos of integers from -6 to 6 (approximately the period of sin and cos)
static SIN_TABLE: [(f64, i32); 13] = [(0.27941549819892586, -6), (0.9589242746631385, -5), (0.7568024953079282, -4), (-0.1411200080598672, -3), (-0.9092974268256817, -2), (-0.8414709848078965, -1), (0.0, 0), (0.8414709848078965, 1), (0.9092974268256817, 2), (0.1411200080598672, 3), (-0.7568024953079282, 4), (-0.9589242746631385, 5), (-0.27941549819892586, 6)];
static COS_TABLE: [(f64, i32); 13] = [(0.960170286650366, -6), (0.28366218546322625, -5), (-0.6536436208636119, -4), (-0.9899924966004454, -3), (-0.4161468365471424, -2), (0.5403023058681398, -1), (1.0, 0), (0.5403023058681398, 1), (-0.4161468365471424, 2), (-0.9899924966004454, 3), (-0.6536436208636119, 4), (0.28366218546322625, 5), (0.960170286650366, 6)];

// taylor polynomial approximation of sin(x)
fn sin(x: f64) -> f64 {
    const TERMS: i64 = 20;
    let x = x % (2.0 * PI);
    let (s, a) = closest(&SIN_TABLE, x);
    let (c, _) = closest(&COS_TABLE, x);

    let mut result = 0.0;
    for i in 0..TERMS {
        // mod four because the derivatives of cos are cyclical with a period of 4
        let coeff = match i % 4 {
            0 => s,
            1 => c,
            2 => -s,
            3 => -c,
            _ => unreachable!(),
        };

        result += coeff * powi(x - a, i) / f(i as u64);
    }

    result
}

// taylor polynomial approximation of cos(x)
fn cos(x: f64) -> f64 {
    const TERMS: i64 = 20;
    let x = x % (2.0 * PI);
    let (c, a) = closest(&COS_TABLE, x);
    let (s, _) = closest(&SIN_TABLE, x);

    let mut result = 0.0;
    for i in 0..TERMS {
        // mod four because the derivatives of cos are cyclical with a period of 4
        let coeff = match i % 4 {
            0 => c,
            1 => -s,
            2 => -c,
            3 => s,
            _ => unreachable!(),
        };

        result += coeff * powi(x - a, i) / f(i as u64);
    }

    result
}

// finds the value in arr which has a arr[_].1 value closest to x
fn closest(arr: &[(f64, i32)], x: f64) -> (f64, f64) {
    let (mut distance, (mut num, mut pos)) = (-1.0, (0.0, 0.0));

    for (i, loc) in arr {
        let dist = (*loc as f64 - x).abs();
        if dist < distance || distance == -1.0 {
            distance = dist;
            num = *i;
            pos = *loc as f64;
        }
    }

    (num, pos)
}