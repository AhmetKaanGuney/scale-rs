use std::{
    ops::{Add, Div, Mul, Sub},
    time::Duration,
};

pub struct LinearScale {
    pub min: f32,
    pub max: f32,
}

pub struct TimeScale {
    pub min: Duration,
    pub max: Duration,
}

#[derive(PartialEq, Eq)]
pub struct ScaleUnit<T>(T);

impl Add for ScaleUnit<f32> {
    type Output = ScaleUnit<f32>;

    fn add(self, rhs: Self) -> Self::Output {
        ScaleUnit(self.0 + rhs.0)
    }
}
impl Sub for ScaleUnit<f32> {
    type Output = ScaleUnit<f32>;

    fn sub(self, rhs: Self) -> Self::Output {
        ScaleUnit(self.0 - rhs.0)
    }
}
impl Div for ScaleUnit<f32> {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 as f64 / rhs.0 as f64
    }
}
impl Mul<ScaleUnit<f32>> for f64 {
    type Output = ScaleUnit<f32>;

    fn mul(self, rhs: ScaleUnit<f32>) -> Self::Output {
        ScaleUnit((self * rhs.0 as f64) as f32)
    }
}

// Duration
impl Add for ScaleUnit<Duration> {
    type Output = ScaleUnit<Duration>;

    fn add(self, rhs: Self) -> Self::Output {
        ScaleUnit(self.0 + rhs.0)
    }
}
impl Sub for ScaleUnit<Duration> {
    type Output = ScaleUnit<Duration>;

    fn sub(self, rhs: Self) -> Self::Output {
        ScaleUnit(self.0 - rhs.0)
    }
}
impl Div for ScaleUnit<Duration> {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.as_secs_f64() / rhs.0.as_secs_f64()
    }
}
impl Mul<ScaleUnit<Duration>> for f64 {
    type Output = ScaleUnit<Duration>;

    fn mul(self, rhs: ScaleUnit<Duration>) -> Self::Output {
        ScaleUnit(Duration::from_secs_f64(self * rhs.0.as_secs_f64()))
    }
}

pub trait Scale<T>
where
    T: std::cmp::PartialEq,
    ScaleUnit<T>: Add<Output = ScaleUnit<T>>,
    ScaleUnit<T>: Sub<Output = ScaleUnit<T>>,
    f64: Mul<ScaleUnit<T>, Output = ScaleUnit<T>>,
    ScaleUnit<T>: Div<Output = f64>,
{
    fn min(&self) -> ScaleUnit<T>;
    fn max(&self) -> ScaleUnit<T>;
    fn range(&self) -> ScaleUnit<T> {
        self.max() - self.min()
    }

    fn norm(&self, value: ScaleUnit<T>) -> f64 {
        if self.min() == self.max() {
            return 0.0;
        }
        (value - self.min()) / self.range()
    }

    fn lerp(&self, t: f64) -> ScaleUnit<T> {
        self.min() + t * self.range()
    }
}

pub fn linear_converter_test(
    src: Box<dyn Scale<f32>>,
    dst: Box<dyn Scale<Duration>>,
) -> Box<dyn Fn(ScaleUnit<f32>) -> ScaleUnit<Duration>> {
    Box::new(move |value| {
        let t = src.norm(value);
        dst.lerp(t)
    })
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num_scale = LinearScale {
            min: 0.0,
            max: 10.0,
        };
        let time_scale = TimeScale {
            min: Duration::from_secs(0),
            max: Duration::from_secs(10),
        };
    }
}
