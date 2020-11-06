use std::path::Path;
use image::{RgbImage, Rgb};
use rand::Rng;

struct CentroidSet {
    items: Vec<Rgb<usize>>,
    counter: usize,
}
impl CentroidSet {
    fn new(size: usize) -> CentroidSet {
        CentroidSet {
            items: Vec::with_capacity(size),
            counter: 0
        }
    }

    fn add(&mut self, pixel: &Rgb<u8>) {
        self.items.push(Rgb([
            pixel.0[0] as usize,
            pixel.0[1] as usize,
            pixel.0[2] as usize,
        ]));
    }

    fn cumulate(&mut self, centroid: Rgb<usize>) {
        for j in 0..3 {
            self.items[i].0[j] += centroid.items[i].0[j];
        }
        self.counter += 1;
    }

    fn is_equal(&self, centroid: &CentroidSet) -> bool {
        for i in 0..self.items.len() {
            for j in 0..3 {
                if self.items[i].0[j] != centroid.items[i].0[j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn calculate_avg(&mut self) {
        for i in 0..self.items.len() {
            for j in 0..3 {
                self.items[i].0[j] = self.items[i].0[j]/self.counter;
            }
        }
        self.counter = 0;
    }

    fn reset(&mut self) {
        for i in 0..self.items.len() {
            for j in 0..3 {
                self.items[i].0[j] = 0;
            }
        }
    }

    fn get_closest_centroid(&self, pixel: &Rgb<u8>) -> Rgb<usize> {
        let distance = std::i32::MAX;
        let closest: Rgb<usize> = Rgb([0,0,0]);

        for cluster in self.items {
            let tmp = 0.0;
            for (i, value) in cluster.0.iter().enumerate() {
                tmp += (*value as f32 - pixel.0[i] as f32).powf(2f32);
            }
            let tmp = tmp.sqrt() as i32;
            if tmp < distance {
                distance = tmp;
                closest = cluster;
            }
        }
        return closest;
    }

    fn clone(&self) -> Self {
        *self
    }
}


//TODO: NEEDS REFACTOR
fn init(k_value: usize, input_path: &str) {
    if k_value == 0 { return; }

    let img = image::open(&Path::new(input_path)).unwrap().to_rgb();
    let (x, y) = img.dimensions();
    let mut img_seg: RgbImage = img.clone();

    let centr = CentroidSet::new(k_value);
    let centrtmp = CentroidSet::new(k_value);

    // Init with random centroids
    for _i in 0..k_value {
        centr.add(img.get_pixel(
            rand::thread_rng().gen_range(0, x),
            rand::thread_rng().gen_range(0, y)
        ));
    }

    // Clustering
    let mut iterations = 0;
    while iterations < 4 {
        //Prepare for average calculation
        centrtmp.reset();
        for y_i in 0..y {
            for x_i in 0..x {
                let pixel = img.get_pixel(x_i, y_i);
                let centroid = centr.get_closest_centroid(pixel);

                img_seg.put_pixel(x_i, y_i, Rgb(centroids[candidate_i]));
                cumulate(&mut centroids_cumulated[candidate_i], &px);
            }
            if y_i % 20 == 0 {
                img_seg.save(Path::new(&format!("./images/{}_{}.jpg", frmt(iterations), frmt(y_i)))).expect("idk");
            }
        }
        for i in 0..k_value {
            avg(&mut centroids_cumulated[i], &mut centroids[i]);
        }
        println!("iteration: {}, centroids: {:?}", iterations, centroids);
        iterations += 1;
    }
}

fn avg(vec1: &mut [usize; 4], vec2: &mut [u8; 3]) {
    vec2[0] = (vec1[0] / vec1[3]) as u8;
    vec2[1] = (vec1[1] / vec1[3]) as u8;
    vec2[2] = (vec1[2] / vec1[3]) as u8;
}

fn frmt(num: u32) -> String {
    if num < 10 {
        return format!("000{}", num)
    } else if num < 100 {
        return format!("00{}", num)
    } else if num < 1000 {
        return format!("0{}", num)
    } else {
        return format!("{}", num)
    }
}

fn main() {
    init(4, "./src/me.jpg");
}
