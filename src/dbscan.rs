use crate::{
  models::{
    merge_indexs::MergeIndexs,
    line_segment::LineSegment
  },
  distance_util::{
    measure_distance_line_to_line
  }
};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use uuid::Uuid;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};

static NOISE: i32 = -1;

pub fn perform_dbscan(eps: f64, min_lns: usize, line_segments: &Vec<LineSegment>) -> (Vec<i32>, usize) {
  let pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();

  let len = line_segments.len();

  let merge_indexs = MergeIndexs::new();
  let merge_indexs = Arc::new(Mutex::new(merge_indexs));
  let core_uuids: HashSet<&Uuid> = HashSet::new();
  let core_uuids = Arc::new(RwLock::new(core_uuids));

  for i in 0..len {
    let line_1 = line_segments.get(i).unwrap();
    let (line_1_start, line_1_end) = line_1.extract_start_end_points();
    let iter_of_line_segments = line_segments.iter().enumerate();

    let clone_merge_indexs = Arc::clone(&merge_indexs);
    let clone_core_uuids = Arc::clone(&core_uuids);

    let is_core = pool.install(|| {
      let mut cluster_size: usize = 0;
      let mut can_merge_index: Vec<usize> = Vec::new();

      for (index, line_2) in iter_of_line_segments {
        let (line_2_start, line_2_end) = line_2.extract_start_end_points();
        // 距离相近，平均速度相近，加速度方向相同
        if measure_distance_line_to_line(line_1_start, line_1_end, line_2_start, line_2_end) <= eps
          && (line_1.get_sog() - line_2.get_sog()).abs() < 2.0
          && line_1.get_acc() * line_2.get_acc() >= 0.0
        {
          cluster_size += 1;

          if clone_core_uuids.read().unwrap().contains(line_2.get_uuid()) {
            can_merge_index.push(index);
          }
        }
      }

      if cluster_size >= min_lns {
        if can_merge_index.len() > 0 {
          clone_merge_indexs.lock().unwrap().set_to_min(&can_merge_index, i);
        } else {
          clone_merge_indexs.lock().unwrap().push(i);
        }

        return true;
      }

      false
    });

    if is_core {
      core_uuids.write().unwrap().insert(line_1.get_uuid());
    }
  }

  merge_indexs.lock().unwrap().correct_indexs();
  let merge_cluster_indexs = merge_indexs.lock().unwrap().map_indexs();

  let mut result: Vec<i32> = vec![NOISE; line_segments.len()];
  
  for (index, merge_cluster_index) in merge_cluster_indexs.iter().enumerate() {
    let mut core_segements: Vec<&LineSegment> = Vec::new();

    for index in merge_cluster_index {
      core_segements.push(line_segments.get(*index).unwrap());
    }

    let others: Vec<usize> = line_segments.par_iter()
      .enumerate()
      .filter_map(|(i, line_segment)| {
        let (line_1_start, line_1_end) = line_segment.extract_start_end_points();
        for core_segement in core_segements.iter() {
          let (line_2_start, line_2_end) = core_segement.extract_start_end_points();
          if measure_distance_line_to_line(line_1_start, line_1_end, line_2_start, line_2_end) <= eps {
            return Some(i);
          }
        }
        return None;
      })
      .collect();

    for other in others {
      result[other] = index as i32;
    }
  }

  (result, merge_cluster_indexs.len())
}
