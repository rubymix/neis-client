#![allow(non_snake_case)]
use super::ToQueryString;
use super::{deserialize_i32_from_string, deserialize_u8_from_string};
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct HisTimetableParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 학년도
    pub AY: Option<String>,
    /// 학기
    pub SEM: Option<String>,
    /// 시간표일자
    pub ALL_TI_YMD: Option<String>,
    /// 주야과정명
    pub DGHT_CRSE_SC_NM: Option<String>,
    /// 계열명
    pub ORD_SC_NM: Option<String>,
    /// 학과명
    pub DDDEP_NM: Option<String>,
    /// 학년
    pub GRADE: Option<String>,
    /// 강의실명
    pub CLRM_NM: Option<String>,
    /// 학급명
    pub CLASS_NM: Option<String>,
    /// 시간표시작일자
    pub TI_FROM_YMD: Option<String>,
    /// 시간표종료일자
    pub TI_TO_YMD: Option<String>,
}

impl HisTimetableParams {
    pub fn new(atpt_ofcdc_sc_code: &str, sd_schul_code: &str) -> Self {
        Self {
            ATPT_OFCDC_SC_CODE: atpt_ofcdc_sc_code.to_owned(),
            SD_SCHUL_CODE: sd_schul_code.to_owned(),
            AY: None,
            SEM: None,
            ALL_TI_YMD: None,
            DGHT_CRSE_SC_NM: None,
            ORD_SC_NM: None,
            DDDEP_NM: None,
            GRADE: None,
            CLRM_NM: None,
            CLASS_NM: None,
            TI_FROM_YMD: None,
            TI_TO_YMD: None,
        }
    }

    pub fn ay(mut self, year: i32) -> Self {
        self.AY = Some(year.to_string());
        self
    }
    pub fn sem(mut self, sem: u8) -> Self {
        self.SEM = Some(sem.to_string());
        self
    }
    pub fn ymd(mut self, year: i32, month: u8, day: u8) -> Self {
        self.ALL_TI_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
    pub fn grade(mut self, grade: u8) -> Self {
        self.GRADE = Some(grade.to_string());
        self
    }
    pub fn from_ymd(mut self, year: i32, month: u8, day: u8) -> Self {
        self.TI_FROM_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
    pub fn to_ymd(mut self, year: i32, month: u8, day: u8) -> Self {
        self.TI_TO_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
}

impl ToQueryString for HisTimetableParams {
    fn to_query_string(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        serializer.append_pair("ATPT_OFCDC_SC_CODE", &self.ATPT_OFCDC_SC_CODE);
        serializer.append_pair("SD_SCHUL_CODE", &self.SD_SCHUL_CODE);
        if let Some(s) = &self.AY {
            serializer.append_pair("AY", s);
        }
        if let Some(s) = &self.SEM {
            serializer.append_pair("SEM", s);
        }
        if let Some(s) = &self.ALL_TI_YMD {
            serializer.append_pair("ALL_TI_YMD", s);
        }
        if let Some(s) = &self.DGHT_CRSE_SC_NM {
            serializer.append_pair("DGHT_CRSE_SC_NM", s);
        }
        if let Some(s) = &self.ORD_SC_NM {
            serializer.append_pair("ORD_SC_NM", s);
        }
        if let Some(s) = &self.DDDEP_NM {
            serializer.append_pair("DDDEP_NM", s);
        }
        if let Some(s) = &self.GRADE {
            serializer.append_pair("GRADE", s);
        }
        if let Some(s) = &self.CLRM_NM {
            serializer.append_pair("CLRM_NM", s);
        }
        if let Some(s) = &self.CLASS_NM {
            serializer.append_pair("CLASS_NM", s);
        }
        if let Some(s) = &self.TI_FROM_YMD {
            serializer.append_pair("TI_FROM_YMD", s);
        }
        if let Some(s) = &self.TI_TO_YMD {
            serializer.append_pair("TI_TO_YMD", s);
        }

        serializer.finish()
    }
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct HisTimetableItem {
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
    /// Example: 2023
    #[serde(deserialize_with = "deserialize_i32_from_string")]
    pub AY: i32,

    /// 학기
    /// Example: 2
    #[serde(deserialize_with = "deserialize_u8_from_string")]
    pub SEM: u8,

    /// 시간표일자
    /// Example: 20230821
    pub ALL_TI_YMD: String,

    /// 주야과정명
    /// Example: 주간
    pub DGHT_CRSE_SC_NM: Option<String>,

    /// 계열명
    /// Example: 일반계
    pub ORD_SC_NM: Option<String>,

    /// 학과명
    /// Example: 일반학과
    pub DDDEP_NM: Option<String>,

    /// 학년
    /// Example: 3
    #[serde(deserialize_with = "deserialize_u8_from_string")]
    pub GRADE: u8,

    /// 강의실명
    /// Example: 301
    pub CLRM_NM: Option<String>,

    /// 학급명
    /// Example: 1
    pub CLASS_NM: Option<String>,

    /// 교시
    /// Example: 5
    #[serde(deserialize_with = "deserialize_u8_from_string")]
    pub PERIO: u8,

    /// 수업내용
    /// Example: [보강]음악 감상과 비평
    pub ITRT_CNTNT: Option<String>,

    /// 수정일자
    /// Example: 20230827
    pub LOAD_DTM: String,
}
