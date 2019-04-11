use crate::{
  models::{
    point::Point
  }
};

/// 计算一个点到一个点的距离
pub fn measure_distance_point_to_point(lp: &Point, rp: &Point) -> f64 {
  let mut square_sum = 0.0;

  square_sum += (lp.get_x() - rp.get_x()).powi(2);
  square_sum += (lp.get_y() - rp.get_y()).powi(2);

  square_sum.sqrt()
}

/// 计算一个点到一条线段的距离
pub fn measure_distance_point_to_line(point: &Point, line_start: &Point, line_end: &Point) -> f64 {
  let (_, project_point) = project_point_to_line(point, line_start, line_end);

  measure_distance_point_to_point(point, &project_point)
}

/// 计算一条线段到一条线段的距离
pub fn measure_distance_line_to_line(line_1_start: &Point, line_1_end: &Point,
  line_2_start: &Point, line_2_end: &Point) -> f64 
{
  let length_1 = measure_distance_point_to_point(line_1_start, line_1_end);
  let length_2 = measure_distance_point_to_point(line_2_start, line_2_end);

  let perpendicular_distance = if length_1 > length_2 {
    measure_perpendicular_distance(line_1_start, line_1_end, line_2_start, line_2_end)
  } else {
    measure_perpendicular_distance(line_2_start, line_2_end, line_1_start, line_1_end)
  };
  let angle_distance = if length_1 > length_2 {
    measure_angle_distance(line_1_start, line_1_end, line_2_start, line_2_end)
  } else {
    measure_angle_distance(line_2_start, line_2_end, line_1_start, line_1_end)
  };
  let parallel_distance = if length_1 > length_2 {
    measure_parallel_distance(line_1_start, line_1_end, line_2_start, line_2_end)
  } else {
    measure_parallel_distance(line_2_start, line_2_end, line_1_start, line_1_end)
  };

  parallel_distance + perpendicular_distance + angle_distance
}

/// 计算两条线段的垂直距离
/// 注意，默认第一条线段比第二条线段长
pub fn measure_perpendicular_distance(line_1_start: &Point, line_1_end: &Point,
  line_2_start: &Point, line_2_end: &Point) -> f64 
{
  let distance_1 = measure_distance_point_to_line(line_2_start, line_1_start, line_1_end);
  let distance_2 = measure_distance_point_to_line(line_2_end, line_1_start, line_1_end);

  if distance_1 == 0.0 && distance_2 == 0.0 { return 0.0; }

  (distance_1.powi(2) + distance_2.powi(2)) / (distance_1 + distance_2)
}

/// 计算两条线段的角度距离
/// 注意，默认第一条线段比第二条线段长
pub fn measure_angle_distance(line_1_start: &Point, line_1_end: &Point,
  line_2_start: &Point, line_2_end: &Point) -> f64 
{
  let vector_2 = Point::new(
    line_2_end.get_x() - line_2_start.get_x(),
    line_2_end.get_y() - line_2_start.get_y(),
    0.0,
    0
  );
  let vector_2_length = compute_vector_length(&vector_2);

  let cos_theta = compute_vectors_cos(line_1_start, line_1_end, line_2_start, line_2_end);
  let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

  vector_2_length * sin_theta
}

/// 计算两条线段的平行距离
/// 注意，默认第一条线段比第二条线段长
pub fn measure_parallel_distance(line_1_start: &Point, line_1_end: &Point,
  line_2_start: &Point, line_2_end: &Point) -> f64
{
  let (cofficient_1, project_point_1) = project_point_to_line(line_2_start, line_1_start, line_1_end);
  let parallel_1 = if cofficient_1 < 0.5 { 
    measure_distance_point_to_point(line_1_start, &project_point_1)
  } else {
    measure_distance_point_to_point(line_1_end, &project_point_1)
  };

  let (cofficient_2, project_point_2) = project_point_to_line(line_2_end, line_1_start, line_1_end);
  let parallel_2 = if cofficient_2 < 0.5 { 
    measure_distance_point_to_point(line_1_start, &project_point_2)
  } else {
    measure_distance_point_to_point(line_1_end, &project_point_2)
  };

  if parallel_1 < parallel_2 { parallel_1 } else { parallel_2 }
}

/// 计算两个向量的点乘
pub fn compute_inner_product(vector_1: &Point, vector_2: &Point) -> f64 {
  let mut inner_product = 0.0;

  inner_product += vector_1.get_x() * vector_2.get_x();
  inner_product += vector_1.get_y() * vector_2.get_y();

  inner_product
}

/// 计算一个向量的长度
pub fn compute_vector_length(vector: &Point) -> f64 {
  let mut square_sum = 0.0;

  square_sum += vector.get_x().powi(2);
  square_sum += vector.get_y().powi(2);

  square_sum.sqrt()
}

/// 计算两个向量之间的 cos
pub fn compute_vectors_cos(line_1_start: &Point, line_1_end: &Point,
  line_2_start: &Point, line_2_end: &Point) -> f64 
{
  let vector_1 = Point::new(
    line_1_end.get_x() - line_1_start.get_x(),
    line_1_end.get_y() - line_1_start.get_y(),
    0.0,
    0
  );
  let vector_2 = Point::new(
    line_2_end.get_x() - line_2_start.get_x(),
    line_2_end.get_y() - line_2_start.get_y(),
    0.0,
    0
  );

  let vector_1_length = compute_vector_length(&vector_1);
  let vector_2_length = compute_vector_length(&vector_2);

  if vector_1_length == 0.0 || vector_2_length == 0.0 { return 0.0; }

  let inner_product = compute_inner_product(&vector_1, &vector_2);
  let mut cos_theta = inner_product / (vector_1_length * vector_2_length);
  cos_theta = if cos_theta > 1.0 { 1.0 } else if cos_theta < -1.0 { -1.0 } else { cos_theta };

  cos_theta
}

// 获得一个点对于一条线段的投影点
fn project_point_to_line(point: &Point, line_start: &Point, line_end: &Point) -> (f64, Point) {
  let vector_1 = Point::new(
    point.get_x() - line_start.get_x(),
    point.get_y() - line_start.get_y(),
    0.0,
    0
  );
  let vector_2 = Point::new(
    line_end.get_x() - line_start.get_x(),
    line_end.get_y() - line_start.get_y(),
    0.0,
    0
  );

  // 获得投影点的坐标
  // @BUG 出现了除以 0
  let cofficient = compute_inner_product(&vector_1, &vector_2) / compute_inner_product(&vector_2, &vector_2);
  let project_point = Point::new(
    line_start.get_x() + cofficient * vector_2.get_x(),
    line_start.get_y() + cofficient * vector_2.get_y(),
    0.0,
    0
  );

  (cofficient, project_point)
}