use ultraviolet::Vec2;

pub struct Line<T>
where
    T: std::ops::AddAssign<T>,
{
    position: Vec2,
    metadata: T,
    offset: (Vec2, T),
    step_count: u32,
}

impl<T> Iterator for Line<T>
where
    T: std::ops::AddAssign<T>,
    T: Clone,
{
    type Item = (Vec2, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_count == 0 {
            return None;
        }

        let n = (self.position, self.metadata.clone());
        self.position += self.offset.0;
        self.metadata += self.offset.1.clone();
        self.step_count -= 1;
        Some(n)
    }
}

impl<T> Line<T> where
    T: std::ops::AddAssign<T>,
    T: Clone,
    T: std::ops::Div<f32, Output = T>,
    T: std::ops::Sub<T, Output = T>,
{
   pub fn from((av, ai): (Vec2, T), (bv, bi): (Vec2, T)) -> Self {
    // find out, along which axis lies the greatest difference between a and b.
    let diff = if (av.x - bv.x).abs() >= (av.y - bv.y) {
        // x axis has a greater difference.
        av.x - bv.x
    } else {
        av.y - bv.y
    };

    let step_count = diff.floor().abs() as u32;

    let offset = (ai.clone() - bi) / diff;
    let offset_vec = (av - bv) / diff;
    let offset = (offset_vec, offset);

    // starting condition is the same as a.
    let position = av;
    let metadata = ai;

    Line {
        metadata,
        offset,
        position,
        step_count
    }
   }
}
