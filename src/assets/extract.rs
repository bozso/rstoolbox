use crate::assets as ast;

pub trait Extractor {
    fn extract(&mut self, file: Pathbuf, target: PathBuf) -> ast::Result<()> {
    }
}
