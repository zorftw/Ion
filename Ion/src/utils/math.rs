use std::intrinsics::sqrtf32;

/**
    Math related utilities
*/

pub mod vec;

pub fn normalize(vector: &mut vec::Vec3) {
    let wrap = |mut value: f32| -> f32 {
        while value < -180 as f32 {
            value += 360 as f32;
        }
        while value > 180 as f32 {
            value -= 360 as f32;
        }

        value
    };

    vector.x = wrap(vector.x);
    vector.y = wrap(vector.y);
    vector.z = wrap(vector.z);
}

pub fn clamp(vector: &mut vec::Vec3) {
    if vector.y > 180 as f32 {
        vector.y = 180 as f32;
    } else if vector.y < -180 as f32 {
        vector.y = -180 as f32;
    }

    if vector.x > 89 as f32 {
        vector.x = 89 as f32;
    } else if vector.x < -89 as f32 {
        vector.x = -89 as f32;
    }

    vector.z = 0 as f32;
}

pub fn calc_angle(source: vec::Vec3, dest: vec::Vec3) -> vec::Vec3 {
    unsafe {
        let mut res = vec::Vec3::empty();
        let delta = source - dest;

        let hyp = sqrtf32(delta.x * delta.x + delta.y * delta.y);

        res.x = f32::to_degrees(f32::atan(delta.z / hyp));
        res.y = f32::to_degrees(f32::atan(delta.y / delta.x));
        res.z = 0 as f32;

        if delta.x >= 0.0 {
            res.y += 180 as f32;
        }

        res
    }
}