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

pub trait ExtractFromResponse {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>)
    where
        Self: Sized;
}

impl ExtractFromResponse for SchoolInfoItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::schoolInfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for ClassInfoItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::classInfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for SchoolMajorInfoItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::schoolMajorinfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for SchoolAflcoInfoItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::schulAflcoinfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for SchoolScheduleItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::SchoolSchedule((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for ElsTimetableItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::elsTimetable((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for MisTimetableItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::misTimetable((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for HisTimetableItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::hisTimetable((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for SpsTimetableItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::spsTimetable((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for ClassRoomInfoItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::tiClrminfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for AcademyInfoItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::acaInsTiInfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}

impl ExtractFromResponse for MealServiceItem {
    fn extract_from_response(response: ResponseBody) -> (usize, Vec<Self>) {
        if let ResponseBody::mealServiceDietInfo((head, data)) = response {
            (head.get_total_count(), data.row)
        } else {
            Default::default()
        }
    }
}
