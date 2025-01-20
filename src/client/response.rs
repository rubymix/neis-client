#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::{
    ClassInfoItem, ClassRoomInfoItem, ElsTimetableItem, HisTimetableItem, MealServiceItem,
    MisTimetableItem, SchoolAflcoInfoItem, SchoolInfoItem, SchoolMajorInfoItem, SchoolScheduleItem,
    SpsTimetableItem,
};
use serde::Deserialize;

use super::AcademyInfoItem;

#[derive(Debug, Deserialize)]
pub struct ResultCode {
    CODE: String,
    MESSAGE: String,
}

#[derive(Debug, Deserialize)]
struct HeadFirst {
    list_total_count: usize,
}

#[derive(Debug, Deserialize)]
struct HeadSecond {
    RESULT: ResultCode,
}

#[derive(Debug, Deserialize)]
pub struct ResultHead {
    head: (HeadFirst, HeadSecond), // 배열
}

impl ResultHead {
    pub fn get_total_count(&self) -> usize {
        self.head.0.list_total_count
    }
}

#[derive(Debug, Deserialize)]
pub struct ResultBody<T> {
    pub row: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub enum ResponseBody {
    #[serde(rename = "RESULT")]
    Result(ResultCode),
    schoolInfo((ResultHead, ResultBody<SchoolInfoItem>)), // 배열
    classInfo((ResultHead, ResultBody<ClassInfoItem>)),   // 배열
    schoolMajorinfo((ResultHead, ResultBody<SchoolMajorInfoItem>)), // 배열
    schulAflcoinfo((ResultHead, ResultBody<SchoolAflcoInfoItem>)), // 배열
    SchoolSchedule((ResultHead, ResultBody<SchoolScheduleItem>)), // 배열
    elsTimetable((ResultHead, ResultBody<ElsTimetableItem>)), // 배열
    misTimetable((ResultHead, ResultBody<MisTimetableItem>)), // 배열
    hisTimetable((ResultHead, ResultBody<HisTimetableItem>)), // 배열
    spsTimetable((ResultHead, ResultBody<SpsTimetableItem>)), // 배열
    tiClrminfo((ResultHead, ResultBody<ClassRoomInfoItem>)), // 배열
    acaInsTiInfo((ResultHead, ResultBody<AcademyInfoItem>)), // 배열
    mealServiceDietInfo((ResultHead, ResultBody<MealServiceItem>)), // 배열
}
