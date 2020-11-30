use crate::distribution::{Continuous, ContinuousCDF};
use crate::function::{beta, gamma};
use crate::is_zero;
use crate::statistics::*;
use crate::{Result, StatsError};
use core::f64::INFINITY as INF;
use rand::Rng;

/// Implements the [Beta](https://en.wikipedia.org/wiki/Beta_distribution)
/// distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::{Beta, Continuous};
/// use statrs::statistics::*;
/// use statrs::prec;
///
/// let n = Beta::new(2.0, 2.0).unwrap();
/// assert_eq!(n.mean().unwrap(), 0.5);
/// assert!(prec::almost_eq(n.pdf(0.5), 1.5, 1e-14));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Beta {
    shape_a: f64,
    shape_b: f64,
}

impl Beta {
    /// Constructs a new beta distribution with shapeA (α) of `shape_a`
    /// and shapeB (β) of `shape_b`
    ///
    /// # Errors
    ///
    /// Returns an error if `shape_a` or `shape_b` are `NaN`.
    /// Also returns an error if `shape_a <= 0.0` or `shape_b <= 0.0`
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Beta;
    ///
    /// let mut result = Beta::new(2.0, 2.0);
    /// assert!(result.is_ok());
    ///
    /// result = Beta::new(0.0, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn new(shape_a: f64, shape_b: f64) -> Result<Beta> {
        let is_nan = shape_a.is_nan() || shape_b.is_nan();
        match (shape_a, shape_b, is_nan) {
            (_, _, true) => Err(StatsError::BadParams),
            (_, _, false) if shape_a <= 0.0 || shape_b <= 0.0 => Err(StatsError::BadParams),
            (_, _, false) => Ok(Beta { shape_a, shape_b }),
        }
    }

    /// Returns the shapeA (α) of the beta distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Beta;
    ///
    /// let n = Beta::new(2.0, 2.0).unwrap();
    /// assert_eq!(n.shape_a(), 2.0);
    /// ```
    pub fn shape_a(&self) -> f64 {
        self.shape_a
    }

    /// Returns the shapeB (β) of the beta distributionβ
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Beta;
    ///
    /// let n = Beta::new(2.0, 2.0).unwrap();
    /// assert_eq!(n.shape_b(), 2.0);
    /// ```
    pub fn shape_b(&self) -> f64 {
        self.shape_b
    }
}

impl ::rand::distributions::Distribution<f64> for Beta {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        // Generated by sampling two gamma distributions and normalizing.
        let x = super::gamma::sample_unchecked(rng, self.shape_a, 1.0);
        let y = super::gamma::sample_unchecked(rng, self.shape_b, 1.0);
        x / (x + y)
    }
}

impl ContinuousCDF<f64, f64> for Beta {
    /// Calculates the cumulative distribution function for the beta
    /// distribution
    /// at `x`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// I_x(α, β)
    /// ```
    ///
    /// where `α` is shapeA, `β` is shapeB, and `I_x` is the regularized
    /// lower incomplete beta function
    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else if x >= 1.0 {
            1.0
        } else if self.shape_a.is_infinite() && self.shape_b.is_infinite() {
            if x < 0.5 {
                0.0
            } else {
                1.0
            }
        } else if self.shape_a.is_infinite() {
            if x < 1.0 {
                0.0
            } else {
                1.0
            }
        } else if self.shape_b.is_infinite() {
            1.0
        } else if ulps_eq!(self.shape_a, 1.0) && ulps_eq!(self.shape_b, 1.0) {
            x
        } else {
            beta::beta_reg(self.shape_a, self.shape_b, x)
        }
    }
}

impl Min<f64> for Beta {
    /// Returns the minimum value in the domain of the
    /// beta distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 0
    /// ```
    fn min(&self) -> f64 {
        0.0
    }
}

impl Max<f64> for Beta {
    /// Returns the maximum value in the domain of the
    /// beta distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 1
    /// ```
    fn max(&self) -> f64 {
        1.0
    }
}

impl Distribution<f64> for Beta {
    /// Returns the mean of the beta distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// α / (α + β)
    /// ```
    ///
    /// where `α` is shapeA and `β` is shapeB
    fn mean(&self) -> Option<f64> {
        let mean = if self.shape_a.is_infinite() && self.shape_b.is_infinite() {
            return None;
        } else if self.shape_a.is_infinite() {
            1.0
        } else {
            self.shape_a / (self.shape_a + self.shape_b)
        };
        Some(mean)
    }
    /// Returns the variance of the beta distribution
    ///
    /// # Remarks
    ///
    /// Returns `None` if both `shape_a` and `shape_b` are
    /// positive infinity, since this limit cannot be consistently
    /// defined.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (α * β) / ((α + β)^2 * (α + β + 1))
    /// ```
    ///
    /// where `α` is shapeA and `β` is shapeB
    fn variance(&self) -> Option<f64> {
        let var = if self.shape_a.is_infinite() && self.shape_b.is_infinite() {
            return None;
        } else if self.shape_a.is_infinite() || self.shape_b.is_infinite() {
            0.0
        } else {
            self.shape_a * self.shape_b
                / ((self.shape_a + self.shape_b)
                    * (self.shape_a + self.shape_b)
                    * (self.shape_a + self.shape_b + 1.0))
        };
        Some(var)
    }
    /// Returns the entropy of the beta distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln(B(α, β)) - (α - 1)ψ(α) - (β - 1)ψ(β) + (α + β - 2)ψ(α + β)
    /// ```
    ///
    /// where `α` is shapeA, `β` is shapeB and `ψ` is the digamma function
    fn entropy(&self) -> Option<f64> {
        let entr = if self.shape_a.is_infinite() || self.shape_b.is_infinite() {
            // unsupported limit
            return None;
        } else {
            beta::ln_beta(self.shape_a, self.shape_b)
                - (self.shape_a - 1.0) * gamma::digamma(self.shape_a)
                - (self.shape_b - 1.0) * gamma::digamma(self.shape_b)
                + (self.shape_a + self.shape_b - 2.0) * gamma::digamma(self.shape_a + self.shape_b)
        };
        Some(entr)
    }
    /// Returns the skewness of the Beta distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 2(β - α) * sqrt(α + β + 1) / ((α + β + 2) * sqrt(αβ))
    /// ```
    ///
    /// where `α` is shapeA and `β` is shapeB
    fn skewness(&self) -> Option<f64> {
        let skew = if self.shape_a.is_infinite() && self.shape_b.is_infinite() {
            0.0
        } else if self.shape_a.is_infinite() {
            -2.0
        } else if self.shape_b.is_infinite() {
            2.0
        } else {
            2.0 * (self.shape_b - self.shape_a) * (self.shape_a + self.shape_b + 1.0).sqrt()
                / ((self.shape_a + self.shape_b + 2.0) * (self.shape_a * self.shape_b).sqrt())
        };
        Some(skew)
    }
}

impl Mode<Option<f64>> for Beta {
    /// Returns the mode of the Beta distribution.
    ///
    /// # Remarks
    ///
    /// Since the mode is technically only calculate for `α > 1, β > 1`, those
    /// are the only values we allow. We may consider relaxing this constraint
    /// in
    /// the future.
    ///
    /// # Panics
    ///
    /// If `α <= 1` or `β <= 1`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (α - 1) / (α + β - 2)
    /// ```
    ///
    /// where `α` is shapeA and `β` is shapeB
    fn mode(&self) -> Option<f64> {
        // TODO: perhaps relax constraint in order to allow calculation
        // of 'anti-mode;
        if self.shape_a <= 1.0
            || self.shape_b <= 1.0
            || self.shape_a.is_infinite() && self.shape_b.is_infinite()
        {
            None
        } else if self.shape_a.is_infinite() {
            Some(1.0)
        } else {
            Some((self.shape_a - 1.0) / (self.shape_a + self.shape_b - 2.0))
        }
    }
}

impl Continuous<f64, f64> for Beta {
    /// Calculates the probability density function for the beta distribution
    /// at `x`.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// let B(α, β) = Γ(α)Γ(β)/Γ(α + β)
    ///
    /// x^(α - 1) * (1 - x)^(β - 1) / B(α, β)
    /// ```
    ///
    /// where `α` is shapeA, `β` is shapeB, and `Γ` is the gamma function
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 || x > 1.0 {
            0.0
        } else if self.shape_a.is_infinite() && self.shape_b.is_infinite() {
            if ulps_eq!(x, 0.5) {
                INF
            } else {
                0.0
            }
        } else if self.shape_a.is_infinite() {
            if ulps_eq!(x, 1.0) {
                INF
            } else {
                0.0
            }
        } else if self.shape_b.is_infinite() {
            if is_zero(x) {
                INF
            } else {
                0.0
            }
        } else if ulps_eq!(self.shape_a, 1.0) && ulps_eq!(self.shape_b, 1.0) {
            1.0
        } else if self.shape_a > 80.0 || self.shape_b > 80.0 {
            self.ln_pdf(x).exp()
        } else {
            let bb = gamma::gamma(self.shape_a + self.shape_b)
                / (gamma::gamma(self.shape_a) * gamma::gamma(self.shape_b));
            bb * x.powf(self.shape_a - 1.0) * (1.0 - x).powf(self.shape_b - 1.0)
        }
    }

    /// Calculates the log probability density function for the beta
    /// distribution at `x`.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// let B(α, β) = Γ(α)Γ(β)/Γ(α + β)
    ///
    /// ln(x^(α - 1) * (1 - x)^(β - 1) / B(α, β))
    /// ```
    ///
    /// where `α` is shapeA, `β` is shapeB, and `Γ` is the gamma function
    fn ln_pdf(&self, x: f64) -> f64 {
        if x < 0.0 || x > 1.0 {
            -INF
        } else if self.shape_a.is_infinite() && self.shape_b.is_infinite() {
            if ulps_eq!(x, 0.5) {
                INF
            } else {
                -INF
            }
        } else if self.shape_a.is_infinite() {
            if ulps_eq!(x, 1.0) {
                INF
            } else {
                -INF
            }
        } else if self.shape_b.is_infinite() {
            if is_zero(x) {
                INF
            } else {
                -INF
            }
        } else if ulps_eq!(self.shape_a, 1.0) && ulps_eq!(self.shape_b, 1.0) {
            0.0
        } else {
            let aa = gamma::ln_gamma(self.shape_a + self.shape_b)
                - gamma::ln_gamma(self.shape_a)
                - gamma::ln_gamma(self.shape_b);
            let bb = if ulps_eq!(self.shape_a, 1.0) && is_zero(x) {
                0.0
            } else if is_zero(x) {
                -INF
            } else {
                (self.shape_a - 1.0) * x.ln()
            };
            let cc = if ulps_eq!(self.shape_b, 1.0) && ulps_eq!(x, 1.0) {
                0.0
            } else if ulps_eq!(x, 1.0) {
                -INF
            } else {
                (self.shape_b - 1.0) * (1.0 - x).ln()
            };
            aa + bb + cc
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::ACC;
    use crate::distribution::internal::*;
    use crate::statistics::*;
    use crate::testing_boiler;

    testing_boiler!((f64, f64), Beta);

    #[test]
    fn test_create() {
        let valid = [(1.0, 1.0), (9.0, 1.0), (5.0, 100.0), (1.0, INF), (INF, 1.0)];
        for &arg in valid.iter() {
            try_create(arg);
        }
    }

    #[test]
    fn test_bad_create() {
        let invalid = [
            (0.0, 0.0),
            (0.0, 0.1),
            (1.0, 0.0),
            (0.0, INF),
            (INF, 0.0),
            (f64::NAN, 1.0),
            (1.0, f64::NAN),
            (f64::NAN, f64::NAN),
            (1.0, -1.0),
            (-1.0, 1.0),
            (-1.0, -1.0),
        ];
        for &arg in invalid.iter() {
            bad_create_case(arg);
        }
    }

    #[test]
    fn test_mean() {
        let f = |x: Beta| x.mean().unwrap();
        let test = [
            ((1.0, 1.0), 0.5),
            ((9.0, 1.0), 0.9),
            ((5.0, 100.0), 0.047619047619047619047616),
            ((1.0, INF), 0.0),
            ((INF, 1.0), 1.0),
        ];
        for &(arg, res) in test.iter() {
            test_case(arg, res, f);
        }
        let mean = |x: Beta| x.mean();
        test_none((INF, INF), mean);
    }

    #[test]
    fn test_variance() {
        let f = |x: Beta| x.variance().unwrap();
        let test = [
            ((1.0, 1.0), 1.0 / 12.0),
            ((9.0, 1.0), 9.0 / 1100.0),
            ((5.0, 100.0), 500.0 / 1168650.0),
            ((1.0, INF), 0.0),
            ((INF, 1.0), 0.0),
        ];
        for &(arg, res) in test.iter() {
            test_case(arg, res, f);
        }
        let variance = |x: Beta| x.variance();
        test_none((INF, INF), variance);
    }

    #[test]
    fn test_entropy() {
        let f = |x: Beta| x.entropy().unwrap();
        let test = [
            ((9.0, 1.0), -1.3083356884473304939016015),
            ((5.0, 100.0), -2.52016231876027436794592),
        ];
        for &(arg, res) in test.iter() {
            test_case(arg, res, f);
        }
        test_case_special((1.0, 1.0), 0.0, 1e-14, f);
        let entropy = |x: Beta| x.entropy();
        test_none((1.0, INF), entropy);
        test_none((INF, 1.0), entropy);
        test_none((INF, INF), entropy);
    }

    #[test]
    fn test_skewness() {
        let skewness = |x: Beta| x.skewness().unwrap();
        test_case((1.0, 1.0), 0.0, skewness);
        test_case((9.0, 1.0), -1.4740554623801777107177478829, skewness);
        test_case((5.0, 100.0), 0.817594109275534303545831591, skewness);
        test_case((1.0, INF), 2.0, skewness);
        test_case((INF, 1.0), -2.0, skewness);
        test_case((INF, INF), 0.0, skewness);
    }

    #[test]
    fn test_mode() {
        let mode = |x: Beta| x.mode().unwrap();
        test_case((5.0, 100.0), 0.038834951456310676243255386, mode);
        test_case((92.0, INF), 0.0, mode);
        test_case((INF, 2.0), 1.0, mode);
        let mode = |x: Beta| x.mode();
        test_none((INF, INF), mode);
    }

    #[test]
    #[should_panic]
    fn test_mode_shape_a_lte_1() {
        let mode = |x: Beta| x.mode().unwrap();
        get_value((1.0, 5.0), mode);
    }

    #[test]
    #[should_panic]
    fn test_mode_shape_b_lte_1() {
        let mode = |x: Beta| x.mode().unwrap();
        get_value((5.0, 1.0), mode);
    }

    #[test]
    fn test_min_max() {
        let min = |x: Beta| x.min();
        let max = |x: Beta| x.max();
        test_case((1.0, 1.0), 0.0, min);
        test_case((1.0, 1.0), 1.0, max);
    }

    #[test]
    fn test_pdf() {
        let f = |arg: f64| move |x: Beta| x.pdf(arg);
        let test = [
            ((1.0, 1.0), 0.0, 1.0),
            ((1.0, 1.0), 0.5, 1.0),
            ((1.0, 1.0), 1.0, 1.0),
            ((9.0, 1.0), 0.0, 0.0),
            ((9.0, 1.0), 0.5, 0.03515625),
            ((9.0, 1.0), 1.0, 9.0),
            ((5.0, 100.0), 0.0, 0.0),
            ((5.0, 100.0), 0.5, 4.534102298350337661e-23),
            ((5.0, 100.0), 1.0, 0.0),
            ((5.0, 100.0), 1.0, 0.0),
            ((1.0, INF), 0.0, INF),
            ((1.0, INF), 0.5, 0.0),
            ((1.0, INF), 1.0, 0.0),
            ((INF, 1.0), 0.0, 0.0),
            ((INF, 1.0), 0.5, 0.0),
            ((INF, 1.0), 1.0, INF),
            ((INF, INF), 0.0, 0.0),
            ((INF, INF), 0.5, INF),
            ((INF, INF), 1.0, 0.0),
        ];
        for &(arg, x, expect) in test.iter() {
            test_case(arg, expect, f(x));
        }
    }

    #[test]
    fn test_pdf_input_lt_0() {
        let pdf = |arg: f64| move |x: Beta| x.pdf(arg);
        test_case((1.0, 1.0), 0.0, pdf(-1.0));
    }

    #[test]
    fn test_pdf_input_gt_0() {
        let pdf = |arg: f64| move |x: Beta| x.pdf(arg);
        test_case((1.0, 1.0), 0.0, pdf(2.0));
    }

    #[test]
    fn test_ln_pdf() {
        let f = |arg: f64| move |x: Beta| x.ln_pdf(arg);
        let test = [
            ((1.0, 1.0), 0.0, 0.0),
            ((1.0, 1.0), 0.5, 0.0),
            ((1.0, 1.0), 1.0, 0.0),
            ((9.0, 1.0), 0.0, -INF),
            ((9.0, 1.0), 0.5, -3.347952867143343092547366497),
            ((9.0, 1.0), 1.0, 2.1972245773362193827904904738),
            ((5.0, 100.0), 0.0, -INF),
            ((5.0, 100.0), 0.5, -51.447830024537682154565870),
            ((5.0, 100.0), 1.0, -INF),
            ((1.0, INF), 0.0, INF),
            ((1.0, INF), 0.5, -INF),
            ((1.0, INF), 1.0, -INF),
            ((INF, 1.0), 0.0, -INF),
            ((INF, 1.0), 0.5, -INF),
            ((INF, 1.0), 1.0, INF),
            ((INF, INF), 0.0, -INF),
            ((INF, INF), 0.5, INF),
            ((INF, INF), 1.0, -INF),
        ];
        for &(arg, x, expect) in test.iter() {
            test_case(arg, expect, f(x));
        }
    }

    #[test]
    fn test_ln_pdf_input_lt_0() {
        let ln_pdf = |arg: f64| move |x: Beta| x.ln_pdf(arg);
        test_case((1.0, 1.0), -INF, ln_pdf(-1.0));
    }

    #[test]
    fn test_ln_pdf_input_gt_1() {
        let ln_pdf = |arg: f64| move |x: Beta| x.ln_pdf(arg);
        test_case((1.0, 1.0), -INF, ln_pdf(2.0));
    }

    #[test]
    fn test_cdf() {
        let cdf = |arg: f64| move |x: Beta| x.cdf(arg);
        let test = [
            ((1.0, 1.0), 0.0, 0.0),
            ((1.0, 1.0), 0.5, 0.5),
            ((1.0, 1.0), 1.0, 1.0),
            ((9.0, 1.0), 0.0, 0.0),
            ((9.0, 1.0), 0.5, 0.001953125),
            ((9.0, 1.0), 1.0, 1.0),
            ((5.0, 100.0), 0.0, 0.0),
            ((5.0, 100.0), 0.5, 1.0),
            ((5.0, 100.0), 1.0, 1.0),
            ((1.0, INF), 0.0, 1.0),
            ((1.0, INF), 0.5, 1.0),
            ((1.0, INF), 1.0, 1.0),
            ((INF, 1.0), 0.0, 0.0),
            ((INF, 1.0), 0.5, 0.0),
            ((INF, 1.0), 1.0, 1.0),
            ((INF, INF), 0.0, 0.0),
            ((INF, INF), 0.5, 1.0),
            ((INF, INF), 1.0, 1.0),
        ];
        for &(arg, x, expect) in test.iter() {
            test_case(arg, expect, cdf(x));
        }
    }

    #[test]
    fn test_cdf_input_lt_0() {
        let cdf = |arg: f64| move |x: Beta| x.cdf(arg);
        test_case((1.0, 1.0), 0.0, cdf(-1.0));
    }

    #[test]
    fn test_cdf_input_gt_1() {
        let cdf = |arg: f64| move |x: Beta| x.cdf(arg);
        test_case((1.0, 1.0), 1.0, cdf(2.0));
    }

    #[test]
    fn test_continuous() {
        test::check_continuous_distribution(&try_create((1.2, 3.4)), 0.0, 1.0);
        test::check_continuous_distribution(&try_create((4.5, 6.7)), 0.0, 1.0);
    }
}
