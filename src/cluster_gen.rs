use crate::{
  models::{
    line_segment_cluster::LineSegmentCluster,
    line_segment::LineSegment,
    point::Point,
    candidate_point::CandidatePoint,
    cluster::Cluster
  },
  distance_util::{
    compute_inner_product,
    compute_vector_length
  }
};
use std::collections::HashSet;

static MIN_LINE_SEGMENT_LENGTH: f64 = 50.0;

pub fn construct_line_segment_cluster(max_index: usize, min_lns: usize,
  cluster_indexs: Vec<i32>, line_segments: Vec<LineSegment>)  -> Vec<LineSegmentCluster>
{
  let mut line_segment_clusters: Vec<LineSegmentCluster> = Vec::with_capacity(max_index);

  // 初始化所有的线段簇
  for i in 0..max_index {
    line_segment_clusters.push(LineSegmentCluster::new(i));
  }

  // 开始计算各个簇的平均方向向量
  for i in 0..line_segments.len() {
    let index = *cluster_indexs.get(i).unwrap();
    if index >= 0 {
      let (start_point, end_point) = line_segments.get(i).unwrap().extract_start_end_points();

      let diff_x: f64 = end_point.get_x() - start_point.get_x();
      let diff_y: f64 = end_point.get_y() - start_point.get_y();
      line_segment_clusters.get_mut(index as usize).unwrap().add_x(diff_x);
      line_segment_clusters.get_mut(index as usize).unwrap().add_y(diff_y);

      line_segment_clusters.get_mut(index as usize).unwrap().add_num_of_line_segments();
    }
  }

  let vector = Point::new(1.0, 0.0);
  for i in 0..max_index {
    let cluster_entry = line_segment_clusters.get_mut(i).unwrap();

    cluster_entry.avg_direction_vector_x();
    cluster_entry.avg_direction_vector_y();
    
    let avg_direction_vector = cluster_entry.get_avg_direcation_vector();
    let vector_length = compute_vector_length(avg_direction_vector);
    let inner_product = compute_inner_product(avg_direction_vector, &vector);
    let mut cos_theta = inner_product / vector_length;
    cos_theta = if cos_theta > 1.0 { 1.0 } else if cos_theta < -1.0 { -1.0 } else { cos_theta };
    let mut sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
    sin_theta = if avg_direction_vector.get_y() < 0.0 { -sin_theta } else { sin_theta };

    cluster_entry.set_theta(cos_theta, sin_theta);
  }

  // 构建在平均方向上的候选点
  for i in 0..line_segments.len() {
    let index = *cluster_indexs.get(i).unwrap();
    if index >= 0 {
      get_candidate_points(line_segment_clusters.get_mut(index as usize).unwrap(), line_segments.get(i).unwrap(), i);
    }
  }

  // 给候选点排序
  for line_segment_cluster in line_segment_clusters.iter_mut() {
    line_segment_cluster.sort();
  }

  // 根据候选点计算扫描线
  for line_segment_cluster in line_segment_clusters.iter_mut() {
    if line_segment_cluster.get_trajectory_length() >= min_lns {
      line_segment_cluster.enable();
      compute_representative_lines(min_lns, line_segment_cluster, &line_segments);
    }
  }

  line_segment_clusters
}

pub fn construct_cluster(line_segment_clusters: Vec<LineSegmentCluster>) -> Vec<Cluster> {
  let mut curr_id: usize = 0;
  let clusters: Vec<Cluster> = line_segment_clusters.into_iter()
    .filter_map(|line_segment_cluster| {
      if !line_segment_cluster.get_enable() { return None; }
      
      let cluster = Cluster::new(curr_id, line_segment_cluster.get_points());
      curr_id += 1;

      Some(cluster)
    })
    .collect();

  clusters
}

fn get_candidate_points(cluster: &mut LineSegmentCluster, line_segment: &LineSegment, line_index: usize) {
  let (start_point, end_point) = line_segment.extract_start_end_points();
  let ordering_value_1 = get_x_rotation(
    start_point.get_x(), 
    start_point.get_y(),
    cluster.get_cos(), cluster.get_sin());
  let ordering_value_2 = get_x_rotation(
    end_point.get_x(), 
    end_point.get_y(),
    cluster.get_cos(), cluster.get_sin());

  let candidate_point_1 = CandidatePoint::new(line_index, ordering_value_1, true);
  let candidate_point_2 = CandidatePoint::new(line_index, ordering_value_2, false);
  cluster.push(candidate_point_1);
  cluster.push(candidate_point_2);

  let trajectory_id = line_segment.get_trajectory_id();
  if !cluster.contains(&trajectory_id) {
    cluster.insert(trajectory_id);
  }
}

fn get_x_rotation(x: f64, y: f64, cos: f64, sin: f64) -> f64 {
  x * cos + y * sin
}

fn get_y_rotation(x: f64, y: f64, cos: f64, sin: f64) -> f64 {
  -x * sin + y * cos
}

fn get_x_rev_rotation(x: f64, y: f64, cos: f64, sin: f64) -> f64 {
  x * cos - y * sin
}

fn get_y_rev_rotation(x: f64, y: f64, cos: f64, sin: f64) -> f64 {
  x * sin + y * cos
}

fn compute_representative_lines(min_lns: usize,
  cluster: &mut LineSegmentCluster, line_segments: &Vec<LineSegment>) -> usize {
  let mut line_segments_list: HashSet<usize> = HashSet::new();
  let mut insertion_list: HashSet<usize> = HashSet::new();
  let mut deletion_list: HashSet<usize> = HashSet::new();

  let mut iter = 0;
  let mut candidate_point: &CandidatePoint;
  let mut next_candidate_point: &CandidatePoint;
  let mut prev_ordering_value: f64 = 0.0;
  let mut cluster_points: usize = 0;

  line_segments_list.clear();

  while iter != (cluster.len() - 1) && cluster.len() > 0 {
    insertion_list.clear();
    deletion_list.clear();

    loop {
      candidate_point = cluster.get_nth_candidate_point(iter);
      iter += 1;
      let id = candidate_point.get_line_segment_id();
      if !line_segments_list.contains(&id) {
        insertion_list.insert(id);
        line_segments_list.insert(id);
      } else {
        deletion_list.insert(id);
      }

      if iter != (cluster.len() - 1) {
        next_candidate_point = cluster.get_nth_candidate_point(iter);
      } else {
        break;
      }

      if candidate_point.get_ordering_value() != next_candidate_point.get_ordering_value() {
        break;
      }
    }

    for insertion in insertion_list.iter() {
      if deletion_list.contains(insertion) {
        deletion_list.remove(insertion);
        line_segments_list.remove(insertion);
      }

      let mut del: i32 = -1;
      for deletion in deletion_list.iter() {
        if line_segments.get(*insertion).unwrap().get_trajectory_id() 
          == line_segments.get(*deletion).unwrap().get_trajectory_id()
        {
          line_segments_list.remove(deletion);
          del = *deletion as i32;
          break;
        }
      }

      // 用了一种很诡异的方法来删除
      if del >= 0 {
        deletion_list.remove(&(del as usize));      
      }
    }

    let mut point: Option<Point> = None;
    if line_segments_list.len() >= min_lns {
      if (candidate_point.get_ordering_value() - prev_ordering_value).abs() > (MIN_LINE_SEGMENT_LENGTH / 1.414) {
        point = Some(compute_cluster_point(cluster, line_segments, candidate_point.get_ordering_value(), &line_segments_list));
        prev_ordering_value = candidate_point.get_ordering_value();
        cluster_points += 1;
      }
    }

    if let Some(point) = point {
      cluster.add_point(point);
    }

    for deletion in deletion_list.iter() {
      line_segments_list.remove(deletion);
    }
  }

  if cluster_points >= 2 {
    return cluster_points;
  }

  return 0;
}

fn compute_cluster_point(cluster: &LineSegmentCluster, line_segments: &Vec<LineSegment>,
  value: f64, line_segments_list: &HashSet<usize>) -> Point
{
  let line_segments_len = line_segments_list.len();
  let mut cluster_point = Point::init();
  let mut sweep_point: Point;

  for line_segment_id in line_segments_list {
    sweep_point = get_sweep_point(cluster, value, line_segments.get(*line_segment_id).unwrap());
    let coordinate_x = cluster_point.get_x() + sweep_point.get_x() / line_segments_len as f64;
    let coordinate_y = cluster_point.get_y() + sweep_point.get_y() / line_segments_len as f64;

    cluster_point.set_x(coordinate_x);
    cluster_point.set_y(coordinate_y);
  }

  let orig_x = get_x_rev_rotation(
    cluster_point.get_x(), 
    cluster_point.get_y(), 
    cluster.get_cos(), cluster.get_sin());
  let orig_y = get_y_rev_rotation(
    cluster_point.get_x(), 
    cluster_point.get_y(), 
    cluster.get_cos(), cluster.get_sin());

  cluster_point.set_x(orig_x);
  cluster_point.set_y(orig_y);

  cluster_point
}

fn get_sweep_point(cluster: &LineSegmentCluster, value: f64, line_segment: &LineSegment) -> Point {
  let (start_point, end_point) = line_segment.extract_start_end_points();

  let new_start_x = get_x_rotation(start_point.get_x(),
    start_point.get_y(), 
    cluster.get_cos(), cluster.get_sin());
  let new_end_x = get_x_rotation(end_point.get_x(),
    end_point.get_y(), 
    cluster.get_cos(), cluster.get_sin());
  let new_start_y = get_y_rotation(start_point.get_x(),
    start_point.get_y(), 
    cluster.get_cos(), cluster.get_sin());
  let new_end_y = get_y_rotation(end_point.get_x(),
    end_point.get_y(), 
    cluster.get_cos(), cluster.get_sin());

  let cofficient = (value - new_start_x) / (new_end_x - new_start_x);
  let sweep_point = Point::new(value, new_start_y + cofficient * (new_end_y - new_start_y));

  sweep_point
}