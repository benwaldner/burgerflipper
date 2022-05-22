use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Long,
    Short,
    Natural,
}

#[derive(Clone)]
pub struct Ema {
    output: Option<f64>,
    data: VecDeque<f64>,
    alpha: f64,
}

impl Ema {
    pub fn new(period: usize) -> Self {
        debug_assert!(period >= 1);

        Self {
            output: None,
            alpha: 2.0 / (1.0 + period as f64),
            data: VecDeque::with_capacity(3),
        }
    }

    pub fn run(&mut self, input: f64) {
        if let Some(output) = &mut self.output {
            *output = input * self.alpha + *output * (1.0 - self.alpha);
            self.data.push_back(*output);
        } else {
            self.output = Some(input);
            self.data.push_back(input);
        }
        if self.data.len() > 3 {
            self.data.pop_front().unwrap();
        }
    }

    pub fn get(&self) -> f64 {
        self.output.expect("No value assigned to EMA.")
    }

    pub fn get_signal(&self) -> Direction {
        let t = &self.data;
        if t[2] > t[1] && t[1] > t[0] {
            return Direction::Long;
        } else if t[2] < t[1] && t[1] < t[0] {
            return Direction::Short;
        } else {
            return Direction::Natural;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut ema = Ema::new(3);
        ema.run(0.0);
        assert_eq!(ema.get(), 0.0);
        ema.run(2.0);
        assert_eq!(ema.get(), 1.0);
        ema.run(-1.0);
        assert_eq!(ema.get(), 0.0);
        ema.run(16.0);
        assert_eq!(ema.get(), 8.0);
        ema.run(16.0);
        assert_eq!(ema.get_signal(), Direction::Long);
        ema.run(17.0);
        assert_eq!(ema.get_signal(), Direction::Long);
        ema.run(18.0);
        assert_eq!(ema.get_signal(), Direction::Long);
        ema.run(20.0);
        assert_eq!(ema.get_signal(), Direction::Long);
        ema.run(20.0);
        assert_eq!(ema.get_signal(), Direction::Long);
        ema.run(20.0);
        assert_eq!(ema.get_signal(), Direction::Long);
        ema.run(19.0);
        assert_eq!(ema.get_signal(), Direction::Natural);
        ema.run(18.0);
        assert_eq!(ema.get_signal(), Direction::Short);
        ema.run(17.0);
        assert_eq!(ema.get_signal(), Direction::Short);
    }
}
