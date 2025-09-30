#![allow(non_snake_case)]
use super::{ToQueryString, YesOrNo};
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct SchoolInfoParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: Option<String>,
    /// 행정표준코드
    pub SD_SCHUL_CODE: Option<String>,
    /// 학교명
    pub SCHUL_NM: Option<String>,
    /// 학교종류명
    pub SCHUL_KND_SC_NM: Option<String>,
    /// 시도명
    pub LCTN_SC_NM: Option<String>,
    /// 설립명
    pub FOND_SC_NM: Option<String>,
}

impl SchoolInfoParams {
    pub fn sd_schul_code(sd_schul_code: &str) -> Self {
        Self {
            SD_SCHUL_CODE: Some(sd_schul_code.to_owned()),
            ..Default::default()
        }
    }
}

impl ToQueryString for SchoolInfoParams {
    fn to_query_string(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        if let Some(s) = &self.ATPT_OFCDC_SC_CODE {
            serializer.append_pair("ATPT_OFCDC_SC_CODE", s);
        }
        if let Some(s) = &self.SD_SCHUL_CODE {
            serializer.append_pair("SD_SCHUL_CODE", s);
        }
        if let Some(s) = &self.SCHUL_NM {
            serializer.append_pair("SCHUL_NM", s);
        }
        if let Some(s) = &self.SCHUL_KND_SC_NM {
            serializer.append_pair("SCHUL_KND_SC_NM", s);
        }
        if let Some(s) = &self.LCTN_SC_NM {
            serializer.append_pair("LCTN_SC_NM", s);
        }
        if let Some(s) = &self.FOND_SC_NM {
            serializer.append_pair("FOND_SC_NM", s);
        }

        serializer.finish()
    }
}

#[derive(Debug, Deserialize)]
pub struct SchoolInfoItem {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String, // B10 | C10 | D10 | E10 | F10 | G10 | H10 | I10 | J10 | K10 | M10 | N10 | P10 | Q10 | R10 | S10 | T10 | V10
    /// 시도교육청명
    pub ATPT_OFCDC_SC_NM: String,
    /// 행정표준코드
    pub SD_SCHUL_CODE: String,
    /// 학교명
    pub SCHUL_NM: String,
    /// 영문학교명
    pub ENG_SCHUL_NM: Option<String>,
    /// 학교종류명
    pub SCHUL_KND_SC_NM: Option<String>, // 초등학교, 중학교, 고등학교, ...
    /// 시도명
    pub LCTN_SC_NM: String,
    /// 관할조직명
    pub JU_ORG_NM: String,
    /// 설립명
    pub FOND_SC_NM: Option<String>,
    /// 도로명우편번호
    pub ORG_RDNZC: Option<String>,
    /// 도로명주소
    pub ORG_RDNMA: Option<String>,
    /// 도로명상세주소
    pub ORG_RDNDA: Option<String>,
    /// 전화번호
    pub ORG_TELNO: Option<String>,
    /// 홈페이지주소
    pub HMPG_ADRES: Option<String>,
    /// 남녀공학구분명
    pub COEDU_SC_NM: String, // 남 | 여 | 남여공학
    /// 팩스번호
    pub ORG_FAXNO: Option<String>,
    /// 고등학교구분명
    pub HS_SC_NM: Option<String>,
    /// 산업체특별학급존재여부
    pub INDST_SPECL_CCCCL_EXST_YN: YesOrNo,
    /// 고등학교일반전문구분명
    pub HS_GNRL_BUSNS_SC_NM: Option<String>,
    /// 특수목적고등학교계열명
    pub SPCLY_PURPS_HS_ORD_NM: Option<String>,
    /// 입시전후기구분명
    pub ENE_BFE_SEHF_SC_NM: String, // 전기 | 후기 | 전후기
    /// 주야구분명
    pub DGHT_SC_NM: String, // 주간 | 야간 | 주야간
    /// 설립일자
    pub FOND_YMD: String,
    /// 개교기념일
    pub FOAS_MEMRD: String,
    /// 수정일자
    pub LOAD_DTM: String,
}
