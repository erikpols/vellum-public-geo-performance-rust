use geo_clipper::Clipper;
// use std::convert::TryFrom;
// use geo_types::*;
// use geo::Polygon;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::area::*;
use geo::algorithm::intersects::Intersects;

pub fn intersect(a: &Vec<geo::Polygon<f64>>, b: &Vec<geo::Polygon<f64>>) {
    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            let res = a[a_index].intersection(&b[b_index], 1000000.0);

            if res.signed_area() > 0. {
                intersects += 1;
            } else {
                non_intersects += 1;
            }

            b_index += 1;
        }
        a_index += 1;
    }
    println!(
        "Intersection tests: {:?} Intersections: {:?} Non-intersections: {:?}",
        intersects + non_intersects,
        intersects,
        non_intersects
    );
}
pub fn bbox_intersect(a: &Vec<geo::Polygon<f64>>, b: &Vec<geo::Polygon<f64>>) {
    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let a_bbox = a[a_index].bounding_rect().unwrap();
        let mut b_index = 0;
        while b_index < b.len() {
            let b_bbox = b[b_index].bounding_rect().unwrap();

            if a_bbox.intersects(&b_bbox) {
                let res = a[a_index].intersection(&b[b_index], 1000000.0);

                if res.signed_area() > 0. {
                    intersects += 1;
                } else {
                    non_intersects += 1;
                }
            } else {
                non_intersects += 1;
            }

            b_index += 1;
        }
        a_index += 1;
    }
    println!(
        "Intersection tests: {:?} Intersections: {:?} Non-intersections: {:?}",
        intersects + non_intersects,
        intersects,
        non_intersects
    );
}
