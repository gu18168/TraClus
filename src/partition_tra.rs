use crate::{
  models::{
    trajectory::Trajectory,
    thick_trajectory::ThickTrajectory,
    point::Point,
    line_segment::LineSegment
  },
  distance_util::{
    measure_distance_point_to_point,
    measure_perpendicular_distance,
    measure_angle_distance
  }
};
use std::collections::HashSet;

static MDL_COST_ADWANTAGE: usize = 25;
static MIN_LINE_SEGMENT_LENGTH: f64 = 50.0;

/// 将轨迹抽象为划分轨迹
pub fn partition_trajectories(trajectories: Vec<Trajectory>) -> Vec<ThickTrajectory> {
  // 直接所有权转移
  trajectories.into_iter()
    .map(|trajectory| partition_trajectory(trajectory))
    .collect()
}

/// 划分单条轨迹
fn partition_trajectory(trajectory: Trajectory) -> ThickTrajectory {
  let len = trajectory.get_points_len();
  let mut partition_indexs = HashSet::new();

  // 添加起点到划分点中
  partition_indexs.insert(0);

  let mut start_index = 0;
  let mut length;
  let mut no_par_cost;
  let mut par_cost;

  // 执行 MDL 算法
  loop {
    no_par_cost = 0;
    length = 1;

    while start_index + length < len {
      let end_index = start_index + length;
      no_par_cost += compute_model_cost(
        trajectory.get_point(end_index - 1).unwrap(), 
        trajectory.get_point(end_index).unwrap()
      );

      par_cost = compute_model_cost(
        trajectory.get_point(start_index).unwrap(), 
        trajectory.get_point(end_index).unwrap()
      ) + compute_encoding_cost(
        &trajectory,
        start_index, 
        end_index
      );

      if no_par_cost + MDL_COST_ADWANTAGE < par_cost {
        partition_indexs.insert(end_index - 1);
        start_index = end_index - 1;
        length = 0;
        break;
      } else {
        length += 1;
      }
    }

    if start_index + length >= len { break; }
  }

  // 添加终点到划分点中
  partition_indexs.insert(len - 1);

  let id = trajectory.get_id();
  let points = trajectory.get_points();
  // 获得所有的轨迹点
  let partition_points: Vec<Point> = points.into_iter()
    .enumerate()
    .filter_map(|(index, point)| {
      if partition_indexs.contains(&index) { 
        return Some(point); 
      }
      else { return None; }
    })
    .collect();

  ThickTrajectory::new(id, partition_points)
}

/// 计算 L(H)
fn compute_model_cost(start_point: &Point, end_point: &Point) -> usize {
  let distance = measure_distance_point_to_point(start_point, end_point);
  if distance < 1.0 { return 0; }

  distance.log2().ceil() as usize
}

// 计算 L(D|H)
fn compute_encoding_cost(trajectory: &Trajectory, start_index: usize, end_index: usize) -> usize {
  let start_point = trajectory.get_point(start_index).unwrap();
  let end_point = trajectory.get_point(end_index).unwrap();
  let mut encoding_cost: usize = 0;

  for i in start_index..end_index {
    let line_start_point = trajectory.get_point(i).unwrap();
    let line_end_point = trajectory.get_point(i + 1).unwrap();

    let mut perpendicular_distance = measure_perpendicular_distance(start_point, end_point, line_start_point, line_end_point);
    let mut angle_distance = measure_angle_distance(start_point, end_point, line_start_point, line_end_point);

    if perpendicular_distance < 1.0 { perpendicular_distance = 1.0; }
    if angle_distance < 1.0 { angle_distance = 1.0; }

    encoding_cost += (perpendicular_distance.log2().ceil() + angle_distance.log2().ceil()) as usize;
  }

  encoding_cost
}

/// 将轨迹的划分点相连成为线段存入数组中
pub fn get_partition_line(trajectories: &Vec<ThickTrajectory>) -> Vec<LineSegment> {
  let mut line_segments = Vec::new();

  for trajectory in trajectories.iter() {
    for i in 0..(trajectory.get_len() - 1) {
      let start_point = trajectory.get_partition_point(i).unwrap();
      let end_point = trajectory.get_partition_point(i + 1).unwrap();

      if measure_distance_point_to_point(start_point, end_point) < MIN_LINE_SEGMENT_LENGTH {
        continue;
      }

      let line_segment = LineSegment::new(trajectory.get_id(), start_point, end_point);
      line_segments.push(line_segment);
    }
  }

  line_segments
} 
