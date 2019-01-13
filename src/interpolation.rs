use super::transform_storage::{FrameId, TransformStorage, ToSecDouble, Stamp,
                               NVector3, NTranslation3, NQuaternion};
use approx;


pub fn stamp_diff(a: &Stamp, b: &Stamp) -> Stamp {
    Stamp {
        sec: a.sec - b.sec,
        nsec: a.nsec - b.nsec
    }
}

pub fn translation_from_vector3(v: &NVector3) -> NTranslation3 {
    NTranslation3::new(v.x, v.y, v.z)
}

pub fn interpolate_vector3(va: &NVector3, vb: &NVector3, r: f64) -> NVector3 {
    va + (vb - va) * r
}

pub fn interpolate_quaternion(qa: &NQuaternion, qb: &NQuaternion, r: f64) -> NQuaternion {
    qa.slerp(qb, r)
}

pub fn interpolate_translation3(ta: &NTranslation3, tb: &NTranslation3, r: f64) -> NTranslation3 {
    let interpolated_vec = interpolate_vector3(&ta.vector, &tb.vector, r);
    translation_from_vector3(&interpolated_vec)
}

pub fn interpolate_two_transform(ta : &TransformStorage, tb: &TransformStorage, time: &Stamp) -> TransformStorage {
    if (ta.stamp == tb.stamp) {
        ta.clone()
    } else {
        let ratio = stamp_diff(time, &ta.stamp).to_sec_double()  / stamp_diff(&tb.stamp, &ta.stamp).to_sec_double();

        let interpolated_translation = interpolate_translation3(
            &ta.translation, 
            &tb.translation, ratio);

        let interpolated_rotation  = interpolate_quaternion(
            &ta.rotation, 
            &tb.rotation, ratio);

        TransformStorage {
            frame_id       : ta.frame_id,
            child_frame_id : ta.child_frame_id,
            translation    : interpolated_translation,
            rotation       : interpolated_rotation,
            stamp          : time.clone()
        }
    }
}


pub fn translation_test_equal(ta: &NTranslation3, tb: &NTranslation3) -> bool {
    let mut equal = true;
    equal = abs_diff_eq!(ta.vector.x, tb.vector.x) && equal;
    equal = abs_diff_eq!(ta.vector.y, tb.vector.y) && equal;
    equal = abs_diff_eq!(ta.vector.z, tb.vector.z) && equal;
    equal
}

pub fn rotation_test_equal(ra: &NQuaternion, rb: &NQuaternion) -> bool {
    let mut equal = true;
    equal = abs_diff_eq!(ra.coords.x, rb.coords.x) && equal;
    equal = abs_diff_eq!(ra.coords.y, rb.coords.y) && equal;
    equal = abs_diff_eq!(ra.coords.z, rb.coords.z) && equal;
    equal = abs_diff_eq!(ra.coords.w, rb.coords.w) && equal;
    equal
}

pub fn transform_storage_test_equal(ta : &TransformStorage, tb: &TransformStorage) -> bool {
    let mut equal = true;
    equal = ta.frame_id == tb.frame_id && equal; 
    equal = ta.child_frame_id == tb.child_frame_id && equal; 
    equal = ta.stamp == tb.stamp && equal; 
    equal = translation_test_equal(&ta.translation, &tb.translation) && equal;
    equal = rotation_test_equal(&ta.rotation, &tb.rotation) && equal;
    equal
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_interpolate_two_transform_same() {
        let ta = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(100)
        };

        let tb = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(0.0, 0.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(100)
        };

        assert!(transform_storage_test_equal(&interpolate_two_transform(&ta, &tb, &Stamp::from_nanos(100)), &ta));
    }


    #[test]
    fn test_interpolate_two_transform_1() {
        let ta = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(1.0, 1.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(100)
        };

        let tb = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(2.0, 2.0, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(200)
        };

        let tc = TransformStorage {
            frame_id       : 1u32, 
            child_frame_id : 2u32, 
            translation    : NTranslation3::new(1.25, 1.25, 0.0),  
            rotation       : NQuaternion::new(NVector3::z()), 
            stamp          : Stamp::from_nanos(125)
        };

        assert!(transform_storage_test_equal(&interpolate_two_transform(&ta, &tb, &Stamp::from_nanos(125)), &tc));
    }

}
