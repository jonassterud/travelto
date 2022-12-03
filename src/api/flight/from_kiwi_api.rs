use super::common::*;
use anyhow::Result;

impl TryFrom<crate::kiwi_api::SearchResponse> for Vec<super::Flight> {
    type Error = anyhow::Error;

    fn try_from(val: crate::kiwi_api::SearchResponse) -> Result<Self> {
        val.data
            .iter()
            .map(|x| -> Result<super::Flight> {
                Ok(super::Flight {
                    from: format!("{}, {}", x.city_from, x.fly_from),
                    to: format!("{}, {}", x.city_to, x.fly_to),
                    departure_date: format_date(&x.local_departure)?,
                    departure_arrival_date: format_date(&x.local_arrival)?,
                    departure_duration: format_duration(x.duration.departure_secs),
                    return_date: x
                        .route
                        .iter()
                        .find(|r| r.is_return > 0)
                        .map(|r| format_date(&r.local_departure))
                        .transpose()?,
                    return_arrival_date: x
                        .route
                        .iter()
                        .rfind(|r| r.is_return > 0)
                        .map(|r| format_date(&r.local_arrival))
                        .transpose()?, // assumes last route in vector is final
                    return_duration: Some(format_duration(x.duration.return_secs)),
                    price: x.price,
                    link: x.deep_link.to_owned(),
                })
            })
            .collect()
    }
}
