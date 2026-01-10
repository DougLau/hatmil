// poly.rs
// Copyright (C) 2025-2026  Douglas P Lau
//
use std::fmt;
use std::fmt::Write;

/// SVG [Polygon] / [Polyline] point builder
///
/// ```rust
/// # use hatmil::svg::Polygon;
/// let mut points = Polygon::point_builder();
/// points.precision(2);
/// points.add([5, 5]);
/// points.add((10.1, 20.2));
/// println!("{points}");
/// ```
///
/// [Polygon]: svg/struct.Polygon.html#method.points
/// [Polyline]: svg/struct.Polyline.html#method.points
#[derive(Clone)]
pub struct PolyPointBuilder {
    /// Precision in decimal places
    precision: usize,
    /// Points string
    points: String,
}

impl fmt::Display for PolyPointBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.points)?;
        Ok(())
    }
}

impl From<PolyPointBuilder> for String {
    fn from(poly: PolyPointBuilder) -> Self {
        // zero-copy alternative to fmt::Display
        poly.points
    }
}

impl PolyPointBuilder {
    /// Create a new SVG polygon / polyline points builder
    pub(crate) fn new() -> Self {
        PolyPointBuilder {
            precision: 2,
            points: String::new(),
        }
    }

    /// Set the precision in decimal places
    pub fn precision(&mut self, digits: usize) -> &mut Self {
        self.precision = digits;
        self
    }

    /// Write one value
    fn value(&mut self, v: f64) {
        write!(&mut self.points, "{v:.0$}", self.precision).unwrap();
        if self.precision > 0 {
            while self.points.ends_with('0') {
                self.points.pop();
            }
            if self.points.ends_with('.') {
                self.points.pop();
            }
        }
    }

    /// Write one point
    fn point(&mut self, x: f64, y: f64) {
        if !self.points.is_empty() {
            self.points.push(' ');
        }
        self.value(x);
        self.points.push(',');
        self.value(y);
    }

    /// Add a point to the polygon/polyline
    pub fn add<P, V>(&mut self, p: P) -> &mut Self
    where
        P: Into<(V, V)>,
        V: Into<f64> + Copy,
    {
        let p = p.into();
        let (x, y) = (p.0.into(), p.1.into());
        self.point(x, y);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let poly = PolyPointBuilder::new();
        assert_eq!(poly.to_string(), "");
    }

    #[test]
    fn mv() {
        let mut poly = PolyPointBuilder::new();
        poly.add([1, 2]);
        assert_eq!(poly.to_string(), "1,2");
    }

    #[test]
    fn line() {
        let mut poly = PolyPointBuilder::new();
        poly.add([1, 2]);
        poly.add([2, 1]);
        assert_eq!(poly.to_string(), "1,2 2,1");
    }

    #[test]
    fn rounding() {
        let mut poly = PolyPointBuilder::new();
        poly.add([2.0001, 0.003]);
        assert_eq!(poly.to_string(), "2,0");
    }

    #[test]
    fn two_decimal_places() {
        let mut poly = PolyPointBuilder::new();
        poly.add([2.2222, 9.994]);
        poly.add([4.444444, 8.88888]);
        assert_eq!(poly.to_string(), "2.22,9.99 4.44,8.89");
    }

    #[test]
    fn three_decimal_places() {
        let mut poly = PolyPointBuilder::new();
        poly.precision(3);
        poly.add([2.2222, 9.994]);
        poly.add([4.444444, 8.88888]);
        poly.add([5.444444, 8.88888]);
        assert_eq!(poly.to_string(), "2.222,9.994 4.444,8.889 5.444,8.889");
    }
}
