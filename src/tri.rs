use ultraviolet::Vec2;

use crate::line::Line;


pub struct Tri<T> where
    T: std::ops::AddAssign<T>,
 {
    long: Line<T>,
    short1: Line<T>,
    short2: Line<T>,
    cursor: Line<T>,
}

impl<T> Tri<T> where 
    T: std::ops::AddAssign<T>,
    T: Clone,
    T: std::ops::Div<f32, Output = T>,
    T: std::ops::Sub<T, Output = T>,
{
    pub fn new(mut points: [(Vec2, T); 3]) -> Self {
        // first, we order the points.
        // we need to know which one is at the top, which one is at the bottom, and which one is in the middle;
        points.sort_by(|a, b| a.0.y.partial_cmp(&b.0.y).unwrap());
        let [top, mid, bottom] = points;
        let mut long = Line::from(top.clone(), bottom.clone());
        let mut short1 = Line::from(top, mid.clone());
        let short2 = Line::from(mid, bottom);

        let cursor = Line::from(
            long.next().unwrap(),
            short1.next().unwrap(),
        );

        Tri {long, short1, short2, cursor}
    }
}


impl<T> Iterator for Tri<T>
where
    T: std::ops::AddAssign<T>,
    T: std::ops::Div<f32, Output = T>,
    T: std::ops::Sub<T, Output = T>,
    T: Clone
{
    type Item = (Vec2, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.cursor.next() {
            return Some(p);
        };

        let n1 = self.long.next()?;
        let n2 = if let Some(p) = self.short1.next() {
            p
        } else {
            self.short2.next()?
        };

        self.cursor = Line::from(n1, n2);

        self.cursor.next()
    }
}

