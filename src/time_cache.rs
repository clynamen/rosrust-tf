use super::transform_storage::{FrameId, TransformStorage, ToSecDouble, Stamp,
                               NVector3, NTranslation3, NQuaternion};
use approx;

use std::collections::VecDeque;

pub struct TimeCache {
    transforms_ordered: VecDeque<TransformStorage>
}

pub enum TfError {
    Generic(String)
}

pub enum FindClosestResult {
    One(TransformStorage), 
    Two(TransformStorage, TransformStorage)
}

pub impl {

    fn insert_ordered_by_time(&mut self, ts: TransformStorage) {
        // most recent at begin
        self.transforms_ordered.iter().cou
    }

    pub fn insert(&mut self, ts: TransformStorage) {
        self.insert_ordered_by_time(ts);
    }

    pub fn find_closest(&self, time: &Stamp) -> Result<FindClosestResult>{

    }
}

