use super::Vec3;
use std::cmp::{Eq, Ord, Ordering, PartialOrd};

//    dead class

#[derive(Debug)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
    pub pixel_color: Vec3,
}

impl Pixel {
    pub fn color(&self) -> Vec3 {
        self.pixel_color
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y.cmp(&other.y) == Ordering::Equal {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y.cmp(&other.y) == Ordering::Equal {
            Some(self.y.cmp(&other.y))
        } else {
            Some(self.x.cmp(&other.x))
        }
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pixel {}
