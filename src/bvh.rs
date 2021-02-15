use super::utils;
use super::Arc;
use super::HitRecord;
use super::Ray;
use super::AABB;
use super::{Hittable, HittableList};
use std::{cmp::Ordering, f64};

pub struct BVHNode {
    root_box: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    fn new(root_box: AABB, left: Arc<dyn Hittable>, right: Arc<dyn Hittable>) -> Self {
        BVHNode {
            root_box,
            left,
            right,
        }
    }

    pub fn construct_tree(hitlist: HittableList, t0: f64, t1: f64) -> Arc<dyn Hittable> {
        assert_ne!(hitlist.len(), 0);
        let mut objects = hitlist.list().clone();
        let axis = utils::random_int(0, 2);

        let comp_fn = match axis {
            0 => Self::comparator_x,
            1 => Self::comparator_y,
            2 => Self::comparator_z,
            _ => panic!("axis out of bounds"),
        };

        let start = 0;

        let (right, left) = match objects.len() {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => {
                assert_eq!(start, 0);
                if comp_fn(&objects[start], &objects[start + 1]) == Ordering::Less {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            _ => {
                objects.sort_by(comp_fn);
                let mid = start + objects.len() / 2;
                let objects_split = objects.split_off(mid);
                let (hitlist_left, hitlist_right) =
                    (HittableList::new(objects), HittableList::new(objects_split));
                (
                    BVHNode::construct_tree(hitlist_left, t0, t1),
                    BVHNode::construct_tree(hitlist_right, t0, t1),
                )
            }
        };

        Arc::new(BVHNode::new(
            AABB::surrounding_box(
                left.bounding_box(t0, t1).unwrap(),
                right.bounding_box(t0, t1).unwrap(),
            ),
            left,
            right,
        ))
    }

    fn comparator(axis: i32, a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        let (box_a, box_b) = (a.bounding_box(0., 0.), b.bounding_box(0., 0.));

        assert_eq!(true, box_a.is_some());
        assert_eq!(true, box_b.is_some());

        box_a.unwrap().min()[axis as usize]
            .partial_cmp(&box_b.unwrap().min()[axis as usize])
            .unwrap()
    }
    fn comparator_x(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::comparator(0, a, b)
    }
    fn comparator_y(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::comparator(1, a, b)
    }
    fn comparator_z(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::comparator(2, a, b)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.root_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(&r, t_min, t_max);
        let hit_right = self.right.hit(&r, t_min, t_max);

        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(hit), None) => Some(hit),
            (None, Some(hit)) => Some(hit),
            (Some(l_hit), Some(r_hit)) => {
                if l_hit.t < r_hit.t {
                    Some(l_hit)
                } else {
                    Some(r_hit)
                }
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.root_box)
    }
}
