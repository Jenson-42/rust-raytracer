/// A struct representing an interval between two numbers.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Create an `Interval` instance ensuring that `min <= max`.
    /// If `min == max`, the instance will be treated as "empty".
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// assert_eq!(Interval::new(10.0, 5.0), Interval::new(5.0, 10.0));
    /// ```
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            min: f64::min(a, b),
            max: f64::max(a, b),
        }
    }

    /// Create an `Interval` instance with `0.0` as min and max.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// assert_eq!(Interval::empty(), Interval::new(0.0, 0.0));
    /// ```
    pub fn empty() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Return true if the interval is empty.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// assert!(Interval::new(0.0, 0.0).is_empty())
    /// ```
    pub fn is_empty(&self) -> bool {
        self.min == self.max
    }

    /// Calculate the size of an interval instance.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// assert_eq!(Interval::new(1.0, 5.0).size(), 4.0);
    /// ```
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Make an interval bigger by a given delta.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// assert_eq!(Interval::new(1.0, 5.0).expand(2.0), Interval::new(0.0, 6.0));
    /// ```
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// Construct a new interval that contains two intervals.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// let a = Interval::new(1.0, 3.0);
    /// let b = Interval::new(5.0, 8.0);
    ///
    /// assert_eq!(Interval::containing(a, b), Interval::new(1.0, 8.0));
    /// ```
    pub fn containing(a: Self, b: Self) -> Self {
        match (a.is_empty(), b.is_empty()) {
            (true, true) => Self::empty(),
            (false, true) => a,
            (true, false) => b,
            (false, false) => Interval::new(f64::min(a.min, b.min), f64::max(a.max, b.max)),
        }
    }

    /// Construct an interval from the overlap of two intervals, if any.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::Interval;
    /// let a = Interval::new(0.0, 1.0);
    /// let b = Interval::new(1.0, 3.0);
    /// let c = Interval::new(2.0, 4.0);
    ///
    /// assert_eq!(Interval::overlap(a, b), None);
    /// assert_eq!(Interval::overlap(b, c), Some(Interval::new(2.0, 3.0)));
    pub fn overlap(a: Self, b: Self) -> Option<Self> {
        let min = f64::max(a.min, b.min);
        let max = f64::min(a.max, b.max);

        if min >= max {
            None
        } else {
            Some(Self { min, max })
        }
    }
}
