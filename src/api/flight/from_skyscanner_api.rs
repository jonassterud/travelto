use super::common::*;
use anyhow::{anyhow, Result};

impl TryFrom<crate::skyscanner_api::SearchResponse> for Vec<super::Flight> {
    type Error = anyhow::Error;

    fn try_from(val: crate::skyscanner_api::SearchResponse) -> Result<Self> {
        val.itineraries
            .ok_or_else(|| anyhow!("missing itineraries"))?
            .buckets
            .iter()
            .flat_map(|x| &x.items)
            .map(|x| -> Result<super::Flight> {
                // TODO: Assumes first leg is departure
                let departure_leg = x.legs.get(0).ok_or_else(|| anyhow!("missing leg"))?;
                // TODO: Assumes second leg is return
                let return_leg = x.legs.get(1);

                Ok(super::Flight {
                    from: departure_leg.origin.name.to_owned(),
                    to: departure_leg.destination.name.to_owned(),
                    departure_date: format_date(&departure_leg.departure)?,
                    departure_arrival_date: format_date(&departure_leg.arrival)?,
                    departure_duration: format_duration(departure_leg.duration_mins * 60),
                    return_date: return_leg.map(|l| format_date(&l.departure)).transpose()?,
                    return_arrival_date: return_leg.map(|l| format_date(&l.arrival)).transpose()?,
                    return_duration: return_leg.map(|l| format_duration(&l.duration_mins * 60)),
                    price: x.price.raw,
                    link: x.deeplink.to_owned(),
                })
            })
            .collect()
    }
}
