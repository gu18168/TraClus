extern crate traclus;

use std::env;
use traclus::{
  models::{
    trajectory::Trajectory
  },
  file_io::{
    read_trajectory_lines, 
    write_cluster,
    FileError
  },
  partition_tra::{
    partition_trajectories,
    get_partition_line,
  },
  dbscan::{
    perform_dbscan
  },
  cluster_gen::{
    construct_line_segment_cluster,
    construct_cluster
  }
};

fn main() {
  let args: Vec<String> = env::args().collect();

  // args[1] input file
  // args[2] output file
  // args[3] eps
  // args[4] minLns
  if args.len() == 5 {
    let eps: f64 = args[3].parse().expect("eps isn't a Double!");
    let min_lns: usize = args[4].parse().expect("minLns isn't a Number!");

    let trajectories: Vec<Trajectory>;

    // 获得轨迹信息
    match read_trajectory_lines(&args[1]) {
      Ok(trajs) => { trajectories = trajs; },
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
    let thick_trajectories = partition_trajectories(trajectories);
    let line_segments = get_partition_line(&thick_trajectories);

    // 执行聚类
    let (cluster_indexs, cluster_index) = perform_dbscan(eps, min_lns, &line_segments);

    // 构建聚类
    let line_segment_clusters = construct_line_segment_cluster(cluster_index, min_lns, cluster_indexs, line_segments);
    let clusters = construct_cluster(line_segment_clusters);

    // 写聚类信息到文件中
    write_cluster(&args[2], &clusters);
  } else {
    println!("Please give me 4 input parameters(inputFilePath, outputFilePath, eps, minLns)!");
    println!("--e.g. cargo run deer_1995.tra testOut.txt 29 8");
  }
}
