use std::{
    io,
    fs,
    convert::TryInto,
};

use crate::{
    template::{Engine, Result, Error}, 
    service::{RequestFns, path},
    thread,
};

use routerify::{
    self as rf,
};

use hyper::{
    Body, Request, Response, body::{HttpBody, Buf}
};

pub struct Service<E> {
    engine: E,
}

impl<E> Service<E> {
    pub fn new(engine: E) -> Self {
        Self {
            engine: engine,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Config {
    context: Option<path::PathOrData>,
    output: std::path::PathBuf,
}

type BuildResult<T, E> = rf::Result<rf::Router<T, E>>;

impl<E: thread::Safe> thread::Safe for Service<E> {}


impl<E> Service<E>
where
    E: Engine<Key = String> + thread::safe::Static,
{
    pub fn router(self) -> BuildResult<Body, Error> {
        rf::Router::builder()
            //.data(Arc::new(Mutex::new(self)))
            .data(self)
            .post("/render/:key", Self::render_template)
            .build()
    }


    async fn render_template(mut req: Request<Body>) -> Result<Response<Body>> {
        let config: Config = serde_json::from_reader(
            req.body_mut().data().await.unwrap().unwrap().reader()
        )?;

        let ctx = if let Some(context) = config.context {
            let reader: path::Reader = context.try_into()?;
            Some(serde_json::from_reader(reader)?)
        } else {
            None
        };

        let writer = io::BufWriter::new(fs::File::create(config.output)?);

        let service = req.must_data::<Self>()?;
        let key = req.must_param("key")?;
        service.engine.render_to(ctx, key, writer).map_err(|e| rf::Error::new(e.to_string()))?;

        Ok(Response::new(Body::from("OK")))
    }
}
