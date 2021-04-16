use std::{
    path::Path,
    io::{BufReader, Read},
    fs::File,
    marker::PhantomData,
};

pub trait RemoteFile<P>
where
    P: AsRef<Path>
{
    type Error: std::error::Error + From<std::io::Error>;

    fn get_remote(&mut self, remote: &P) -> Result<P, Self::Error>;

    fn get_buf_reader(&mut self, remote: &P) -> Result<BufReader<File>, Self::Error> {
        let file = File::open(self.get_remote(remote)?)?;
        Ok(BufReader::new(file))
    }

    fn read_contents(&mut self, remote: &P) -> Result<String, Self::Error> {
        let mut reader = self.get_buf_reader(remote)?;
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

struct Impl<P, R> {
    remote_file: R,
    tag: PhantomData<P>,
}

impl<P, R> Impl<P, R> 
where
    P: AsRef<Path>,
    R: RemoteFile<P>,
{
    async fn get_remote(&mut self, remote: &P) -> Result<P, R::Error> {
        self.remote_file.get_remote(remote)
    }
}

