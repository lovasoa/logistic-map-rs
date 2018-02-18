extern crate gnuplot;
extern crate rand;
extern crate rayon;
#[macro_use] extern crate log;
extern crate env_logger;

use rayon::prelude::*;
use gnuplot::{Figure, PointSymbol};
use rand::{Rng, Open01, XorShiftRng};

fn fun (lambda: f32, x: f32) -> f32 {
  lambda * x * (1.-x)
}

fn compute_y(lambda: f32, seed: f32) -> f32 {
  let mut x = seed;
  for _ in 1..1000 {x = fun(lambda, x)}
  x
}

fn points(f: f32, t:f32, n:usize) -> Vec<(f32,f32)>{

  let mut rng = XorShiftRng::new_unseeded();
  let seeds : Vec<f32> = rng.gen_iter::<Open01<f32>>()
    .take(n)
    .map(|Open01(x)| x)
    .collect();

  let step = (t-f) / (n as f32);

  (0..n)
    .into_par_iter()
    .zip(seeds)
    .map(|(k, seed)| {
      let x = f + (k as f32)*step;
      let y = compute_y(x, seed);
      (x,y)
    })
    .collect()
}

fn main() {
    env_logger::init();
    info!("Starting");

    let xy = points(1.0, 4.0, 1_000_000);

    info!("y calculé");

    let x : Vec<f32> = xy.iter().map(|&(x,_y)| x).collect(); 
    let y : Vec<f32> = xy.iter().map(|&(_x,y)| y).collect(); 
    let mut fg = Figure::new();
    fg.axes2d().points(&x, &y, &[PointSymbol('.')]);
    fg.show();

    info!("fini");
}

