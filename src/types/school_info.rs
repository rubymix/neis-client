#![allow(non_snake_case)]
use super::{ToQueryString, YesOrNo};
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
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
    pub fn school_code(school_code: &str) -> Self {
        Self {
            SD_SCHUL_CODE: Some(school_code.to_owned()),
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

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct SchoolInfoItem {
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

    /// 영문학교명
    /// Example: MUNHYEON HIGH SCHOOL
    pub ENG_SCHUL_NM: Option<String>,

    /// 학교종류명
    /// Example: 고등학교
    pub SCHUL_KND_SC_NM: Option<String>, // 초등학교, 중학교, 고등학교, ...

    /// 시도명
    /// Example: 서울특별시
    pub LCTN_SC_NM: String,

    /// 관할조직명
    /// Example: 서울특별시교육청
    pub JU_ORG_NM: String,

    /// 설립명
    /// Example: 공립
    pub FOND_SC_NM: Option<String>,

    /// 도로명우편번호
    /// Example: 05811
    pub ORG_RDNZC: Option<String>,

    /// 도로명주소
    /// Example: 서울특별시 송파구 충민로 115
    pub ORG_RDNMA: Option<String>,

    /// 도로명상세주소
    /// Example: (장지동)
    pub ORG_RDNDA: Option<String>,

    /// 전화번호
    /// Example: 02-6951-8702
    pub ORG_TELNO: Option<String>,

    /// 홈페이지주소
    /// Example: http://munhyeon.sen.hs.kr
    pub HMPG_ADRES: Option<String>,

    /// 남녀공학구분명
    /// 남 | 여 | 남여공학
    pub COEDU_SC_NM: String,

    /// 팩스번호
    /// Example: 02-3012-0880
    pub ORG_FAXNO: Option<String>,

    /// 고등학교구분명
    /// Example: 일반고
    pub HS_SC_NM: Option<String>,

    /// 산업체특별학급존재여부
    pub INDST_SPECL_CCCCL_EXST_YN: YesOrNo,

    /// 고등학교일반전문구분명
    /// Example: 일반계
    pub HS_GNRL_BUSNS_SC_NM: Option<String>,

    /// 특수목적고등학교계열명
    pub SPCLY_PURPS_HS_ORD_NM: Option<String>,

    /// 입시전후기구분명
    /// 전기 | 후기 | 전후기
    pub ENE_BFE_SEHF_SC_NM: String,

    /// 주야구분명
    /// 주간 | 야간 | 주야간
    pub DGHT_SC_NM: String,

    /// 설립일자
    /// Example: 20100301
    pub FOND_YMD: String,

    /// 개교기념일
    /// Example: 20100301
    pub FOAS_MEMRD: String,

    /// 수정일자
    /// Example: 20230627
    pub LOAD_DTM: String,
}
