mod response;

use crate::error::Error;
use crate::types::*;
use http_body_util::{BodyExt, Empty};
use hyper::body::{Buf, Bytes};
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use response::ResponseBody;

pub struct NeisClient {
    api_key: String,
    client: Client<HttpsConnector<HttpConnector>, Empty<Bytes>>,
}

impl NeisClient {
    const BASE_URL: &'static str = "https://open.neis.go.kr";

    pub fn new(api_key: &str) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder(TokioExecutor::new()).build(https);

        Self {
            api_key: api_key.to_owned(),
            client,
        }
    }

    /// 학교기본정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17020190531110010104913&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::SchoolInfoParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = SchoolInfoParams {
    ///     SD_SCHUL_CODE: Some(String::from("7031115")),
    ///     ..Default::default()
    /// };
    /// let items = client.school_info(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn school_info(
        &self,
        params: SchoolInfoParams,
    ) -> Result<Vec<SchoolInfoItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/schoolInfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::schoolInfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 학급정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN15320190408174919197546&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::ClassInfoParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = ClassInfoParams::new("B10", "7031115");
    /// let items = client.class_info(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn class_info(&self, params: ClassInfoParams) -> Result<Vec<ClassInfoItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/classInfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::classInfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 학교학과정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN14020190311111456561190&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::SchoolMajorInfoParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = SchoolMajorInfoParams::new("B10").school_code("7031115");
    /// let items = client.school_major_info(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn school_major_info(
        &self,
        params: SchoolMajorInfoParams,
    ) -> Result<Vec<SchoolMajorInfoItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/schoolMajorinfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::schoolMajorinfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 학교계열정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN13920190311110530306647&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::SchoolAflcoInfoParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = SchoolAflcoInfoParams::new("B10").school_code("7031115");
    /// let items = client.school_aflco_info(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn school_aflco_info(
        &self,
        params: SchoolAflcoInfoParams,
    ) -> Result<Vec<SchoolAflcoInfoItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/schulAflcoinfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::schulAflcoinfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 학사일정
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17220190722175038389180&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::SchoolScheduleParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = SchoolScheduleParams::new("B10", "7031115");
    /// let items = client.school_schedule(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn school_schedule(
        &self,
        params: SchoolScheduleParams,
    ) -> Result<Vec<SchoolScheduleItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/SchoolSchedule?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::SchoolSchedule((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 초등학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN15020190408160341416743&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::ElsTimetableParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = ElsTimetableParams::new("B10", "7031115");
    /// let items = client.els_timetable(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn els_timetable(
        &self,
        params: ElsTimetableParams,
    ) -> Result<Vec<ElsTimetableItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/elsTimetable?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::elsTimetable((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 중학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN15120190408165334348844&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::MisTimetableParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = MisTimetableParams::new("D10", "7261011");
    /// let items = client.mis_timetable(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mis_timetable(
        &self,
        params: MisTimetableParams,
    ) -> Result<Vec<MisTimetableItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/misTimetable?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::misTimetable((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 고등학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN18620200826103326268120&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::HisTimetableParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = HisTimetableParams::new("D10", "7240090");
    /// let items = client.his_timetable(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn his_timetable(
        &self,
        params: HisTimetableParams,
    ) -> Result<Vec<HisTimetableItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/hisTimetable?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::hisTimetable((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 특수학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN18520200826093359591792&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::SpsTimetableParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = SpsTimetableParams::new("B10", "7010575");
    /// let items = client.sps_timetable(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sps_timetable(
        &self,
        params: SpsTimetableParams,
    ) -> Result<Vec<SpsTimetableItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/spsTimetable?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::spsTimetable((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 시간표강의실정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=2&rows=10&sortColumn=&sortDirection=&infId=OPEN14120190311112536362172&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::ClassRoomInfoParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = ClassRoomInfoParams::new("B10", "7010575");
    /// let items = client.class_room_info(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn class_room_info(
        &self,
        params: ClassRoomInfoParams,
    ) -> Result<Vec<ClassRoomInfoItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/tiClrminfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::tiClrminfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 학원교습소정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN19220231012134453534385&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::AcademyInfoParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = AcademyInfoParams::new("D10");
    /// let items = client.academy_info(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn academy_info(
        &self,
        params: AcademyInfoParams,
    ) -> Result<Vec<AcademyInfoItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/acaInsTiInfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::acaInsTiInfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }

    /// 급식식단정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17320190722180924242823&infSeq=2
    ///
    /// # Example
    ///
    /// ```rust
    /// use neis_client::{types::MealServiceParams, Error, NeisClient};
    ///
    /// # async fn foo() -> Result<(), Error> {
    /// let api_key = std::env::var("NEIS_API_KEY").unwrap();
    /// let client = NeisClient::new(&api_key);
    ///
    /// let params = MealServiceParams::new("B10", "7031115").from_ymd(2025, 1, 1);
    /// let items = client.meal_service(params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn meal_service(
        &self,
        params: MealServiceParams,
    ) -> Result<Vec<MealServiceItem>, Error> {
        let mut page = 1;
        let page_size = 1000;
        let mut items = Vec::new();

        loop {
            let query = {
                let mut query = params.to_serializer();
                query.append_pair("KEY", &self.api_key);
                query.append_pair("Type", "json");
                query.append_pair("pSize", &page_size.to_string());
                query.append_pair("pIndex", &page.to_string());
                query.finish()
            };
            let url = format!("{}/hub/mealServiceDietInfo?{}", Self::BASE_URL, query);

            let res = self.client.get(url.try_into().unwrap()).await?;
            let status = res.status();
            let body = res.collect().await?.to_bytes();
            tracing::trace!(?body);

            if status.is_success() {
                let data: ResponseBody = serde_json::from_reader(body.reader())?;

                if let ResponseBody::mealServiceDietInfo((head, body)) = data {
                    items.extend(body.row);

                    if head.get_total_count() > page * page_size {
                        page += 1;
                    } else {
                        break; // while loop
                    }
                } else {
                    break; // while loop
                }
            } else {
                return Err(Error::new_unknown(&format!("status: {}", status)));
            }
        }

        Ok(items)
    }
}
