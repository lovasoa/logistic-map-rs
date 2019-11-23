use gnuplot::{Figure, PointSymbol};
use rand::{distributions::Open01, Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;

use log::{info, log};

fn fun (lambda: f32, x: f32) -> f32 {
  lambda * x * (1.-x)
}

fn compute_y(lambda: f32, seed: f32) -> f32 {
  let mut x = seed;
  for _ in 1..1000 {x = fun(lambda, x)}
  x
}

fn points(f: f32, t:f32, n:usize) -> Vec<(f32,f32)>{
  let rng = XorShiftRng::from_entropy();
  let seeds: Vec<f32> = rng.sample_iter(Open01).take(n).collect();

  let step = (t-f) / (n as f32);

  (0..n)
    .into_par_iter()
    .zip(seeds)
    .map(|(k, seed)| {
      let x = step.mul_add(k as f32, f);
      let y = compute_y(x, seed);
      (x,y)
    })
    .collect()
}

fn main() {
  flexi_logger::Logger::with_env()
    .format(flexi_logger::detailed_format)
    .start()
    .unwrap();

  info!("Starting");

  let xy = points(1.0, 4.0, 1_000_000);

  info!("y calculé");

  let x : Vec<f32> = xy.iter().map(|&(x,_y)| x).collect(); 
  let y : Vec<f32> = xy.iter().map(|&(_x,y)| y).collect(); 
  let mut fg = Figure::new();
  fg.axes2d().points(&x, &y, &[PointSymbol('.')]);
  fg.show().unwrap();

  info!("fini");
}

