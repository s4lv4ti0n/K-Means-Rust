use std::{path::Path, env};
use image::{Rgb, ImageBuffer};
use rand::Rng;

fn k_means(k_value: usize, input_path: &Path, save_artefacts: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img = image::open(input_path).unwrap().to_rgb();
    let (x, y) = img.dimensions();
    let mut img_seg = img.clone();
    let mut centroids: Vec<Rgb<u8>> = Vec::with_capacity(k_value);
    let mut centroids_cumulated: Vec<(Rgb<u32>, u32)>;

    // Init with random centroids
    for _i in 0..k_value {
        centroids.push(img.get_pixel(
            rand::thread_rng().gen_range(0, x),
            rand::thread_rng().gen_range(0, y)
        ).clone());
    }

    // Clustering
    let mut iteration_counter = 0;
    loop {
        centroids_cumulated = vec![(Rgb([0,0,0]), 0); k_value];
        for y_i in 0..y {
            for x_i in 0..x {
                let pixel = img.get_pixel(x_i, y_i);
                let mut distance = std::f32::MAX;
                let mut candidate_i = 0;
                for (i, centroid) in centroids.iter().enumerate() {
                    let tmp = euclidean_distance(&pixel, &centroid);
                    if tmp < distance {
                        candidate_i = i;
                        distance = tmp;
                    }
                }
                img_seg.put_pixel(x_i, y_i, centroids[candidate_i]);
                cumulate(&mut centroids_cumulated[candidate_i], &pixel);
            }
            if y_i % 20 == 0 && save_artefacts {
                img_seg.save(
                    Path::new(&format!("./images/{}_{}.jpg", frmt(iteration_counter), frmt(y_i))))
                .expect("idk");
            }
        }
        avg(&mut centroids_cumulated);
        if are_same(&mut centroids, &centroids_cumulated) {
            return img_seg;
        } else {
            println!("iteration: {}, centroids: {:?}", iteration_counter, centroids);
            iteration_counter += 1;
        }
    }
}

fn euclidean_distance(pixel1: &Rgb<u8>, pixel2: &Rgb<u8>) -> f32 {
    ((pixel1.0[0] as f32 - pixel2.0[0] as f32).powf(2f32) +
    (pixel1.0[1] as f32 - pixel2.0[1] as f32).powf(2f32) +
    (pixel1.0[2] as f32 - pixel2.0[2] as f32).powf(2f32)).sqrt()
}

fn cumulate(centroid: &mut (Rgb<u32>, u32), pixel: &Rgb<u8>) {
    centroid.0.0[0] += pixel[0] as u32;
    centroid.0.0[1] += pixel[1] as u32;
    centroid.0.0[2] += pixel[2] as u32;
    centroid.1 += 1;
}

fn avg(centroids_cumulated: &mut Vec<(Rgb<u32>, u32)>) {
    for centroid in centroids_cumulated {
        for i in 0..3 {
            centroid.0[i] = centroid.0[i]/centroid.1;
        }
    }
}

fn are_same(centroids: &mut Vec<Rgb<u8>>, centroids_cumulated: &Vec<(Rgb<u32>, u32)>) -> bool {
    let mut same = true;
    for i in 0..centroids.len() {
        for j in 0..3 {
            if centroids[i].0[j] != centroids_cumulated[i].0[j] as u8 {
                same = false;
            }
            centroids[i].0[j] = centroids_cumulated[i].0[j] as u8
        }
    }
    return same;
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
    let args: Vec<String> = env::args().collect();
    let k: usize = args[1].parse().unwrap();
    let input = &args[2];
    let output = &args[3];
    let artefacts: bool = args[4].parse().unwrap();

    if k > 0 {
        k_means(k, Path::new(&input), artefacts)
            .save(Path::new(&output))
            .expect("idk");
    } else {
        println!("k-value must be greater than 0");
    }
}
