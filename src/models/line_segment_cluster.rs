use crate::{
  models::{
    multi_dimen_point::MultiDimenPoint,
    candidate_point::CandidatePoint,
  }
};
use std::collections::HashSet;

pub struct LineSegmentCluster {
  id: usize,
  avg_direction_vector: MultiDimenPoint,
  num_of_line_segments: usize,
  cos_theta: f64,
  sin_theta: f64,
  candidate_points: Vec<CandidatePoint>,
  points: Vec<MultiDimenPoint>,
  trajectory_ids: HashSet<usize>,
  enabled: bool
}

impl LineSegmentCluster {
  pub fn new(id: usize, dimension: usize) -> Self {
    Self {
      id,
      avg_direction_vector: MultiDimenPoint::new(dimension),
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

  pub fn get_points(self) -> Vec<MultiDimenPoint> {
    self.points
  }

  pub fn get_enable(&self) -> bool {
    self.enabled
  }

  pub fn get_sin(&self) -> &f64 {
    &self.sin_theta
  }

  pub fn get_cos(&self) -> &f64 {
    &self.cos_theta
  }

  pub fn add_point(&mut self, point: MultiDimenPoint) {
    self.points.push(point);
  }

  pub fn add_nth_direction_vector(&mut self, j: usize, value: f64) {
    let origin = self.avg_direction_vector.get_nth_coordinate(j).unwrap();
    self.avg_direction_vector.set_nth_coordinate(j, value + origin);
  }

  pub fn avg_nth_direction_vector(&mut self, j: usize) {
    let val = self.avg_direction_vector.get_nth_coordinate(j).unwrap();
    self.avg_direction_vector.set_nth_coordinate(j, val / self.num_of_line_segments as f64);
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

  pub fn get_avg_direcation_vector(&self) -> &MultiDimenPoint {
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