use nalgebra::geometry::{Translation3, Quaternion, UnitQuaternion};
use nalgebra::Vector3;
use rosrust::{Time, Duration}; 


pub type NVector3 = Vector3<f64>;
pub type NTranslation3 = Translation3<f64>;
pub type NQuaternion = UnitQuaternion<f64>;
pub type FrameId = u32; 
pub type Stamp = Duration;

pub struct TransformStorage {
    pub frame_id       : FrameId,
    pub child_frame_id : FrameId,
    pub translation    : NTranslation3,
    pub rotation       : NQuaternion,
    pub stamp          : Stamp          
}

impl Clone for TransformStorage {
    fn clone(&self) -> TransformStorage {
        TransformStorage {
            frame_id:       self.frame_id,
            child_frame_id: self.child_frame_id,
            translation:    self.translation,
            rotation:       self.rotation,
            stamp:          self.stamp.clone()
        }
    }
}

pub trait ToSecDouble {
    fn to_sec_double(&self) -> f64;
}

impl ToSecDouble for Time {
    fn to_sec_double(&self) -> f64 {
        self.sec as f64 + self.nsec as f64 * 1.0e-9
    }
}

impl ToSecDouble for Duration {
    fn to_sec_double(&self) -> f64 {
        self.sec as f64 + self.nsec as f64 * 1.0e-9
    }
}

