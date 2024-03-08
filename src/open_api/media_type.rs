use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::desc::Desc;
use crate::open_api::ref_or::RefOr;
use crate::open_api::schema::Schema;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MediaType {
    pub schema: RefOr<Schema>
}

impl MediaType {
    pub fn of(desc: Desc, context: &Context) -> MediaType {
        MediaType {
            schema: RefOr::Item(Schema::of_desc(&desc, "MediaType".to_string(), None, context))
        }
    }
}