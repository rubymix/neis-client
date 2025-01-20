#![allow(non_snake_case)]
use super::{deserialize_i32_from_string, deserialize_u8_from_string};
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ClassInfoParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 학년도
    pub AY: Option<String>,
    /// 학년
    pub GRADE: Option<String>,
    /// 주야과정명
    pub DGHT_CRSE_SC_NM: Option<String>,
    /// 학교과정명
    pub SCHUL_CRSE_SC_NM: Option<String>,
    /// 계열명
    pub ORD_SC_NM: Option<String>,
    /// 학과명
    pub DDDEP_NM: Option<String>,
}

impl ClassInfoParams {
    pub fn new(atpt_ofcdc_sc_code: &str, sd_schul_code: &str) -> Self {
        Self {
            ATPT_OFCDC_SC_CODE: atpt_ofcdc_sc_code.to_owned(),
            SD_SCHUL_CODE: sd_schul_code.to_owned(),
            AY: None,
            GRADE: None,
            DGHT_CRSE_SC_NM: None,
            SCHUL_CRSE_SC_NM: None,
            ORD_SC_NM: None,
            DDDEP_NM: None,
        }
    }

    pub fn ay(mut self, year: i32) -> Self {
        self.AY = Some(year.to_string());
        self
    }
    pub fn grade(mut self, grade: u8) -> Self {
        self.GRADE = Some(grade.to_string());
        self
    }

    pub fn to_serializer(&self) -> Serializer<String> {
        let mut serializer = Serializer::new(String::new());

        serializer.append_pair("ATPT_OFCDC_SC_CODE", &self.ATPT_OFCDC_SC_CODE);
        serializer.append_pair("SD_SCHUL_CODE", &self.SD_SCHUL_CODE);
        if let Some(s) = &self.AY {
            serializer.append_pair("AY", s);
        }
        if let Some(s) = &self.GRADE {
            serializer.append_pair("GRADE", s);
        }
        if let Some(s) = &self.DGHT_CRSE_SC_NM {
            serializer.append_pair("DGHT_CRSE_SC_NM", s);
        }
        if let Some(s) = &self.SCHUL_CRSE_SC_NM {
            serializer.append_pair("SCHUL_CRSE_SC_NM", s);
        }
        if let Some(s) = &self.ORD_SC_NM {
            serializer.append_pair("ORD_SC_NM", s);
        }
        if let Some(s) = &self.DDDEP_NM {
            serializer.append_pair("DDDEP_NM", s);
        }

        serializer
    }
}

#[derive(Debug, Deserialize)]
pub struct ClassInfoItem {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 시도교육청명
    pub ATPT_OFCDC_SC_NM: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 학교명
    pub SCHUL_NM: String,
    /// 학년도
    #[serde(deserialize_with = "deserialize_i32_from_string")]
    pub AY: i32,
    /// 학년
    #[serde(deserialize_with = "deserialize_u8_from_string")]
    pub GRADE: u8,
    /// 주야과정명
    pub DGHT_CRSE_SC_NM: Option<String>,
    /// 학교과정명
    pub SCHUL_CRSE_SC_NM: Option<String>,
    /// 계열명
    pub ORD_SC_NM: Option<String>,
    /// 학과명
    pub DDDEP_NM: Option<String>,
    /// 학급명
    pub CLASS_NM: Option<String>,
    /// 수정일자
    pub LOAD_DTM: String,
}
