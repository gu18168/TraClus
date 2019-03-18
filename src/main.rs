extern crate traclus;

use std::env;
use traclus::{
  models::{
    trajectory::Trajectory
  },
  file_io::{
    read_info_lines, 
    read_trajectory_lines, 
    FileError
  },
  cluster_gen::{
    partition_trajectory
  }
};

fn main() {
  let args: Vec<String> = env::args().collect();

  // args[1] input file
  // args[2] output file
  // args[3] eps
  // args[4] minLns
  if args.len() == 5 {
    let dimension: usize;
    let mut trajectorys: Vec<Trajectory>;

    // 获得维度信息
    match read_info_lines(&args[1]) {
      Ok(dimen) => {
        dimension = dimen;
      }
      Err(e) => {
        match e {
          FileError::FileOpenError => {
            eprintln!("Can't open the input file!");
          }
          FileError::DimensionError => {
            eprintln!("The type of Dimension isn't a usize!");
          }
          _ => {
            eprintln!("Something wrong!");
          }
        }
        return;
      }
    }

    // 获得轨迹信息
    match read_trajectory_lines(&args[1], dimension) {
      Ok(trajs) => { trajectorys = trajs; },
      Err(e) => {
        match e {
          FileError::DimensionMismatchError => {
            eprintln!("The sum of infos isn't multiple of dimension!");
          },
          FileError::DimensionPointError => {
            eprintln!("The coordination of point isn't a f64!");
          },
          FileError::IdError => {
            eprintln!("The id of trajectory isn't a usize");
          }
          _ => {
            eprintln!("Something wrong!");
          }
        }
        return;
      }
    }

    // 划分轨迹
    for mut trajectory in trajectorys {
      partition_trajectory(&mut trajectory);
    }

  } else {
    println!("Please give me 4 input parameters(inputFilePath, outputFilePath, eps, minLns)!");
    println!("--e.g. cargo run deer_1995.tra testOut.txt 29 8");
  }
}
