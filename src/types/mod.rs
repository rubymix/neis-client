mod academy_info;
mod class_info;
mod class_room_info;
mod els_timetable;
mod his_timetable;
mod meal_service;
mod mis_timetable;
mod school_aflco_info;
mod school_info;
mod school_major_info;
mod school_schedule;
mod sps_timetable;

pub use academy_info::{AcademyInfoItem, AcademyInfoParams};
pub use class_info::{ClassInfoItem, ClassInfoParams};
pub use class_room_info::{ClassRoomInfoItem, ClassRoomInfoParams};
pub use els_timetable::{ElsTimetableItem, ElsTimetableParams};
pub use his_timetable::{HisTimetableItem, HisTimetableParams};
pub use meal_service::{MealServiceItem, MealServiceParams};
pub use mis_timetable::{MisTimetableItem, MisTimetableParams};
pub use school_aflco_info::{SchoolAflcoInfoItem, SchoolAflcoInfoParams};
pub use school_info::{SchoolInfoItem, SchoolInfoParams};
pub use school_major_info::{SchoolMajorInfoItem, SchoolMajorInfoParams};
pub use school_schedule::{SchoolScheduleItem, SchoolScheduleParams};
use serde::Deserialize;
pub use sps_timetable::{SpsTimetableItem, SpsTimetableParams};

#[derive(Debug, Deserialize, PartialEq)]
pub enum YesOrNo {
    Y,
    N,
}

fn deserialize_u8_from_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn deserialize_i32_from_string<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn deserialize_i32_from_f64<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let n = f64::deserialize(deserializer)?;
    Ok(n as i32)
}
