use num_traits::Float;

use crate::error::Error;

#[derive(Debug)]
pub struct Output<T> {
    pub x: T,
    pub iter: usize,
}

#[derive(Default)]
pub struct Builder<T> {
    tol: Option<T>,
    step: Option<T>,
    max_iter: Option<usize>,
}

impl<T: Float> Builder<T> {
    pub fn tol(mut self, tol: T) -> Self {
        self.tol = Some(tol);
        self
    }

    pub fn step(mut self, step: T) -> Self {
        self.step = Some(step);
        self
    }

    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = Some(max_iter);
        self
    }

    pub fn build(self) -> Solver<T> {
        Solver {
            tol: self.tol.unwrap_or_else(|| T::from(1.48_e-8).unwrap()),
            step: self.step.unwrap_or_else(|| T::from(1.0_e-4).unwrap()),
            max_iter: self.max_iter.unwrap_or(50),
        }
    }
}

#[derive(Debug)]
pub struct Solver<T> {
    tol: T,
    step: T,
    max_iter: usize,
}

impl<T: Float> Default for Solver<T> {
    fn default() -> Solver<T> {
        Solver {
            tol: T::from(1.48_e-8).unwrap(),
            step: T::from(1.0_e-4).unwrap(),
            max_iter: 50,
        }
    }
}

impl<T: Float> Solver<T> {
    pub fn solve<F>(&self, mut x0: T, mut f: F) -> Result<Output<T>, Error>
    where
        F: FnMut(T) -> T,
    {
        let mut x1 = if x0 >= T::zero() {
            (T::one() + self.step) * x0 + self.step
        } else {
            (T::one() + self.step) * x0 - self.step
        };

        let mut y0 = f(x0);
        let mut y1 = f(x1);

        for iter in 0..self.max_iter {
            if y1 == y0 {
                return Ok(Output {
                    x: T::from(0.5).unwrap() * (x1 + x0),
                    iter,
                });
            }

            let dx = y1 * (x1 - x0) / (y1 - y0);
            let x = x1 - dx;

            if dx.abs() < self.tol {
                return Ok(Output { x, iter });
            }

            x0 = x1;
            y0 = y1;
            x1 = x;
            y1 = f(x1);
        }

        Err(Error::MaxIter(self.max_iter))
    }
}
