use std::collections::HashMap;

use def::DEFS;
use serde::{Deserialize, Serialize};
use typetag::serde;

use crate::lib::context::Context;
use crate::lib::def;
use crate::lib::open_api::components::Components;
use crate::lib::open_api::context::Context as OpenApiContext;
use crate::lib::open_api::path::Path;
use crate::lib::open_api::ref_or::RefOr;
use crate::lib::open_api::schema::schemas_path;
use crate::lib::pkg::Pkg;
use crate::lib::r#ref::Ref;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OpenApi {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub paths: HashMap<String, RefOr<Path>>,
    #[serde(skip_serializing_if = "Components::is_empty")]
    #[serde(default)]
    pub components: Components,
}

impl OpenApi {
    pub fn of(pkg: Pkg, context: &Context) -> OpenApi {
        OpenApi {
            paths: pkg
                .ops
                .iter()
                .map(|(id, ops)| (id.clone(), RefOr::Item(Path::of(ops, context))))
                .collect(),
            components: Components::of(pkg.defs, context),
        }
    }

    pub fn pkg(&self, context: &OpenApiContext) -> Pkg {
        let mut with_mapped_all_of = self
            .components
            .schemas
            .iter()
            .filter(|(_, schema)| !schema.all_of.is_empty())
            .map(|(name, schema)| {
                let name_clone = name.clone();
                (name_clone, schema.with_mapped_all_of())
            })
            .collect::<Vec<_>>();

        let mut other = self
            .components
            .schemas
            .iter()
            .filter(|(_, schema)| schema.all_of.is_empty())
            .map(|(name, schema)| (name.clone(), schema.clone()))
            .collect::<Vec<_>>();
        other.append(&mut with_mapped_all_of);

        Pkg {
            ops: self
                .paths
                .iter()
                .flat_map(|(id, path)| path.clone().item().map(|p| (id, p)))
                .map(|(id, path)| (id.clone(), path.ops(context)))
                .filter(|(_, ops)| !ops.is_empty())
                .collect(),
            defs: other
                .iter()
                .map(|(name, schema)| (name.clone().clone(), schema.def(name.clone(), context)))
                .collect::<HashMap<_, _>>(),
        }
    }

    pub fn trust_ref(r#ref: String) -> Ref {
        let (src, path) = OpenApiContext::src_and_path(r#ref);
        let defs = DEFS;
        Ref {
            src,
            path: path.replace((schemas_path() + "/").as_str(), format!("{defs}.").as_str()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    pub(crate) property_name: String,
    pub(crate) mapping: HashMap<String, String>,
}
