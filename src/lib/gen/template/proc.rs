use crate::{
    gen::{gen::Gen, lang::stub_impl},
    lib::{
        context::Context,
        def::{Def, Int, Obj, Str},
        desc::Desc,
        ext::Ext,
    },
};
use handlebars::{
    Handlebars, Helper, HelperDef, JsonRender, RenderContext, RenderError, ScopedJson,
};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub(crate) struct Parents;

impl HelperDef for Parents {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let obj: Obj = serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
        let mut refs = obj.mix;
        obj.ext.iter().for_each(|ext| {
            refs.insert(0, ext.clone().r#ref);
        });
        Ok(serde_json::to_value(refs).unwrap().into())
    }
}

#[derive(Clone)]
pub(crate) struct Resolve {
    pub context: Context,
}

impl HelperDef for Resolve {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        if let Ok(Desc::Ref(r#ref)) = serde_json::from_value(h.param(0).unwrap().value().clone()) {
            let mut value: Value = serde_json::to_value(self.context.resolve(&r#ref)).unwrap();
            value
                .as_object_mut()
                .unwrap()
                .insert("origin".to_string(), serde_json::to_value(r#ref).unwrap());
            Ok(value.into())
        } else {
            Ok(ScopedJson::Missing)
        }
    }
}

#[derive(Clone)]
pub(crate) struct ResolveIfMappedType {
    pub type_mapping: HashMap<String, String>,
}

impl HelperDef for ResolveIfMappedType {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _hb: &'reg Handlebars<'reg>,
        _c: &'rc handlebars::Context,
        _r: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let dto: Option<String> =
            serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
        Ok(dto
            .and_then(|d| {
                self.type_mapping
                    .get(&d)
                    .map(|mapped| ScopedJson::from(serde_json::to_value(mapped).unwrap()))
            })
            .unwrap_or(ScopedJson::from(serde_json::Value::Null)))
    }
}

#[derive(Clone)]
pub(crate) struct ResolveIfRef {
    pub resolve: Resolve,
}

impl HelperDef for ResolveIfRef {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        hb: &'reg Handlebars<'reg>,
        c: &'rc handlebars::Context,
        r: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        Ok(self
            .resolve
            .call_inner(h, hb, c, r)
            .unwrap_or_else(|_| h.param(0).unwrap().value().clone().into()))
    }
}

#[derive(Clone)]
pub(crate) struct StubImpl {
    pub gen: Box<dyn Gen>,
    pub context: Context,
}

impl HelperDef for StubImpl {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let desc: Desc = serde_json::from_value(h.param(0).unwrap().value().clone())
            .unwrap_or_else(|_| {
                let r#type: String =
                    serde_json::from_value(h.param(0).unwrap().value().clone()).unwrap();
                Desc::Def(match r#type.as_str() {
                    "bool" => Def::Bool(Default::default()),
                    "dec" => Def::Dec(Default::default()),
                    "int" => Def::Int(Default::default()),
                    "str" => Def::Str(Default::default()),
                    _ => unreachable!(),
                })
            });
        Ok(
            serde_json::to_value(stub_impl(&self.gen.lang(), &desc, &self.context))
                .unwrap()
                .into(),
        )
    }
}

#[derive(Clone)]
pub(crate) struct TypeArgs;

impl HelperDef for TypeArgs {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let ext: Result<Ext, _> = serde_json::from_value(h.param(0).unwrap().value().clone());
        ext.map(|e| {
            let mut vec: Vec<_> = e.args.into_iter().collect();
            vec.sort_by(|(name0, _), (name1, _)| name0.cmp(name1));
            Ok(
                serde_json::to_value(vec.iter().map(|(_, desc)| desc).collect::<Vec<_>>())
                    .unwrap()
                    .into(),
            )
        })
        .unwrap_or(Ok(Value::Array(Vec::new()).into()))
    }
}

#[derive(Clone)]
pub(crate) struct TypeParams;

impl HelperDef for TypeParams {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let obj: Result<Obj, _> = serde_json::from_value(h.param(0).unwrap().value().clone());
        obj.map(|o| {
            let mut vec: Vec<_> = o
                .vars
                .iter()
                .flat_map(|(_, var)| var.desc.param().map(str::to_string))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            vec.sort();
            Ok(serde_json::to_value(vec).unwrap().into())
        })
        .unwrap_or(Ok(Value::Array(Vec::new()).into()))
    }
}

#[derive(Clone)]
pub(crate) struct ValueDef {}

impl HelperDef for ValueDef {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let val = h.param(0).unwrap().value().render();
        Ok(Value::from(
            serde_json::to_value(match val.parse::<i64>() {
                Ok(_) => Def::Int(Int { null: false }),
                _ => Def::Str(Str { null: false }),
            })
            .unwrap(),
        )
        .into())
    }
}
