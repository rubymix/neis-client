#![allow(non_snake_case)]
use super::ToQueryString;
use super::deserialize_i32_from_f64;
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct MealServiceParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 식사코드
    pub MMEAL_SC_CODE: Option<String>,
    /// 급식일자
    pub MLSV_YMD: Option<String>,
    /// 급식시작일자
    pub MLSV_FROM_YMD: Option<String>,
    /// 급식종료일자
    pub MLSV_TO_YMD: Option<String>,
}

impl MealServiceParams {
    pub fn new(atpt_ofcdc_sc_code: &str, sd_schul_code: &str) -> Self {
        Self {
            ATPT_OFCDC_SC_CODE: atpt_ofcdc_sc_code.to_owned(),
            SD_SCHUL_CODE: sd_schul_code.to_owned(),
            MMEAL_SC_CODE: None,
            MLSV_YMD: None,
            MLSV_FROM_YMD: None,
            MLSV_TO_YMD: None,
        }
    }

    pub fn ymd(mut self, year: i32, month: u8, day: u8) -> Self {
        self.MLSV_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
    pub fn from_ymd(mut self, year: i32, month: u8, day: u8) -> Self {
        self.MLSV_FROM_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
    pub fn to_ymd(mut self, year: i32, month: u8, day: u8) -> Self {
        self.MLSV_TO_YMD = Some(format!("{:04}{:02}{:02}", year, month, day));
        self
    }
}

impl ToQueryString for MealServiceParams {
    fn to_query_string(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        serializer.append_pair("ATPT_OFCDC_SC_CODE", &self.ATPT_OFCDC_SC_CODE);
        serializer.append_pair("SD_SCHUL_CODE", &self.SD_SCHUL_CODE);
        if let Some(s) = &self.MMEAL_SC_CODE {
            serializer.append_pair("MMEAL_SC_CODE", s);
        }
        if let Some(s) = &self.MLSV_YMD {
            serializer.append_pair("MLSV_YMD", s);
        }
        if let Some(s) = &self.MLSV_FROM_YMD {
            serializer.append_pair("MLSV_FROM_YMD", s);
        }
        if let Some(s) = &self.MLSV_TO_YMD {
            serializer.append_pair("MLSV_TO_YMD", s);
        }

        serializer.finish()
    }
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct MealServiceItem {
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

    /// 식사코드
    /// Example: 2
    pub MMEAL_SC_CODE: String,

    /// 식사명
    /// Example: 중식
    pub MMEAL_SC_NM: String,

    /// 급식일자
    /// Example: 20210104
    pub MLSV_YMD: String,

    /// 급식인원수
    /// Example: 498
    #[serde(deserialize_with = "deserialize_i32_from_f64")]
    pub MLSV_FGR: i32,

    /// 요리명
    pub DDISH_NM: String,

    /// 원산지정보
    pub ORPLC_INFO: String,

    /// 칼로리정보
    pub CAL_INFO: Option<String>,

    /// 영양정보
    pub NTR_INFO: Option<String>,

    /// 급식시작일자
    /// Example: 20210104
    pub MLSV_FROM_YMD: String,

    /// 급식종료일자
    /// Example: 20210104
    pub MLSV_TO_YMD: String,

    /// 수정일자
    /// Example: 20210111043017
    pub LOAD_DTM: String,
}
