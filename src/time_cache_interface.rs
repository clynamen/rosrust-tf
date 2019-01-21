use super::transform_storage::{FrameId, TransformStorage, ToSecDouble, Stamp,
                               NVector3, NTranslation3, NQuaternion};
use super::interpolation::interpolate_two_transform;
use approx;

use std::collections::VecDeque;

pub trait TimeCacheInterface {
    fn get_data(&self, stamp: &Stamp) -> Result<TransformStorage, TfError>;
    fn get_parent(&self, stamp: &Stamp) -> Result<FrameId, TfError>;
    fn insert_data(&mut self, new_ts: TransformStorage) -> bool;
    fn clear(&mut self);
    fn get_latest_time_and_parent(&self) -> Option<(Stamp, FrameId)>;
    fn get_length(&self) -> usize;
    fn get_latest_timestamp(&self) -> Option<Stamp>;
    fn get_oldest_timestamp(&self) -> Option<Stamp>;
}

pub struct TimeCache {
    pub transforms_ordered: VecDeque<TransformStorage>
}

#[derive(Debug)]
pub enum TfError {
    Generic(&'static str),
    TransformNotFound,
    ExtrapolationError1,
    ExtrapolationError2,
    ExtrapolationError3,
    NoParent
}

#[derive(Debug)]
pub enum FindClosestResult<'a> {
    NoClose,
    OneClose(&'a TransformStorage), 
    TwoClose(&'a TransformStorage, &'a TransformStorage)
}

use FindClosestResult::*;
use TfError::*;