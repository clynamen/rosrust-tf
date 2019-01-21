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
    transforms_ordered: VecDeque<TransformStorage>
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

impl TimeCacheInterface for TimeCache {

    fn get_data(&self, stamp: &Stamp) -> Result<TransformStorage, TfError> {
        let closest_res = self.find_closest(stamp)?;
        match(closest_res) {
            NoClose => {
                Err(TransformNotFound)
            },
            OneClose(ts) => {
                Ok(ts.clone())
            },
            TwoClose(left_ts, right_ts) => {
                Ok(interpolate_two_transform(left_ts, right_ts, stamp))
            }
        }
    }

    fn get_parent(&self, stamp: &Stamp) -> Result<FrameId, TfError> 
    {
        let closest_res = self.find_closest(stamp)?;
        match(closest_res) {
            TwoClose(left_ts, right_ts) => {
                Ok(left_ts.frame_id)
            }
            _ => {
                Err(NoParent)
            }
        }
    }

    fn insert_data(&mut self, new_ts: TransformStorage) -> bool {
        self.insert_ordered_by_time(new_ts);
        true
    }

    fn clear(&mut self) {
        self.transforms_ordered.clear();
    }

    fn get_latest_time_and_parent(&self) -> Option<(Stamp, FrameId)> {
        let latest_ts = self.transforms_ordered.front()?;
        Some((latest_ts.stamp.clone(), latest_ts.frame_id))
    }

    fn get_length(&self) -> usize {
        self.transforms_ordered.len()
    }

    fn get_latest_timestamp(&self) -> Option<Stamp> {
        Some(self.transforms_ordered.front()?.stamp.clone())
    }

    fn get_oldest_timestamp(&self) -> Option<Stamp> {
        Some(self.transforms_ordered.back()?.stamp.clone())
    }

}

impl TimeCache {

    fn new() -> TimeCache {
        TimeCache {
            transforms_ordered: VecDeque::new() 
        }
    }

    fn insert_ordered_by_time(&mut self, new_ts: TransformStorage) {
        // most recent at begin
        let insert_point = self.transforms_ordered.iter().filter(|x| x.stamp > new_ts.stamp).count();
        self.transforms_ordered.insert(insert_point, new_ts)
    }

    pub fn len(&self) -> usize {
        self.transforms_ordered.len()
    }

    pub fn insert(&mut self, ts: TransformStorage) {
        self.insert_ordered_by_time(ts);
    }

    pub fn find_closest(&self, req_time: &Stamp) -> Result<FindClosestResult, TfError>{
        if self.len() > 0 {
            if *req_time == Stamp::from_nanos(0) {
                let latest = self.transforms_ordered.front().unwrap();
                Ok(OneClose(latest))
            } else if self.len() == 1 {
                let latest = self.transforms_ordered.front().unwrap();
                if *req_time == latest.stamp {
                    Ok(OneClose(latest))
                } else {
                    Err(ExtrapolationError1)
                }
            } else {
                let earliest_tran = self.transforms_ordered.back().unwrap();
                let latest_tran   = self.transforms_ordered.front().unwrap();
                let earliest_stamp = &earliest_tran.stamp;
                let latest_stamp = &latest_tran.stamp;
                if *req_time == *earliest_stamp {
                    Ok(OneClose(earliest_tran))
                } else if *req_time == *latest_stamp {
                    Ok(OneClose(latest_tran))
                } else if *req_time < *earliest_stamp {
                    Err(ExtrapolationError2)
                } else if *req_time > *latest_stamp {
                    Err(ExtrapolationError3)
                } else {
                   let left_index = self.transforms_ordered.iter().filter(|x| *req_time > x.stamp).count() - 1;
                   let left_tran = self.transforms_ordered.get(left_index).unwrap();
                   let right_tran = self.transforms_ordered.get(left_index + 1).unwrap();
                   Ok(TwoClose(left_tran, right_tran))
                }
            }
        } else {
            Ok(NoClose)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_transform_storage_with_stamp(stamp: Stamp) -> TransformStorage {
        TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : stamp
        }
    }

    #[test]
    fn test_insert_first() {
        let mut time_cache = TimeCache::new();

        let ta = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(100)
        };

        assert_eq!(0usize, time_cache.len());
        time_cache.insert_ordered_by_time(ta);
        assert_eq!(1usize, time_cache.len());
    }

    #[test]
    fn test_insert_more_recent_after() {
        let mut time_cache = TimeCache::new();

        let t1 = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(100)
        };

        let t2 = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(200)
        };

        assert_eq!(0usize, time_cache.len());
        time_cache.insert_ordered_by_time(t1);
        time_cache.insert_ordered_by_time(t2);
        assert_eq!(2usize, time_cache.len());
        assert_eq!(Stamp::from_nanos(200), time_cache.transforms_ordered.get(0).unwrap().stamp);
    }

    fn test_insert_more_recent_before() {
        let mut time_cache = TimeCache::new();

        let t1 = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(100)
        };

        let t2 = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(200)
        };

        assert_eq!(0usize, time_cache.len());
        time_cache.insert_ordered_by_time(t2);
        time_cache.insert_ordered_by_time(t1);
        assert_eq!(2usize, time_cache.len());
        assert_eq!(Stamp::from_nanos(200), time_cache.transforms_ordered.get(0).unwrap().stamp);
    }

    #[test]
    fn test_insert_between() {
        let mut time_cache = TimeCache::new();

        for i in 0..12 {
            let ts = TransformStorage {
                frame_id       : 1u32, 
                child_frame_id : 2u32, 
                translation    : NTranslation3::new(0.0, 0.0, 0.0),  
                rotation       : NQuaternion::new(NVector3::z()), 
                stamp          : Stamp::from_nanos(i*100)
            };
            time_cache.insert_ordered_by_time(ts);
        }

        let t_between = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(650)
        };

        time_cache.insert_ordered_by_time(t_between);
        assert_eq!(Stamp::from_nanos(650), time_cache.transforms_ordered.get(5).unwrap().stamp);
    }

    #[test]
    fn test_find_closest_earliest() {
        let mut time_cache = TimeCache::new();
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(100)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(200)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(300)));    
        let res = time_cache.find_closest(&Stamp::from_nanos(100)).unwrap();

        match res {
            OneClose(res_transform) => {
                assert_eq!(Stamp::from_nanos(100), res_transform.stamp);
            },
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_find_closest_too_early() {
        let mut time_cache = TimeCache::new();
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(100)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(200)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(300)));    
        let res = time_cache.find_closest(&Stamp::from_nanos(99));

        match res {
            Err(ExtrapolationError2) => {
            },
            _ => {
                assert!(false, "result {:?} was not expected", res)
            }
        }
    }

    #[test]
    fn test_find_closest_latest() {
        let mut time_cache = TimeCache::new();
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(100)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(200)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(300)));    
        let res = time_cache.find_closest(&Stamp::from_nanos(300)).unwrap();

        match res {
            OneClose(res_transform) => {
                assert_eq!(Stamp::from_nanos(300), res_transform.stamp);
            },
            _ => {
                assert!(false)
            }
        }
        
    }

    #[test]
    fn test_find_closest_too_late() {
        let mut time_cache = TimeCache::new();
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(100)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(200)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(300)));    
        let res = time_cache.find_closest(&Stamp::from_nanos(303));

        match res {
            Err(ExtrapolationError3) => {
            },
            _ => {
                assert!(false, "result {:?} was not expected", res)
            }
        }
        
    }

    #[test]
    fn test_find_closest_between() {
        let mut time_cache = TimeCache::new();
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(100)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(200)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(400)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(300)));    
        let res = time_cache.find_closest(&Stamp::from_nanos(220)).unwrap();

        match res {
            TwoClose(left_tran, right_tran) => {
                assert_eq!(Stamp::from_nanos(300), left_tran.stamp);
                assert_eq!(Stamp::from_nanos(200), right_tran.stamp);
            },
            _ => {
                assert!(false)
            }
        }
        
    }

    #[test]
    fn get_data_closest_between() {
        let mut time_cache = TimeCache::new();
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(100)));    
        time_cache.insert_ordered_by_time(make_transform_storage_with_stamp(Stamp::from_nanos(400)));    

        let ts_a = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(1.0, 10.0, 100.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(200)
        };
        
        let ts_b = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(2.0, 9.0, 100.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(300)
        };
        time_cache.insert_ordered_by_time(ts_a);
        time_cache.insert_ordered_by_time(ts_b);

        let res = time_cache.get_data(&Stamp::from_nanos(220));
        assert!(res.is_ok());
        let ts = res.unwrap();

        assert_eq!(Stamp::from_nanos(220), ts.stamp);
        assert!(abs_diff_eq!(1.2,   ts.translation.vector.x));
        assert!(abs_diff_eq!(9.8,   ts.translation.vector.y));
        assert!(abs_diff_eq!(100.0, ts.translation.vector.z));

        
    }

}