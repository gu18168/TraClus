use crate::{
  models::{
    point::Point,
    trajectory::Trajectory,
    cluster::Cluster
  }
};
use chrono::prelude::*;
use csv::Reader;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};

#[derive(Debug)]
pub enum FileError {
  // 文件打开有误
  FileOpenError,
  // 轨迹 ID 读取有误
  IdError,
  // 维度数量匹配有误
  DimensionMismatchError,
  // 构建二维点有误
  DimensionPointError,
}

/// 读取文件的轨迹行
pub fn read_trajectory_lines(path: &str) -> Result<Vec<Trajectory>, FileError> {
  let open_file = File::open(path);

  if let Ok(contents) = open_file {
    let line_of_contents = BufReader::new(contents);
    let mut trajectorys: Vec<Trajectory> = Vec::new();

    // 根据文件构建轨迹
    for line in line_of_contents.lines().filter_map(|result| result.ok()) {
      let trajectory_infos: Vec<&str> = line.split(' ').collect();
      let len = trajectory_infos.len();

      // 点坐标的数量应该是维度的倍数
      // 注意要减去第一个 id 点
      if (len - 1) % 2 != 0 {
        return Err(FileError::DimensionMismatchError);
      }

      if let Ok(trajectory_id) = trajectory_infos[0].parse::<usize>() { 
        let mut trajectory = Trajectory::new(trajectory_id);

        // 构建轨迹点并加入到轨迹中
        let mut i = 1;
        while i < len {
          let mut point: Point = Point::init();

          // 设置 x
          if let Ok(coord) = trajectory_infos[i].parse::<f64>() {
            point.set_x(coord);
          } else {
            return Err(FileError::DimensionPointError);
          }

          // 设置 y
          if let Ok(coord) = trajectory_infos[i + 1].parse::<f64>() {
            point.set_y(coord);
          } else {
            return Err(FileError::DimensionPointError);
          }

          trajectory.add_point(point);
          i += 2;
        }

        trajectorys.push(trajectory);
      } else { 
        return Err(FileError::IdError) 
      };
    }

    return Ok(trajectorys);
  } else {
    return Err(FileError::FileOpenError);
  }
}

/// 从 csv 文件读取轨迹行
pub fn read_trajectory_lines_from_csv(path: &str) -> Result<Vec<Trajectory>, FileError> {
  let open_file = Reader::from_path(path);

  if let Ok(mut rdr) = open_file {
    let mut trajectorys: Vec<Trajectory> = Vec::new();
    let mut prev_mmsi = String::from("");

    for record in rdr.records().filter_map(|result| result.ok()) {
      let mmsi = record.get(0).unwrap();
      let sog: f64 = record.get(1).unwrap().parse().expect("SOG must be a f64!");
      let longitude: f64 = record.get(2).unwrap().parse().expect("longitude must be a f64!");
      let latitude: f64 = record.get(3).unwrap().parse().expect("latitude must be a f64!");
      let hour: usize = record.get(4).unwrap().parse().expect("Hour must be a usize!");
      let mins: usize = record.get(5).unwrap().parse().expect("Mins must be a usize!");
      let secs: usize = record.get(6).unwrap().parse().expect("Secs must be a usize!");

      let timestamp = time_to_second(
        &(String::from("20181222_") + 
        hour.to_string().as_str() + 
        mins.to_string().as_str() + 
        secs.to_string().as_str())
      ).expect("Time format must be %Y%m%d_%H%M%S!");

      let point = Point::new(longitude, latitude, sog, timestamp);

      if prev_mmsi != mmsi {
        let mut trajectory = Trajectory::new(trajectorys.len());
        trajectory.add_point(point);
        trajectorys.push(trajectory);
        prev_mmsi = String::from(mmsi);
      } else {
        let index = trajectorys.len() - 1;
        trajectorys[index].add_point(point);
      }
    }

    return Ok(trajectorys);
  } else {
    return Err(FileError::FileOpenError);
  }
}

fn time_to_second(time: &str) -> Result<i64, FileError> {
  let date = Utc.datetime_from_str(time, "%Y%m%d_%H%M%S");
  if let Ok(date) = date {
    return Ok(date.timestamp());
  } else {
    return Err(FileError::DimensionPointError);
  }
}

/// 将簇写入到文件中
pub fn write_cluster(out_path: &str, clusters: &Vec<Cluster>) {
  let mut file = OpenOptions::new().write(true).create(true)
    .open(out_path).expect("File can't write");
  
  for cluster in clusters {
    let info_line = cluster.get_id().to_string() + " cluster\tpoint num: " + &cluster.get_len().to_string() + "\n";
    file.write_all(info_line.as_bytes()).expect("File can't write");
    for point in cluster.get_points() {
      let x = point.get_x();
      let y = point.get_y();
      let point_line = x.to_string() + " " + &y.to_string() + "\t";
      file.write_all(point_line.as_bytes()).expect("File can't write");
    }
    file.write_all(b"\n").expect("File can't write");
  }
}