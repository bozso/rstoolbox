use crate::assets::{
    //Downloader, Extractor, Task, Config, Result,
    Downloader, Task, Config, Result,
    task::Downloaded,
};

pub struct Service<D, E> {
    pub(crate) down: D,
    pub(crate) extract: E,
    pub(crate) config: Config,
}

impl<D: Downloader, E> Service<D, E> {
    pub fn download<'a>(&mut self, task: &'a Task) -> Result<Downloaded<'a>> {
        let root = self.config.paths.get_root(&task.target);
        
        //TODO(bozso): implement downloading

        Ok(Downloaded {
            root: root,
            symlinks: &task.symlinks,
        })
    }

    //pub fn extract_entry(&mut self, 
}

