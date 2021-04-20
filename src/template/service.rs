use std::{
    io::{
        self as io, Write
    },
    fs,
};

use crate::{
    template::{
        self as tpl, Template, Result
    },
    service::{RequestFns, path},
    thread,
};

use routerify::{
    self as rf,
};

use hyper::{
    self as hp,
    Body, Request
};

pub struct Service<L> {
    lookup: L,
}

impl<L> Service<L> {
    pub fn new(lookup: L) -> Self {
        Self {
            lookup: lookup,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Config {
    context: Option<path::PathOrData>,
    output: std::path::PathBuf,
}

type BuildResult<T, E> = rf::Result<rf::Router<T, E>>;

impl<L: thread::Safe> thread::Safe for Service<L> {}

/*
impl<L: tpl::Lookup + thread::safe::Static> Service<L> {
    pub fn router(self) -> BuildResult<hp::Body, tpl::Error> {
        rf::Router::builder()
            //.data(Arc::new(Mutex::new(self)))
            .data(self)
            .post("/render", Self::render_template)
            .build()
    }

    async fn render_template(req: Request<Body>) -> Result<hp::Response<Body>> {
        let service = req.must_data::<Self>()?;
        let name = req.must_param("name")?;
        let (_, body) = req.into_parts();
        let config: Config = serde_json::from_slice(&body)?;

        let tpl = config.context.map(
            |ctx| service.lookup.with_context(name, ctx)
        ).unwrap_or_else(
            || service.lookup.get(name)?
        );

        let writer = io::BufWriter::new(fs::File::create(config.output)?);
        tpl.render_to(writer)?;

        Ok(hp::Response::new(hp::Body::from(config.output.to_string_lossy())))
    }

    pub fn render_to(&mut self, key: &L::Key, write: impl Write) -> Result<()> {
        let tpl = self.lookup.get(key)?;

        tpl.render_to(write)?;

        Ok(())
    }
}
*/

