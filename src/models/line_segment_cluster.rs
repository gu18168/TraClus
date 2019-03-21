use crate::{
  models::{
    point::Point,
    candidate_point::CandidatePoint,
  }
};
use std::collections::HashSet;

pub struct LineSegmentCluster {
  id: usize,
  avg_direction_vector: Point,
  num_of_line_segments: usize,
  cos_theta: f64,
  sin_theta: f64,
  candidate_points: Vec<CandidatePoint>,
  points: Vec<Point>,
  trajectory_ids: HashSet<usize>,
  enabled: bool
}

impl LineSegmentCluster {
  pub fn new(id: usize) -> Self {
    Self {
      id,
      avg_direction_vector: Point::init(),
      num_of_line_segments: 0,
      cos_theta: 0.0,
      sin_theta: 0.0,
      candidate_points: Vec::new(),
      points: Vec::new(),
      trajectory_ids: HashSet::new(),
      enabled: false
    }
  }

  pub fn get_trajectory_length(&self) -> usize {
    self.trajectory_ids.len()
  }

  pub fn enable(&mut self) {
    self.enabled = true;
  }

  pub fn get_points(self) -> Vec<Point> {
    self.points
  }

  pub fn get_enable(&self) -> bool {
    self.enabled
  }

  pub fn get_sin(&self) -> f64 {
    self.sin_theta
  }

  pub fn get_cos(&self) -> f64 {
    self.cos_theta
  }

  pub fn add_point(&mut self, point: Point) {
    self.points.push(point);
  }

  pub fn add_x(&mut self, value: f64) {
    let origin = self.avg_direction_vector.get_x();
    self.avg_direction_vector.set_x(value + origin);
  }

  pub fn add_y(&mut self, value: f64) {
    let origin = self.avg_direction_vector.get_y();
    self.avg_direction_vector.set_y(value + origin);
  }

  pub fn avg_direction_vector_x(&mut self) {
    let val = self.avg_direction_vector.get_x();
    self.avg_direction_vector.set_x(val / self.num_of_line_segments as f64);
  }

  pub fn avg_direction_vector_y(&mut self) {
    let val = self.avg_direction_vector.get_y();
    self.avg_direction_vector.set_y(val / self.num_of_line_segments as f64);
  }

  pub fn set_theta(&mut self, cos_theta: f64, sin_thera: f64) {
    self.sin_theta = sin_thera;
    self.cos_theta = cos_theta;
  }

  pub fn add_num_of_line_segments(&mut self) {
    self.num_of_line_segments += 1;
  }

  pub fn get_num_of_line_segments(&self) -> usize {
    self.num_of_line_segments
  }

  pub fn get_avg_direcation_vector(&self) -> &Point {
    &self.avg_direction_vector
  }

  pub fn push(&mut self, point: CandidatePoint) {
    self.candidate_points.push(point);
  }

  pub fn len(&self) -> usize {
    self.candidate_points.len()
  }

  pub fn get_nth_candidate_point(&self, index: usize) -> &CandidatePoint {
    self.candidate_points.get(index).unwrap()
  }

  pub fn contains(&self, id: &usize) -> bool {
    self.trajectory_ids.contains(id)
  }

  pub fn insert(&mut self, id: usize) {
    self.trajectory_ids.insert(id);
  }

  pub fn sort(&mut self) {
    self.candidate_points.sort_by(|lhs, rhs| { lhs.get_ordering_value().partial_cmp(&rhs.get_ordering_value()).unwrap() });
  }
}