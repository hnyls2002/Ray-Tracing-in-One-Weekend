use crate::basic::{random_double_unit, vec3::Vec3};

use super::PDF;

pub struct MixturePDF<TP0, TP1>
where
    TP0: PDF,
    TP1: PDF,
{
    pub p: (TP0, TP1),
}

impl<TP0, TP1> MixturePDF<TP0, TP1>
where
    TP0: PDF,
    TP1: PDF,
{
    pub fn new(p0: TP0, p1: TP1) -> MixturePDF<TP0, TP1>
    where
        TP0: PDF,
        TP1: PDF,
    {
        MixturePDF { p: (p0, p1) }
    }
}

impl<TP0, TP1> PDF for MixturePDF<TP0, TP1>
where
    TP0: PDF,
    TP1: PDF,
{
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p.0.value(direction) + 0.5 * self.p.1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double_unit() < 0.5 {
            self.p.0.generate()
        } else {
            self.p.1.generate()
        }
    }
}
