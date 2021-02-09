use super::utils;
use super::Arc;
use super::HitRecord;
use super::Hittable;
use super::Ray;
use super::AABB;
use std::f64;

/*
    UNTESTED
*/

struct BVHNode {
    node_box: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(node_box: AABB, left: Arc<dyn Hittable>, right: Arc<dyn Hittable>) -> Self {
        BVHNode {
            node_box,
            left,
            right,
        }
    }
    pub fn construct_tree(
        mut hittable_list: &mut Vec<Arc<dyn Hittable>>,
        t0: f64,
        t1: f64,
        start: usize,
        end: usize,
    ) -> Arc<dyn Hittable> {
        let axis = utils::random_int(0, 2);
        let size = hittable_list.len();
        let (right, left) = match hittable_list.len() {
            1 => (hittable_list[start].clone(), hittable_list[start].clone()),
            2 => {
                //comparator
                (
                    hittable_list[start].clone(),
                    hittable_list[start + 1].clone(),
                )
            }
            _ => {
                hittable_list.sort_by(|a, b| {
                    let box_a = a.bounding_box(0., 0.);
                    let box_b = b.bounding_box(0., 0.);
                    match (box_a, box_b) {
                        (Some(ba), Some(bb)) => ba
                            .min()
                            .index(axis)
                            .partial_cmp(&bb.min().index(axis))
                            .unwrap(),
                        (_, _) => panic!("no bounding boxes"),
                    }
                });
                let mid = start + size / 2;
                (
                    BVHNode::construct_tree(&mut hittable_list, t0, t1, start, mid),
                    BVHNode::construct_tree(&mut hittable_list, t0, t1, mid, end),
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
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.node_box.hit(r, t_min, t_max) {
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
        Some(self.node_box)
    }
}
