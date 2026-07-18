use crate::DataForSeoClient;

mod amazon;
mod google;

pub use amazon::*;
pub use google::*;

/// Entry point for the Merchant API domain.
/// See <https://docs.dataforseo.com/v3/merchant/>.
pub struct MerchantApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Access the Merchant API (Amazon and Google Shopping).
    /// See <https://docs.dataforseo.com/v3/merchant/>.
    pub fn merchant(&self) -> MerchantApi<'_> {
        MerchantApi { client: self }
    }
}
