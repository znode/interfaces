// use rand::{Alphanumeric, Distribution, StandardUniform};
use rand::{
    Rng,
    distr::{Alphanumeric, Distribution, StandardUniform},
};
use std::time::{SystemTime, UNIX_EPOCH};
use std_msgs::Time;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub fn random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn random_bytes(length: usize) -> Vec<u8> {
    (0..length).map(|_| rand::random::<u8>()).collect()
}

pub fn empty_bytes() -> Vec<u8> {
    Vec::new()
}

pub fn random_floats(length: usize) -> Vec<f32> {
    (0..length).map(|_| rand::random::<f32>()).collect()
}

pub fn random_doubles(length: usize) -> Vec<f64> {
    (0..length).map(|_| rand::random::<f64>()).collect()
}

impl Distribution<std_msgs::Header> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> std_msgs::Header {
        let now = SystemTime::now();
        let now_as_duration = now
            .duration_since(UNIX_EPOCH)
            .expect("System time went backwards");
        std_msgs::Header {
            stamp: Some(Time {
                sec: now_as_duration.as_secs(),
                nanosec: now_as_duration.subsec_nanos(),
            }),
            frame_id: random_string(16),
        }
    }
}

impl Distribution<geometry_msgs::Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Point {
        geometry_msgs::Point {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }
}

impl Distribution<geometry_msgs::Quaternion> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Quaternion {
        geometry_msgs::Quaternion {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
            w: rng.random(),
        }
    }
}

impl Distribution<geometry_msgs::Vector3> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Vector3 {
        geometry_msgs::Vector3 {
            x: rng.random(),
            y: rng.random(),
            z: rng.random(),
        }
    }
}

impl Distribution<geometry_msgs::Vector3Stamped> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Vector3Stamped {
        geometry_msgs::Vector3Stamped {
            header: Some(rng.random()),
            vector: Some(rng.random()),
        }
    }
}

impl Distribution<geometry_msgs::Pose> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Pose {
        geometry_msgs::Pose {
            position: Some(rng.random()),
            orientation: Some(rng.random()),
        }
    }
}

impl Distribution<geometry_msgs::Twist> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Twist {
        geometry_msgs::Twist {
            linear: Some(rng.random()),
            angular: Some(rng.random()),
        }
    }
}

impl Distribution<geometry_msgs::TwistWithCovariance> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::TwistWithCovariance {
        geometry_msgs::TwistWithCovariance {
            twist: Some(rng.random()),
            covariance: random_doubles(36),
        }
    }
}

impl Distribution<geometry_msgs::TwistWithCovarianceStamped> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::TwistWithCovarianceStamped {
        geometry_msgs::TwistWithCovarianceStamped {
            header: Some(rng.random()),
            twist: Some(rng.random()),
        }
    }
}

impl Distribution<geometry_msgs::Wrench> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::Wrench {
        geometry_msgs::Wrench {
            force: Some(rng.random()),
            torque: Some(rng.random()),
        }
    }
}

impl Distribution<geometry_msgs::WrenchStamped> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> geometry_msgs::WrenchStamped {
        geometry_msgs::WrenchStamped {
            header: Some(rng.random()),
            wrench: Some(rng.random()),
        }
    }
}

impl Distribution<sensor_msgs::Image> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> sensor_msgs::Image {
        sensor_msgs::Image {
            header: Some(rng.random()),
            height: rng.random(),
            width: rng.random(),
            encoding: random_string(32),
            is_bigendian: rng.random(),
            step: rng.random(),
            //data: random_bytes(1920 * 1080 * 3),
            data: empty_bytes(),
        }
    }
}

impl Distribution<sensor_msgs::point_field::DataType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> sensor_msgs::point_field::DataType {
        match rng.random_range(0..=7) {
            0 => sensor_msgs::point_field::DataType::Int8,
            1 => sensor_msgs::point_field::DataType::Uint8,
            2 => sensor_msgs::point_field::DataType::Int16,
            3 => sensor_msgs::point_field::DataType::Uint16,
            4 => sensor_msgs::point_field::DataType::Int32,
            5 => sensor_msgs::point_field::DataType::Uint32,
            6 => sensor_msgs::point_field::DataType::Float32,
            7 => sensor_msgs::point_field::DataType::Float64,
            _ => sensor_msgs::point_field::DataType::Int8,
        }
    }
}

impl Distribution<sensor_msgs::PointField> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> sensor_msgs::PointField {
        sensor_msgs::PointField {
            name: random_string(32),
            offset: rng.random(),
            datatype: rng.random(),
            count: rng.random(),
        }
    }
}

fn random_point_fields(length: usize) -> Vec<sensor_msgs::PointField> {
    (0..length)
        .map(|_| rand::random::<sensor_msgs::PointField>())
        .collect()
}

impl Distribution<sensor_msgs::PointCloud2> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> sensor_msgs::PointCloud2 {
        sensor_msgs::PointCloud2 {
            header: Some(rng.random()),
            height: rng.random(),
            width: rng.random(),
            fields: random_point_fields(3),
            is_bigendian: rng.random(),
            point_step: rng.random(),
            row_step: rng.random(),
            //data: random_bytes(4 * 4 * 4 * 1280 * 960),
            data: empty_bytes(),
            is_dense: rng.random(),
        }
    }
}

impl Distribution<sensor_msgs::LaserScan> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> sensor_msgs::LaserScan {
        sensor_msgs::LaserScan {
            header: Some(rng.random()),
            angle_min: rng.random(),
            angle_max: rng.random(),
            angle_increment: rng.random(),
            time_increment: rng.random(),
            scan_time: rng.random(),
            range_min: rng.random(),
            range_max: rng.random(),
            ranges: random_floats(1024),
            intensities: random_floats(1024),
        }
    }
}
