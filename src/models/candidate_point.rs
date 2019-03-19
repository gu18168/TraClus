pub struct CandidatePoint {
  ordering_value: f64,
  line_segment_id: usize,
  is_start: bool
}

impl CandidatePoint {
  pub fn new(line_segment_id: usize, ordering_value: f64, is_start: bool) -> Self {
    Self {
      ordering_value,
      line_segment_id,
      is_start
    }
  }

  pub fn get_ordering_value(&self) -> f64 {
    self.ordering_value
  }

  pub fn get_line_segment_id(&self) -> usize {
    self.line_segment_id
  }
}