use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::intersects::Intersects;

pub fn intersect(a: &Vec<geo::Polygon<f64>>, b: &Vec<geo::Polygon<f64>>) {
    let mut intersects = 0;
    let mut non_intersects = 0;

    let mut a_index = 0;
    while a_index < a.len() {
        let mut b_index = 0;
        while b_index < b.len() {
            if a[a_index].intersects(&b[b_index]) {
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
                if a[a_index].intersects(&b[b_index]) {
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
