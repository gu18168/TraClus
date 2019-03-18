use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint
  }
};

/// 计算两个点的坐标系距离
pub fn measure_distance_point_to_point(lp: &MultiDimenPoint, rp: &MultiDimenPoint) -> Option<f64> {
  if lp.get_dimension() != rp.get_dimension() { return None; }

  let mut square_sum = 0.0;

  for i in 0..lp.get_dimension() {
    square_sum += lp.get_nth_coordinate(i).unwrap().powf(2.0) + rp.get_nth_coordinate(i).unwrap().powf(2.0);
  }

  Some(square_sum.sqrt())
}

/// 计算一个点到一条线段的距离
pub fn measure_distance_point_to_line(point: &MultiDimenPoint, line_start: &MultiDimenPoint, line_end: &MultiDimenPoint) -> Option<f64> {
  if point.get_dimension() != line_start.get_dimension() { return None; }

  let dimension = point.get_dimension();
  let mut vector_1 = MultiDimenPoint::new(dimension);
  let mut vector_2 = MultiDimenPoint::new(dimension);
  let mut project_point = MultiDimenPoint::new(dimension);

  for i in 0..dimension {
    vector_1.set_nth_coordinate(i, point.get_nth_coordinate(i).unwrap() - line_start.get_nth_coordinate(i).unwrap());
    vector_2.set_nth_coordinate(i, line_end.get_nth_coordinate(i).unwrap() - line_start.get_nth_coordinate(i).unwrap());
  }

  // 获得投影点的坐标
  let cofficient = compute_inner_product(&vector_1, &vector_2) / compute_inner_product(&vector_2, &vector_2);
  for i in 0..dimension {
    project_point.set_nth_coordinate(i, line_start.get_nth_coordinate(i).unwrap() + cofficient * vector_2.get_nth_coordinate(i).unwrap());
  }

  measure_distance_point_to_point(point, &project_point)
}

/// 计算两条线段的垂直距离
pub fn measure_perpendicular_distance(line_1_start: &MultiDimenPoint, line_1_end: &MultiDimenPoint,
  line_2_start: &MultiDimenPoint, line_2_end: &MultiDimenPoint) -> f64 
{
  let distance_1 = measure_distance_point_to_line(line_2_start, line_1_start, line_1_end).unwrap();
  let distance_2 = measure_distance_point_to_line(line_2_end, line_1_start, line_1_end).unwrap();

  if distance_1 == 0.0 && distance_2 == 0.0 { return 0.0; }

  (distance_1.powi(2) + distance_2.powi(2)) / (distance_1 + distance_2)
}

/// 计算两条线段的角度距离
pub fn measure_angle_distance(line_1_start: &MultiDimenPoint, line_1_end: &MultiDimenPoint,
  line_2_start: &MultiDimenPoint, line_2_end: &MultiDimenPoint) -> f64 
{
  let dimension = line_1_start.get_dimension();
  let mut vector_1 = MultiDimenPoint::new(dimension);
  let mut vector_2 = MultiDimenPoint::new(dimension);
  
  for i in 0..dimension {
    vector_1.set_nth_coordinate(i, line_1_end.get_nth_coordinate(i).unwrap() - line_1_start.get_nth_coordinate(i).unwrap());
    vector_2.set_nth_coordinate(i, line_2_end.get_nth_coordinate(i).unwrap() - line_2_start.get_nth_coordinate(i).unwrap());
  }

  let vector_1_length = compute_vector_length(&vector_1);
  let vector_2_length = compute_vector_length(&vector_2);

  if vector_1_length == 0.0 || vector_2_length == 0.0 { return 0.0; }

  let inner_product = compute_inner_product(&vector_1, &vector_2);
  let mut cos_theta = inner_product / (vector_1_length * vector_2_length);
  cos_theta = if cos_theta > 1.0 { 1.0 } else if cos_theta < -1.0 { -1.0 } else { cos_theta };
  let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

  vector_2_length * sin_theta
}

/// 计算两个向量的点乘
fn compute_inner_product(vector_1: &MultiDimenPoint, vector_2: &MultiDimenPoint) -> f64 {
  let dimension = vector_1.get_dimension();
  let mut inner_product = 0.0;

  for i in 0..dimension {
    inner_product += vector_1.get_nth_coordinate(i).unwrap() * vector_2.get_nth_coordinate(i).unwrap();
  }

  inner_product
}

/// 计算一个向量的长度
fn compute_vector_length(vector: &MultiDimenPoint) -> f64 {
  let mut square_sum = 0.0;

  for i in 0..vector.get_dimension() {
    square_sum += vector.get_nth_coordinate(i).unwrap().powi(2);
  }

  square_sum.sqrt()
}