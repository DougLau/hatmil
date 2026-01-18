// definition.rs
// Copyright (C) 2025-2026  Douglas P Lau
//
use std::fmt;
use std::fmt::Write;

/// SVG [Path] definition builder
///
/// ```rust
/// # use hatmil::svg::Path;
/// let mut path = Path::def_builder();
/// path.precision(3);
/// path.move_to([5, 5]);
/// path.line((10.1, 20.2));
/// path.cubic(None, (20, 25), (50, 55));
/// path.close();
/// println!("{path}");
/// ```
///
/// [Path]: svg/struct.Path.html#method.d
#[derive(Clone)]
pub struct PathDefBuilder {
    /// Absolute vs. relative output mode
    absolute: bool,
    /// Precision in decimal places
    precision: usize,
    /// Start X value
    x0: f64,
    /// Start Y value
    y0: f64,
    /// Current pen X value
    x: f64,
    /// Current pen Y value
    y: f64,
    /// Definition string
    d: String,
}

impl fmt::Display for PathDefBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.d)?;
        Ok(())
    }
}

impl From<PathDefBuilder> for String {
    fn from(path: PathDefBuilder) -> Self {
        // zero-copy alternative to fmt::Display
        path.d
    }
}

impl PathDefBuilder {
    /// Create a new SVG path definition builder
    pub(crate) fn new() -> Self {
        PathDefBuilder {
            absolute: false,
            precision: 2,
            x0: 0.0,
            y0: 0.0,
            x: 0.0,
            y: 0.0,
            d: String::new(),
        }
    }

    /// Set absolute or relative output mode
    pub fn absolute(&mut self, absolute: bool) -> &mut Self {
        self.absolute = absolute;
        self
    }

    /// Set the precision in decimal places
    pub fn precision(&mut self, digits: usize) -> &mut Self {
        self.precision = digits;
        self
    }

    /// Check if two values are equal with specified precision
    fn value_eq(&self, v1: f64, v2: f64) -> bool {
        let mut s1 = String::with_capacity(16);
        let mut s2 = String::with_capacity(16);
        write!(&mut s1, "{v1:.0$}", self.precision).unwrap();
        write!(&mut s2, "{v2:.0$}", self.precision).unwrap();
        s1 == s2
    }

    /// Write one value
    fn value(&mut self, v: f64) {
        write!(&mut self.d, "{v:.0$}", self.precision).unwrap();
        if self.precision > 0 {
            while self.d.ends_with('0') {
                self.d.pop();
            }
            if self.d.ends_with('.') {
                self.d.pop();
            }
        }
    }

    /// Write one point
    fn point(&mut self, x: f64, y: f64) {
        self.value(x);
        self.d.push(' ');
        self.value(y);
    }

    /// Close the current subpath
    pub fn close(&mut self) -> &mut Self {
        self.d.push('z');
        (self.x, self.y) = (self.x0, self.y0);
        self
    }

    /// Move to a point, starting a new subpath
    pub fn move_to<P, V>(&mut self, p: P) -> &mut Self
    where
        P: Into<(V, V)>,
        V: Into<f64> + Copy,
    {
        let p = p.into();
        let (mut x, mut y) = (p.0.into(), p.1.into());
        if self.absolute {
            self.d.push('M');
        } else {
            self.d.push('m');
            x -= self.x;
            y -= self.y;
        }
        self.point(x, y);
        (self.x, self.y) = (p.0.into(), p.1.into());
        (self.x0, self.y0) = (self.x, self.y);
        self
    }

    /// Draw a line to the given point
    pub fn line<P, V>(&mut self, p: P) -> &mut Self
    where
        P: Into<(V, V)>,
        V: Into<f64> + Copy,
    {
        let p = p.into();
        let (mut x, mut y) = (p.0.into(), p.1.into());
        let x_same = self.value_eq(x, self.x);
        let y_same = self.value_eq(y, self.y);
        if !self.absolute {
            x -= self.x;
            y -= self.y;
        }
        match (x_same, y_same) {
            (true, false) => {
                self.d.push(if self.absolute { 'V' } else { 'v' });
                self.value(y);
            }
            (false, true) => {
                self.d.push(if self.absolute { 'H' } else { 'h' });
                self.value(x);
            }
            _ => {
                self.d.push(if self.absolute { 'L' } else { 'l' });
                self.point(x, y);
            }
        }
        (self.x, self.y) = (p.0.into(), p.1.into());
        self
    }

    /// Draw a cubic Bézier curve
    pub fn cubic<P, V>(&mut self, p1: Option<P>, p2: P, p: P) -> &mut Self
    where
        P: Into<(V, V)>,
        V: Into<f64> + Copy,
    {
        let p2 = p2.into();
        let (mut x2, mut y2) = (p2.0.into(), p2.1.into());
        let p = p.into();
        let (mut x, mut y) = (p.0.into(), p.1.into());
        match p1 {
            Some(p1) => {
                let p1 = p1.into();
                let (mut x1, mut y1) = (p1.0.into(), p1.1.into());
                if self.absolute {
                    self.d.push('C');
                } else {
                    self.d.push('c');
                    x1 -= self.x;
                    y1 -= self.y;
                    x2 -= self.x;
                    y2 -= self.y;
                    x -= self.x;
                    y -= self.y;
                }
                self.point(x1, y1);
                self.d.push(' ');
                self.point(x2, y2);
                self.d.push(' ');
                self.point(x, y);
            }
            None => {
                if self.absolute {
                    self.d.push('S');
                } else {
                    self.d.push('s');
                    x2 -= self.x;
                    y2 -= self.y;
                    x -= self.x;
                    y -= self.y;
                }
                self.point(x2, y2);
                self.d.push(' ');
                self.point(x, y);
            }
        }
        (self.x, self.y) = (p.0.into(), p.1.into());
        self
    }

    /// Draw a quadratic Bézier curve
    pub fn quad<P, V>(&mut self, p1: Option<P>, p: P) -> &mut Self
    where
        P: Into<(V, V)>,
        V: Into<f64> + Copy,
    {
        let p = p.into();
        let (mut x, mut y) = (p.0.into(), p.1.into());
        match p1 {
            Some(p1) => {
                let p1 = p1.into();
                let (mut x1, mut y1) = (p1.0.into(), p1.1.into());
                if self.absolute {
                    self.d.push('Q');
                } else {
                    self.d.push('q');
                    x1 -= self.x;
                    y1 -= self.y;
                    x -= self.x;
                    y -= self.y;
                }
                self.point(x1, y1);
                self.d.push(' ');
                self.point(x, y);
            }
            None => {
                if self.absolute {
                    self.d.push('T');
                } else {
                    self.d.push('t');
                    x -= self.x;
                    y -= self.y;
                }
                self.point(x, y);
            }
        }
        (self.x, self.y) = (p.0.into(), p.1.into());
        self
    }

    /// Draw an elliptical arc
    pub fn arc<P, V>(
        &mut self,
        rx: V,
        ry: V,
        angle: V,
        large_arc: bool,
        sweep: bool,
        p: P,
    ) -> &mut Self
    where
        P: Into<(V, V)>,
        V: Into<f64> + Copy,
    {
        let rx = rx.into();
        let ry = ry.into();
        let angle = angle.into();
        let p = p.into();
        let (mut x, mut y) = (p.0.into(), p.1.into());
        if self.absolute {
            self.d.push('A');
        } else {
            self.d.push('a');
            x -= self.x;
            y -= self.y;
        }
        self.value(rx);
        self.d.push(' ');
        self.value(ry);
        self.d.push(' ');
        self.value(angle);
        self.d.push(' ');
        self.d.push(if large_arc { '1' } else { '0' });
        self.d.push(' ');
        self.d.push(if sweep { '1' } else { '0' });
        self.d.push(' ');
        self.point(x, y);
        (self.x, self.y) = (p.0.into(), p.1.into());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let path = PathDefBuilder::new();
        assert_eq!(path.to_string(), "");
    }

    #[test]
    fn mv() {
        let mut path = PathDefBuilder::new();
        path.move_to([1, 2]);
        assert_eq!(path.to_string(), "m1 2");
    }

    #[test]
    fn line() {
        let mut path = PathDefBuilder::new();
        path.line([2, 1]);
        assert_eq!(path.to_string(), "l2 1");
    }

    #[test]
    fn horizontal() {
        let mut path = PathDefBuilder::new();
        path.line([2.0001, 0.003]);
        assert_eq!(path.to_string(), "h2");
    }

    #[test]
    fn vertical() {
        let mut path = PathDefBuilder::new();
        path.line([0, -6]);
        assert_eq!(path.to_string(), "v-6");
    }

    #[test]
    fn cubic() {
        let mut path = PathDefBuilder::new();
        path.cubic(Some([1, 0]), [5, 5], [0, 10]);
        assert_eq!(path.to_string(), "c1 0 5 5 0 10");
    }

    #[test]
    fn cubic_smooth() {
        let mut path = PathDefBuilder::new();
        path.cubic(None, [5, 5], [0, 10]);
        assert_eq!(path.to_string(), "s5 5 0 10");
    }

    #[test]
    fn quad() {
        let mut path = PathDefBuilder::new();
        path.quad(Some([1, 0]), [0, 10]);
        assert_eq!(path.to_string(), "q1 0 0 10");
    }

    #[test]
    fn quad_smooth() {
        let mut path = PathDefBuilder::new();
        path.precision(0);
        path.quad(None, [0.4, 9.6]);
        assert_eq!(path.to_string(), "t0 10");
    }

    #[test]
    fn arc() {
        let mut path = PathDefBuilder::new();
        path.arc(20, 25, 90, true, false, [50, 10]);
        assert_eq!(path.to_string(), "a20 25 90 1 0 50 10");
    }

    #[test]
    fn relative() {
        let mut path = PathDefBuilder::new();
        path.line([2, 4]);
        path.line([4, 2]);
        assert_eq!(path.to_string(), "l2 4l2 -2");
    }

    #[test]
    fn two_decimal_places() {
        let mut path = PathDefBuilder::new();
        path.absolute(true);
        path.line([2.2222, 9.994]);
        path.line([4.444444, 8.88888]);
        assert_eq!(path.to_string(), "L2.22 9.99L4.44 8.89");
    }

    #[test]
    fn three_decimal_places() {
        let mut path = PathDefBuilder::new();
        path.precision(3);
        path.line([2.2222, 9.994]);
        path.line([4.444444, 8.88888]);
        path.line([5.444444, 8.88888]);
        assert_eq!(path.to_string(), "l2.222 9.994l2.222 -1.105h1");
    }

    #[test]
    fn close_move() {
        let mut path = PathDefBuilder::new();
        path.move_to([0, 0]);
        path.line([5, 5]);
        path.line([5, 0]);
        path.close();
        path.move_to([10, 0]);
        path.line([15, 5]);
        path.line([15, 0]);
        path.close();
        assert_eq!(String::from(path), "m0 0l5 5v-5zm10 0l5 5v-5z");
    }

    #[test]
    fn move_move() {
        let mut path = PathDefBuilder::new();
        path.move_to([0, 0]);
        path.line([5, 5]);
        path.line([5, 0]);
        path.move_to([10, 0]);
        path.line([15, 5]);
        path.line([15, 0]);
        path.close();
        assert_eq!(String::from(path), "m0 0l5 5v-5m5 0l5 5v-5z");
    }
}
