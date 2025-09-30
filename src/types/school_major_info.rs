#![allow(non_snake_case)]
use super::ToQueryString;
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SchoolMajorInfoParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: Option<String>,
    /// 주야과정명
    pub DGHT_CRSE_SC_NM: Option<String>,
    /// 계열명
    pub ORD_SC_NM: Option<String>,
}

impl SchoolMajorInfoParams {
    pub fn new(atpt_ofcdc_sc_code: &str) -> Self {
        Self {
            ATPT_OFCDC_SC_CODE: atpt_ofcdc_sc_code.to_owned(),
            SD_SCHUL_CODE: None,
            DGHT_CRSE_SC_NM: None,
            ORD_SC_NM: None,
        }
    }

    pub fn school_code(mut self, s: &str) -> Self {
        self.SD_SCHUL_CODE = Some(s.to_owned());
        self
    }
}

impl ToQueryString for SchoolMajorInfoParams {
    fn to_query_string(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        serializer.append_pair("ATPT_OFCDC_SC_CODE", &self.ATPT_OFCDC_SC_CODE);
        if let Some(s) = &self.SD_SCHUL_CODE {
            serializer.append_pair("SD_SCHUL_CODE", s);
        }
        if let Some(s) = &self.DGHT_CRSE_SC_NM {
            serializer.append_pair("DGHT_CRSE_SC_NM", s);
        }
        if let Some(s) = &self.ORD_SC_NM {
            serializer.append_pair("ORD_SC_NM", s);
        }

        serializer.finish()
    }
}

#[derive(Debug, Deserialize)]
pub struct SchoolMajorInfoItem {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 시도교육청명
    pub ATPT_OFCDC_SC_NM: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 학교명
    pub SCHUL_NM: String,
    /// 주야과정명
    pub DGHT_CRSE_SC_NM: Option<String>,
    /// 계열명
    pub ORD_SC_NM: Option<String>,
    /// 학과명
    pub DDDEP_NM: Option<String>,
    /// 수정일자
    pub LOAD_DTM: String,
}
