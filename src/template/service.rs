use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{
    ThreadStatic, ThreadSafe,
    template::{
        self as tpl, Template
    },
    service::RequestFns,
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

type BuildResult<T, E> = rf::Result<rf::Router<T, E>>;

impl<L: ThreadSafe> ThreadSafe for Service<L> {}
impl<L: ThreadStatic> ThreadStatic for Service<L> {}

impl<L: tpl::Lookup + ThreadStatic> Service<L> {
    pub fn router(self) -> BuildResult<hp::Body, tpl::Error> {
        rf::Router::builder()
            .data(Arc::new(Mutex::new(self)))
            .build()
    }

    async fn render_template(req: Request<Body>) -> tpl::Result<Body> {
        let service = req.must_data::<Self>()?;

        Ok(hp::Body::from("render_template"))
    }

    pub fn render_to(&mut self, key: &L::Key, write: impl Write) -> tpl::Result<()> {
        let tpl = self.lookup.get(key)?;

        tpl.render_to(write)?;

        Ok(())
    }
}

