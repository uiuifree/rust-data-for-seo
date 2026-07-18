//! Response models for the Google Shopping Merchant API.
//!
//! See <https://docs.dataforseo.com/v3/merchant/google/overview/>.

mod product_info;
mod products;
mod reviews;
mod sellers;

pub use product_info::*;
pub use products::*;
pub use reviews::*;
pub use sellers::*;
