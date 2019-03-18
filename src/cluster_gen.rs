use crate::{
  models::{
    trajectory::Trajectory,
    multi_dimen_point::MultiDimenPoint,
  },
  cluster_gen_util::{
    measure_distance_point_to_point,
    measure_perpendicular_distance,
    measure_angle_distance
  }
};

static MDL_COST_ADWANTAGE: usize = 25;

pub fn partition_trajectory<'a>(trajectory: &'a mut Trajectory<'a>) {
  let len = trajectory.get_points_len();
  let mut partition_indexs: Vec<usize> = Vec::new();
  // 添加起点到划分点中
  partition_indexs.push(0);

  let mut start_index = 1;
  let mut length = 0;
  let mut no_par_cost;
  let mut par_cost;

  loop {
    no_par_cost = 0;
    let end_index = start_index + length;

    while end_index < len {
      no_par_cost += compute_model_cost(
        trajectory.get_point(end_index - 1).unwrap(), 
        trajectory.get_point(end_index).unwrap()
      );

      par_cost = compute_model_cost(
        trajectory.get_point(start_index).unwrap(), 
        trajectory.get_point(end_index).unwrap()
      ) + compute_encoding_cost(
        trajectory,
        start_index, 
        end_index
      );

      if no_par_cost + MDL_COST_ADWANTAGE < par_cost {
        partition_indexs.push(end_index - 1);
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
  partition_indexs.push(len - 1);

  // 设置真正的划分点
  trajectory.add_partition_points(partition_indexs);
}

/// 计算 L(H)
fn compute_model_cost(start_point: &MultiDimenPoint, end_point: &MultiDimenPoint) -> usize {
  if let Some(distance) = measure_distance_point_to_point(start_point, end_point) {
    if distance < 1.0 { return 0; }

    distance.log2().ceil() as usize
  } else {
    0
  }
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

