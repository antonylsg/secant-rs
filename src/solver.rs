use error::Error;

#[derive(Debug)]
pub struct Output {
    pub x: f64,
    pub iter: usize,
}

#[derive(Default)]
pub struct SolverBuilder {
    tol: Option<f64>,
    step: Option<f64>,
    max_iter: Option<usize>,
}

impl SolverBuilder {
    pub fn tol(mut self, tol: f64) -> Self {
        self.tol = Some(tol);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = Some(max_iter);
        self
    }

    pub fn build(self) -> Solver {
        Solver {
            tol: self.tol.unwrap_or(1.48_e-8),
            step: self.step.unwrap_or(1.0_e-4),
            max_iter: self.max_iter.unwrap_or(50),
        }
    }
}

#[derive(Debug)]
pub struct Solver {
    tol: f64,
    step: f64,
    max_iter: usize,
}

impl Default for Solver {
    fn default() -> Solver {
        Solver {
            tol: 1.48_e-8,
            step: 1.0_e-4,
            max_iter: 50,
        }
    }
}

impl Solver {
    pub fn solve<F>(&self, mut x0: f64, mut f: F) -> Result<Output, Error>
    where
        F: FnMut(f64) -> f64,
    {
        let mut x1 = if x0 >= 0.0 {
            (1.0 + self.step) * x0 + self.step
        } else {
            (1.0 + self.step) * x0 - self.step
        };

        let mut y0 = f(x0);
        let mut y1 = f(x1);

        for iter in 0..self.max_iter {
            if y1 == y0 {
                return Ok(Output {
                    x: 0.5 * (x1 + x0),
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
