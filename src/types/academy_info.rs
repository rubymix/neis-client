#![allow(non_snake_case)]
use super::{ToQueryString, YesOrNo};
use form_urlencoded::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct AcademyInfoParams {
    /// 시도교육청코드
    pub ATPT_OFCDC_SC_CODE: String,
    /// 행정구역명
    pub ADMST_ZONE_NM: Option<String>,
    /// 학원지정번호
    pub ACA_ASNUM: Option<String>,
    /// 학원명
    pub ACA_NM: Option<String>,
    /// 분야명
    pub REALM_SC_NM: Option<String>,
    /// 교습계열명
    pub LE_ORD_NM: Option<String>,
    /// 교습과정명
    pub LE_CRSE_NM: Option<String>,
}

impl AcademyInfoParams {
    pub fn new(atpt_ofcdc_sc_code: &str) -> Self {
        Self {
            ATPT_OFCDC_SC_CODE: atpt_ofcdc_sc_code.to_owned(),
            ADMST_ZONE_NM: None,
            ACA_ASNUM: None,
            ACA_NM: None,
            REALM_SC_NM: None,
            LE_ORD_NM: None,
            LE_CRSE_NM: None,
        }
    }
}

impl ToQueryString for AcademyInfoParams {
    fn to_query_string(&self) -> String {
        let mut serializer = Serializer::new(String::new());

        serializer.append_pair("ATPT_OFCDC_SC_CODE", &self.ATPT_OFCDC_SC_CODE);
        if let Some(s) = &self.ADMST_ZONE_NM {
            serializer.append_pair("ADMST_ZONE_NM", s);
        }
        if let Some(s) = &self.ACA_ASNUM {
            serializer.append_pair("ACA_ASNUM", s);
        }
        if let Some(s) = &self.ACA_NM {
            serializer.append_pair("ACA_NM", s);
        }
        if let Some(s) = &self.REALM_SC_NM {
            serializer.append_pair("REALM_SC_NM", s);
        }
        if let Some(s) = &self.LE_ORD_NM {
            serializer.append_pair("LE_ORD_NM", s);
        }
        if let Some(s) = &self.LE_CRSE_NM {
            serializer.append_pair("LE_CRSE_NM", s);
        }

        serializer.finish()
    }
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct AcademyInfoItem {
    /// 시도교육청코드
    /// B10 | C10 | D10 | E10 | F10 | G10 | H10 | I10 | J10 | K10 | M10 | N10 | P10 | Q10 | R10 | S10 | T10 | V10
    pub ATPT_OFCDC_SC_CODE: String,

    /// 시도교육청명
    /// Example: 서울특별시교육청
    pub ATPT_OFCDC_SC_NM: String,

    /// 행정구역명
    /// Example: 강남구
    pub ADMST_ZONE_NM: Option<String>,

    /// 학원교습소명
    /// Example: 교습소
    pub ACA_INSTI_SC_NM: String,

    /// 학원지정번호
    /// Example: 3000024877
    pub ACA_ASNUM: String,

    /// 학원명
    /// Example: 9087스튜디오미술교습소
    pub ACA_NM: String,

    /// 개설일자
    /// Example: 20151208
    pub ESTBL_YMD: String,

    /// 등록일자
    /// Example: 20151208
    pub REG_YMD: String,

    /// 등록상태명
    /// Example: 개원
    pub REG_STTUS_NM: String,

    /// 휴원시작일자
    /// Example:
    pub CAA_BEGIN_YMD: String,

    /// 휴원종료일자
    /// Example: 99991231
    pub CAA_END_YMD: String,

    /// 정원합계
    /// Example: 4
    pub TOFOR_SMTOT: i64,

    /// 일시수용능력인원합계
    /// Example: 9
    pub DTM_RCPTN_ABLTY_NMPR_SMTOT: i64,

    /// 분야명
    /// Example: 예능(대)
    pub REALM_SC_NM: Option<String>,

    /// 교습계열명
    /// Example: 예능(중)
    pub LE_ORD_NM: Option<String>,

    /// 교습과정목록명
    /// Example: 드로잉
    pub LE_CRSE_LIST_NM: Option<String>,

    /// 교습과정명
    /// Example: 미술
    pub LE_CRSE_NM: Option<String>,

    /// 인당수강료
    /// Example:
    pub PSNBY_THCC_CNTNT: String,

    /// 수강료공개여부
    pub THCC_OTHBC_YN: YesOrNo,

    /// 기숙사학원여부
    /// Example: N
    pub BRHS_ACA_YN: String,

    /// 도로명주소
    /// Example: 서울특별시 서초구 바우뫼로20길 25
    pub FA_RDNMA: String,

    /// 도로명상세주소
    /// Example: , 201호 (양재동, 서두빌딩)
    pub FA_RDNDA: String,

    /// 도로명우편번호
    /// Example: 06755
    pub FA_RDNZC: String,

    /// 전화번호
    pub FA_TELNO: Option<String>,

    /// 수정일자
    /// Example: 20231018
    pub LOAD_DTM: String,
}
