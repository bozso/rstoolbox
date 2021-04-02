use crate::assets::{
    Downloader, Extractor, Task, Config, Result,
    task::Downloaded,
};

pub struct Service<D, E> {
    down: D,
    extract: E,
    config: Config,
}

impl<D: Downloader, E: Extractor> Service<D, E> {
    pub fn download<'a>(&mut self, task: &'a Task) -> Result<Downloaded<'a>> {
        let root = self.config.paths.get_root(task.target);
        
        todo!("implement downloading");

        Ok(Downloaded {
            root: root,
            symlinks: &task.symlinks,
        })
    }

    //pub fn extract_entry(&mut self, 
}

