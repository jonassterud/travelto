use serde::Deserialize;

/// Response struct for search.
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    /// Itineraries.
    pub itineraries: Itineraries,
    /// Context.
    pub context: Context,
}

/// Itineraries.
#[derive(Debug, Deserialize)]
pub struct Itineraries {
    /// Vector of itinerary buckets.
    pub buckets: Vec<ItineraryBucket>,
}

/// Itinerary bucket.
#[derive(Debug, Deserialize)]
pub struct ItineraryBucket {
    /// Bucket id.
    pub id: String,
    /// Bucket name
    pub name: String,
    /// Bucket items.
    pub items: Vec<BucketItem>,
}

/// Itinerary bucket item.
#[derive(Debug, Deserialize)]
pub struct BucketItem {
    /// Price.
    pub price: ItemPrice,
    /// Legs.
    pub legs: Vec<ItemLeg>,
    /// Booking link.
    pub deeplink: String,
}

/// Itinerary bucket item price.
#[derive(Debug, Deserialize)]
pub struct ItemPrice {
    /// Price value.
    pub raw: u32,
}

/// Itinerary bucket item leg.
#[derive(Debug, Deserialize)]
pub struct ItemLeg {
    /// Origin location.
    pub origin: Location,
    /// Destination location.
    pub destination: Location,
    /// Duration in minutes.
    #[serde(rename = "durationInMinutes")]
    pub duration_mins: u64,
    /// Departure date.
    pub departure: String,
    /// Arrival date.
    pub arrival: String,
}

/// Location.
#[derive(Debug, Deserialize)]
pub struct Location {
    /// Location id.
    pub id: String,
    /// Location name.
    pub name: String,
}

/// Context.
#[derive(Debug, Deserialize)]
pub struct Context {
    /// Status of search - `"incomplete"` or `"complete"`.
    pub status: String,
}
