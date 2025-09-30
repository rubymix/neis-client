#![allow(non_snake_case)]
use super::ToQueryString;
use super::deserialize_i32_from_string;
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct SchoolScheduleParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 주야과정명
    pub DGHT_CRSE_SC_NM: Option<String>,
    /// 학교과정명
    pub SCHUL_CRSE_SC_NM: Option<String>,
    /// 학사일자
    pub AA_YMD: Option<String>,
    /// 학사시작일자
    pub AA_FROM_YMD: Option<String>,
    /// 학사종료일자
    pub AA_TO_YMD: Option<String>,
}

impl SchoolScheduleParams {
    pub fn new(atpt_ofcdc_sc_code: &str, sd_schul_code: &str) -> Self {
        Self {
            ATPT_OFCDC_SC_CODE: atpt_ofcdc_sc_code.to_owned(),
            SD_SCHUL_CODE: sd_schul_code.to_owned(),
            DGHT_CRSE_SC_NM: None,
            SCHUL_CRSE_SC_NM: None,
            AA_YMD: None,
            AA_FROM_YMD: None,
            AA_TO_YMD: None,
        }
    }

    pub fn ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        self.AA_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
    pub fn from_ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        self.AA_FROM_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
    pub fn to_ymd(mut self, year: i32, month: u32, day: u32) -> Self {
        self.AA_TO_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
}

impl ToQueryString for SchoolScheduleParams {
    fn to_query_string(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        serializer.append_pair("ATPT_OFCDC_SC_CODE", &self.ATPT_OFCDC_SC_CODE);
        serializer.append_pair("SD_SCHUL_CODE", &self.SD_SCHUL_CODE);
        if let Some(s) = &self.DGHT_CRSE_SC_NM {
            serializer.append_pair("DGHT_CRSE_SC_NM", s);
        }
        if let Some(s) = &self.SCHUL_CRSE_SC_NM {
            serializer.append_pair("SCHUL_CRSE_SC_NM", s);
        }
        if let Some(s) = &self.AA_YMD {
            serializer.append_pair("AA_YMD", s);
        }
        if let Some(s) = &self.AA_FROM_YMD {
            serializer.append_pair("AA_FROM_YMD", s);
        }
        if let Some(s) = &self.AA_TO_YMD {
            serializer.append_pair("AA_TO_YMD", s);
        }

        serializer.finish()
    }
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct SchoolScheduleItem {
    /// 시도교육청코드
    /// B10 | C10 | D10 | E10 | F10 | G10 | H10 | I10 | J10 | K10 | M10 | N10 | P10 | Q10 | R10 | S10 | T10 | V10
    pub ATPT_OFCDC_SC_CODE: String,

    /// 시도교육청명
    /// Example: 서울특별시교육청
    pub ATPT_OFCDC_SC_NM: String,

    /// 행정표준코드
    /// Example: 7010959
    pub SD_SCHUL_CODE: String,

    /// 학교명
    /// Example: 문현고등학교
    pub SCHUL_NM: String,

    /// 학년도
    /// Example: 2024
    #[serde(deserialize_with = "deserialize_i32_from_string")]
    pub AY: i32,

    /// 주야과정명
    /// Example: 주간
    pub DGHT_CRSE_SC_NM: Option<String>,

    /// 학교과정명
    /// Example: 고등학교
    pub SCHUL_CRSE_SC_NM: Option<String>,

    /// 수업공제일명
    /// Example: 휴업일
    pub SBTR_DD_SC_NM: Option<String>,

    /// 학사일자
    /// Example: 20240309
    pub AA_YMD: String,

    /// 행사명
    /// Example: 토요휴업일
    pub EVENT_NM: String,

    /// 행사내용
    /// Example:
    pub EVENT_CNTNT: String,

    /// 1학년행사여부
    /// Y | N
    pub ONE_GRADE_EVENT_YN: String,
    /// 2학년행사여부
    /// Y | N
    pub TW_GRADE_EVENT_YN: String,
    /// 3학년행사여부
    /// Y | N
    pub THREE_GRADE_EVENT_YN: String,
    /// 4학년행사여부
    /// Y | N | *
    pub FR_GRADE_EVENT_YN: String,
    /// 5학년행사여부
    /// Y | N | *
    pub FIV_GRADE_EVENT_YN: String,
    /// 6학년행사여부
    /// Y | N | *
    pub SIX_GRADE_EVENT_YN: String,

    /// 수정일자
    /// Example: 20250930
    pub LOAD_DTM: String,
}

impl SchoolScheduleItem {
    pub fn is_event_for_grade(&self, grade: u8) -> bool {
        match grade {
            1 => self.ONE_GRADE_EVENT_YN == "Y",
            2 => self.TW_GRADE_EVENT_YN == "Y",
            3 => self.THREE_GRADE_EVENT_YN == "Y",
            4 => self.FR_GRADE_EVENT_YN == "Y",
            5 => self.FIV_GRADE_EVENT_YN == "Y",
            6 => self.SIX_GRADE_EVENT_YN == "Y",
            _ => false,
        }
    }
}
