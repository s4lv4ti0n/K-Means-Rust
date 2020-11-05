use std::path::Path;
use image::{RgbImage};
use rand::Rng;

fn init(k_value: usize, input_path: &str) {
    if k_value == 0 { return; }

    let img = image::open(&Path::new(input_path)).unwrap().to_rgb();
    let (x, y) = img.dimensions();
    let mut img_seg: RgbImage = img.clone();
    let mut centroids: Vec<[u8; 3]> = Vec::with_capacity(k_value);
    let mut centroids_cumulated: Vec<[u32; 4]> = Vec::with_capacity(k_value);

    // Init with random centroids
    for _i in 0..k_value {
        centroids.push(img.get_pixel(
            rand::thread_rng().gen_range(0, x),
            rand::thread_rng().gen_range(0, y)
        ).0);
        centroids_cumulated.push([0; 4]);
    }

    // Clustering
    let mut iterations = 0;
    loop {
        //Prepare for average calculation
        for c in centroids_cumulated.iter_mut() {
            c[0] = 0;
            c[1] = 0;
            c[2] = 0;
            c[3] = 0;
        }
        for y_i in 0..y {
            for x_i in 0..x {
                let pixel = img.get_pixel(x_i, y_i);
                let px= pixel.0;
                let mut prev_distance = euclidean_distance(&px, &centroids[0]);
                let mut candidate_i = 0;
                for (i, centroid) in centroids.iter().enumerate().skip(1) {
                    let distance = euclidean_distance(&px, &centroid);
                    if distance < prev_distance {
                        candidate_i = i;
                        prev_distance = distance;
                    }
                }
                img_seg.get_pixel_mut(x_i, y_i).0[0] = centroids[candidate_i][0];
                img_seg.get_pixel_mut(x_i, y_i).0[1] = centroids[candidate_i][1];
                img_seg.get_pixel_mut(x_i, y_i).0[2] = centroids[candidate_i][2];
                cumulate(&mut centroids_cumulated[candidate_i], &px);
            }
            if y_i % 20 == 0 {
                img_seg.save(Path::new(&format!("./images/{}_{}.jpg", iterations, frmt(y_i)))).expect("idk");
            }
        }
        for i in 0..k_value {
            avg(&mut centroids_cumulated[i], &mut centroids[i]);
        }
        println!("iteration: {}, centroids: {:?}", iterations, centroids);
        iterations += 1;
    }
}

fn euclidean_distance(vec1: &[u8; 3], vec2: &[u8; 3]) -> f32 {
    ((vec1[0] as f32 - vec2[0] as f32).powf(2f32) +
    (vec1[1] as f32 - vec2[1] as f32).powf(2f32) +
    (vec1[2] as f32 - vec2[2] as f32).powf(2f32)).sqrt()
}

fn cumulate(vec1: &mut [u32; 4], vec2: &[u8; 3]) {
    vec1[0] += vec2[0] as u32;
    vec1[1] += vec2[1] as u32;
    vec1[2] += vec2[2] as u32;
    vec1[3] += 1;
}

fn avg(vec1: &mut [u32; 4], vec2: &mut [u8; 3]) {
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
