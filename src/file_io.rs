use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    trajectory::Trajectory,
    cluster::Cluster
  }
};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};
use regex::Regex;

pub enum FileError {
  // 文件打开有误
  FileOpenError,
  // 维度读取有误
  DimensionError,
  // 轨迹 ID 读取有误
  IdError,
  // 维度数量匹配有误
  DimensionMismatchError,
  // 构建多维点有误
  DimensionPointError,
}

/// 读取文件的维度信息行
pub fn read_info_lines(path: &str) -> Result<usize, FileError> {
  let open_file = File::open(path);

  if let Ok(contents) = open_file {
    let mut line_of_contents = BufReader::new(contents);

    let dimension: usize;
    let mut line = String::new();

    // 利用正则表达式去除 CRLF
    let re = Regex::new(r"\d+").unwrap();

    // 读取第一行的维度
    line_of_contents.read_line(&mut line).unwrap();
    match re.captures(&line) {
      Some(res) => match res.get(0).unwrap().as_str().parse::<usize>() {
        Ok(dimen) => dimension = dimen,
        Err(_) => { return Err(FileError::DimensionError); }
      },
      None => { return Err(FileError::DimensionError); }
    }

    return Ok(dimension);
  } else {
    return Err(FileError::FileOpenError);
  }
}

/// 读取文件的轨迹行
pub fn read_trajectory_lines(path: &str, dimension: usize) -> Result<Vec<Trajectory>, FileError> {
  let open_file = File::open(path);

  if let Ok(contents) = open_file {
    let line_of_contents = BufReader::new(contents);
    let mut trajectorys: Vec<Trajectory> = Vec::new();

    // 根据文件构建轨迹
    // 注意需要跳过维度信息行
    for line in line_of_contents.lines().skip(1).filter_map(|result| result.ok()) {
      let trajectory_infos: Vec<&str> = line.split(' ').collect();

      if let Ok(trajectory_id) = trajectory_infos[0].parse::<usize>() { 
        let mut trajectory = Trajectory::new(trajectory_id, dimension);
        let len = trajectory_infos.len();
        let mut i = 1;

        // 点坐标的数量应该是维度的倍数
        // 注意要减去第一个 id 点
        if (len - 1) % dimension != 0 {
          return Err(FileError::DimensionMismatchError);
        }

        // 构建轨迹点并加入到轨迹中
        while i < len {
          let mut point: MultiDimenPoint = MultiDimenPoint::new(dimension);
          for j in 0..dimension {
            if let Ok(coord) = trajectory_infos[i + j].parse::<f64>() {
              point.set_nth_coordinate(j, coord);
            } else {
              return Err(FileError::DimensionPointError);
            }
          }

          // 我们能保证维度肯定相同，所以无视 Result
          trajectory.add_point(point).ok().unwrap();
          i += dimension;
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

pub fn write_cluster(out_path: &str, clusters: &Vec<Cluster>) {
  let mut file = OpenOptions::new().write(true).create(true)
    .open(out_path).expect("File can't write");
  
  for cluster in clusters {
    let info_line = cluster.get_id().to_string() + " cluster\tpoint num: " + &cluster.get_len().to_string() + "\n";
    file.write_all(info_line.as_bytes()).expect("File can't write");
    for point in cluster.get_points() {
      let x = point.get_nth_coordinate(0).unwrap();
      let y = point.get_nth_coordinate(1).unwrap();
      let point_line = x.to_string() + " " + &y.to_string() + "\t";
      file.write_all(point_line.as_bytes()).expect("File can't write");
    }
    file.write_all(b"\n").expect("File can't write");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_info_success() {
    assert_eq!(read_info_lines(".\\data\\deer_1995test.tra").ok().unwrap(), 2);
  }
}