use std::{
    num, fmt, error,
};

use crate::{
    handle::{Error, Status, Create},
};

#[derive(Debug)]
pub struct GotZero<I> {
    num: I,
}

impl<I> GotZero<I> {
    fn new(num: I) -> Self {
        Self {
            num: num,
        }
    }
}

impl<I: fmt::Display> fmt::Display for GotZero<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected non zero {} value, got {}",
                std::any::type_name::<I>(), self.num)
    }

}

impl<I: fmt::Debug + fmt::Display> error::Error for GotZero<I> {}

pub struct NTimes(num::NonZeroU64);

impl NTimes {
    pub fn new(n_tries: u64) -> Result<Self, GotZero<u64>> {
        Ok(Self(
            num::NonZeroU64::new(n_tries)
                .ok_or(GotZero::new(n_tries))?
        ))
    }
}

impl Create for NTimes {
    type Handler = NTimesImpl;
    type Err = std::convert::Infallible;

    fn create_handler(&self) -> Result<Self::Handler, Self::Err> {
        Ok(Self::Handler {
            n_tries: self.0.get(),
            current: 0,
        })
    }
}

pub struct NTimesImpl {
    n_tries: u64,
    current: u64,
}

impl Error for NTimesImpl {
    fn handle<T, E>(&mut self, res: &Result<T, E>) -> Status {
        if res.is_ok() || self.current >= self.n_tries {
            Status::Finished
        } else {
            self.current += 1;
            Status::Continue
        }
    }
}

fn reset() {
    let n = NTimes::new(5).unwrap();
    {
        let mut h = n.create_handler().unwrap();
        h.drain_result(|| Result::<(),()>::Ok(()));
        assert!(h.current < 5);
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn all() {
        super::reset();
    }
}
