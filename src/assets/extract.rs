/*
use std::{
    path::Path,
    convert::TryFrom,
};

use crate::assets as ast;

pub struct Extractable<P> {
    path: P,
}

pub struct PathRef<AR: AsRef<Path>>(AR);

impl<P: AsRef<Path>> From<&P> for ast::Result<Extractable<P>> {
    fn from(path: &P) -> Self {
        let r = path.as_ref();

        Ok(Extractable {
            path,
        })
    }
}

//impl<P: AsRef<Path>> TryFrom<PathRef<P>> for Extractable<P> {
/*
impl<P: AsRef<Path>> TryFrom<&P> for Extractable<P> {
    type Error = ast::Error;

    fn try_from(path: PathRef<P>) -> Result<Self, Self::Error> {
        let r = path.0.as_ref();

        r.

        todo!("implement extension check!");

        Ok(Extractable {
            path,
        })
    }
}
*/

pub trait Extractor {
    type Path: AsRef<Path>;
    fn extract(&mut self, file: Extractable<Self::Path>, target: Self::Path) -> ast::Result<()>;
}
*/
