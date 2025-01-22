mod response;

use crate::types::*;
use reqwest::Client;
use response::ResponseBody;

pub struct NeisClient {
    api_key: String,
    client: Client,
}

impl NeisClient {
    const BASE_URL: &'static str = "https://open.neis.go.kr";

    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            client: Client::builder()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .unwrap(),
        }
    }

    /// 학교기본정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17020190531110010104913&infSeq=2
    pub async fn school_info(
        &self,
        params: SchoolInfoParams,
    ) -> Result<Vec<SchoolInfoItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 학급정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN15320190408174919197546&infSeq=2
    pub async fn class_info(
        &self,
        params: ClassInfoParams,
    ) -> Result<Vec<ClassInfoItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 학교학과정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN14020190311111456561190&infSeq=2
    pub async fn school_major_info(
        &self,
        params: SchoolMajorInfoParams,
    ) -> Result<Vec<SchoolMajorInfoItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 학교계열정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN13920190311110530306647&infSeq=2
    pub async fn school_aflco_info(
        &self,
        params: SchoolAflcoInfoParams,
    ) -> Result<Vec<SchoolAflcoInfoItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 학사일정
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17220190722175038389180&infSeq=2
    pub async fn school_schedule(
        &self,
        params: SchoolScheduleParams,
    ) -> Result<Vec<SchoolScheduleItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 초등학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN15020190408160341416743&infSeq=2
    pub async fn els_timetable(
        &self,
        params: ElsTimetableParams,
    ) -> Result<Vec<ElsTimetableItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 중학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN15120190408165334348844&infSeq=2
    pub async fn mis_timetable(
        &self,
        params: MisTimetableParams,
    ) -> Result<Vec<MisTimetableItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 고등학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN18620200826103326268120&infSeq=2
    pub async fn his_timetable(
        &self,
        params: HisTimetableParams,
    ) -> Result<Vec<HisTimetableItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 특수학교시간표
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN18520200826093359591792&infSeq=2
    pub async fn sps_timetable(
        &self,
        params: SpsTimetableParams,
    ) -> Result<Vec<SpsTimetableItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 시간표강의실정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=2&rows=10&sortColumn=&sortDirection=&infId=OPEN14120190311112536362172&infSeq=2
    pub async fn class_room_info(
        &self,
        params: ClassRoomInfoParams,
    ) -> Result<Vec<ClassRoomInfoItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 학원교습소정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN19220231012134453534385&infSeq=2
    pub async fn academy_info(
        &self,
        params: AcademyInfoParams,
    ) -> Result<Vec<AcademyInfoItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }

    /// 급식식단정보
    /// https://open.neis.go.kr/portal/data/service/selectServicePage.do?page=1&rows=10&sortColumn=&sortDirection=&infId=OPEN17320190722180924242823&infSeq=2
    pub async fn meal_service(
        &self,
        params: MealServiceParams,
    ) -> Result<Vec<MealServiceItem>, reqwest::Error> {
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

            let res = self.client.get(&url).send().await?;
            let data: ResponseBody = res.json().await?;

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
        }

        Ok(items)
    }
}
