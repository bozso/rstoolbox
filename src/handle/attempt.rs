use std::{error, fmt, num};

use crate::handle::{Create, Error, Status};

#[derive(Debug)]
pub struct GotZero<I> {
    num: I,
}

impl<I> GotZero<I> {
    fn new(num: I) -> Self {
        Self { num: num }
    }
}

impl<I: fmt::Display> fmt::Display for GotZero<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected non zero {} value, got {}",
            std::any::type_name::<I>(),
            self.num
        )
    }
}

impl<I: fmt::Debug + fmt::Display> error::Error for GotZero<I> {}

pub struct NTimes {
    n_tries: num::NonZeroU64,
}

impl NTimes {
    pub fn new(n_tries: u64) -> Result<Self, GotZero<u64>> {
        Ok(Self {
            n_tries: num::NonZeroU64::new(n_tries).ok_or(GotZero::new(n_tries))?,
        })
    }
}

impl Create for NTimes {
    type Handler = NTimesImpl;
    type Err = std::convert::Infallible;

    fn create(&self) -> Result<Self::Handler, Self::Err> {
        Ok(Self::Handler {
            n_tries: self.n_tries.get(),
            current: 0,
        })
    }
}

pub struct NTimesImpl {
    n_tries: u64,
    current: u64,
}

impl Error for NTimesImpl {
    fn handle<E>(&mut self, result: &E) -> Status {
        if self.current >= self.n_tries {
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
        let mut h = n.create().unwrap();
        h.drain_result(|| Result::<(), ()>::Ok(()));
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
