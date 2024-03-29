use std::{collections::HashMap, time::Instant};

use rand::{Rng, SeedableRng};

use super::{
    quantizer::QuantizerResult,
    src::{point_provider::PointProvider, point_provider_lab::PointProviderLab},
};

#[derive(PartialEq, PartialOrd)]
pub struct DistanceAndIndex {
    pub distance: f64,
    pub index: i64,
}
impl DistanceAndIndex {
    pub fn new(distance: f64, index: i64) -> DistanceAndIndex {
        DistanceAndIndex { distance, index }
    }
}
impl Eq for DistanceAndIndex {}
impl Ord for DistanceAndIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.distance < other.distance {
            return std::cmp::Ordering::Less;
        } else if self.distance > other.distance {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}

/*
class QuantizerWsmeans {*/
pub struct QuantizerWsmeans {}
impl QuantizerWsmeans {
    const DEBUG: bool = false;

    pub fn debug_log(log: String) {
        if Self::DEBUG {
            println!("{}", log);
        }
    }

    pub fn quantize(
        input_pixels: &Vec<i64>,
        max_colors: i64,
        starting_clusters: Option<&Vec<i64>>,
        point_provider: Option<&PointProviderLab>,
        max_iterations: Option<i64>,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult {
        let starting_clusters_default = Vec::new();
        let starting_clusters = starting_clusters.unwrap_or(&starting_clusters_default);
        let point_provider_default = PointProviderLab::new();
        let point_provider = point_provider.unwrap_or(&point_provider_default);
        let max_iterations = max_iterations.unwrap_or(5);
        let return_input_pixel_to_cluster_pixel =
            return_input_pixel_to_cluster_pixel.unwrap_or(false);
        //

        let mut pixel_to_count: HashMap<i64, i64> = HashMap::new();
        let mut points: Vec<Vec<f64>> = Vec::new();
        let mut pixels: Vec<i64> = Vec::new();
        let mut point_count = 0;
        for input_pixel in input_pixels {
            let pixel_count = pixel_to_count
                .entry(*input_pixel)
                .and_modify(|value| *value += 1)
                .or_insert(1);
            if pixel_count.clone() == 1 {
                point_count += 1;
                points.push(point_provider.from_int(*input_pixel));
                pixels.push(*input_pixel);
            }
        }

        let mut counts: Vec<i64> = vec![0; point_count];
        for i in 0..point_count {
            let pixel = pixels[i];
            let count = pixel_to_count.get(&pixel).unwrap();
            counts[i] = *count;
        }

        let cluster_count = max_colors.min(point_count as i64);

        let mut clusters: Vec<Vec<f64>> = starting_clusters
            .iter()
            .map(|e| point_provider.from_int(*e))
            .collect();
        let additional_clusters_needed = cluster_count - clusters.len() as i64;
        if additional_clusters_needed > 0 {
            // let random = math.Random(0x42688);
            let mut random = rand::rngs::StdRng::seed_from_u64(0x42688);
            let mut indices: Vec<i64> = Vec::new();
            for _ in 0..additional_clusters_needed {
                // Use existing points rather than generating random centroids.
                //
                // KMeans is extremely sensitive to initial clusters. This quantizer
                // is meant to be used with a Wu quantizer that provides initial
                // centroids, but Wu is very slow on unscaled images and when extracting
                // more than 256 colors.
                //
                // Here, we can safely assume that more than 256 colors were requested
                // for extraction. Generating random centroids tends to lead to many
                // "empty" centroids, as the random centroids are nowhere near any pixels
                // in the image, and the centroids from Wu are very refined and close
                // to pixels in the image.
                //
                // Rather than generate random centroids, we'll pick centroids that
                // are actual pixels in the image, and avoid duplicating centroids.

                //let index = random.nextInt(points.length);
                let mut index = random.gen_range(0..points.len());
                while indices.contains(&(index as i64)) {
                    index = random.gen_range(0..points.len());
                }
                indices.push(index as i64);
            }
            for index in indices {
                clusters.push(points[index as usize].iter().map(|it| *it).collect());
            }
        }
        Self::debug_log(format!(
            "have {} starting clusters, {} points",
            clusters.len(),
            points.len()
        ));
        let mut cluster_indices: Vec<i64> = (0..point_count)
            .map(|index| index as i64 % cluster_count)
            .collect();
        let mut index_matrix: Vec<Vec<i64>> = (0..cluster_count)
            .map(|_| vec![0; cluster_count as usize])
            .collect();
        let mut distance_to_index_matrix: Vec<Vec<DistanceAndIndex>> = (0..cluster_count)
            .map(|_| {
                (0..cluster_count)
                    .map(|index| DistanceAndIndex::new(0.0, index))
                    .collect()
            })
            .collect();
        let mut pixel_count_sums: Vec<i64> = vec![0; cluster_count as usize];
        for iteration in 0..max_iterations {
            if Self::DEBUG {
                for i in 0..cluster_count {
                    pixel_count_sums[i as usize] = 0;
                }
                for i in 0..point_count {
                    let cluster_index = cluster_indices[i];
                    let count = counts[i];
                    pixel_count_sums[cluster_index as usize] += count;
                }
                let mut empty_clusters = 0;
                for cluster in 0..cluster_count {
                    if pixel_count_sums[cluster as usize] == 0 {
                        empty_clusters += 1;
                    }
                }
                Self::debug_log(format!(
                    "starting iteration {}; {} clusters are empty of {}",
                    iteration + 1,
                    empty_clusters,
                    cluster_count
                ));
            }

            let mut points_moved = 0;
            for i in 0..cluster_count {
                for j in (i + 1)..cluster_count {
                    let distance = point_provider.distance(
                        &clusters[i as usize].iter().map(|it| *it).collect(),
                        &clusters[j as usize].iter().map(|it| *it).collect(),
                    );
                    distance_to_index_matrix[j as usize][i as usize].distance = distance;
                    distance_to_index_matrix[j as usize][i as usize].index = i;
                    distance_to_index_matrix[i as usize][j as usize].distance = distance;
                    distance_to_index_matrix[i as usize][j as usize].index = j;
                }
                distance_to_index_matrix[i as usize].sort();
                for j in 0..cluster_count {
                    index_matrix[i as usize][j as usize] =
                        distance_to_index_matrix[i as usize][j as usize].index;
                }
            }

            for i in 0..point_count {
                let point = &points[i];
                let previous_cluster_index = cluster_indices[i];
                let previous_cluster = &clusters[previous_cluster_index as usize];
                let previous_distance =
                    point_provider.distance(&point.to_vec(), &previous_cluster.to_vec());
                let mut minimum_distance = previous_distance;
                let mut new_cluster_index = -1;
                for j in 0..cluster_count {
                    if distance_to_index_matrix[previous_cluster_index as usize][j as usize]
                        .distance
                        >= (4.0 * previous_distance)
                    {
                        continue;
                    }
                    let distance = point_provider.distance(
                        &point.to_vec(),
                        &clusters[j as usize].iter().map(|it| *it).collect(),
                    );
                    if distance < minimum_distance {
                        minimum_distance = distance;
                        new_cluster_index = j;
                    }
                }
                if new_cluster_index != -1 {
                    points_moved += 1;
                    cluster_indices[i] = new_cluster_index;
                }
            }

            if points_moved == 0 && iteration > 0 {
                Self::debug_log(format!("terminated after {} k-means iterations", iteration));
                break;
            }

            Self::debug_log(format!(
                "iteration {} moved {}",
                iteration + 1,
                points_moved
            ));
            let mut component_a_sums = vec![0.0; cluster_count as usize];
            let mut component_b_sums = vec![0.0; cluster_count as usize];
            let mut component_c_sums = vec![0.0; cluster_count as usize];

            for i in 0..cluster_count {
                pixel_count_sums[i as usize] = 0;
            }
            for i in 0..point_count {
                let cluster_index = cluster_indices[i];
                let point = &points[i];
                let count = counts[i];
                pixel_count_sums[cluster_index as usize] += count;
                component_a_sums[cluster_index as usize] += point[0] * count as f64;
                component_b_sums[cluster_index as usize] += point[1] * count as f64;
                component_c_sums[cluster_index as usize] += point[2] * count as f64;
            }
            for i in 0..cluster_count {
                let count = pixel_count_sums[i as usize];
                if count == 0 {
                    clusters[i as usize] = [0.0, 0.0, 0.0].to_vec();
                    continue;
                }
                let a = component_a_sums[i as usize] / count as f64;
                let b = component_b_sums[i as usize] / count as f64;
                let c = component_c_sums[i as usize] / count as f64;
                clusters[i as usize] = [a, b, c].to_vec();
            }
        }

        let mut cluster_argbs: Vec<i64> = vec![];
        let mut cluster_populations: Vec<i64> = vec![];
        for i in 0..cluster_count {
            let count = pixel_count_sums[i as usize];
            if count == 0 {
                continue;
            }

            let possible_new_cluster =
                point_provider.to_int(&clusters[i as usize].iter().map(|it| *it).collect());
            if cluster_argbs.contains(&possible_new_cluster) {
                continue;
            }

            cluster_argbs.push(possible_new_cluster);
            cluster_populations.push(count);
        }
        Self::debug_log(format!(
            "kmeans finished and generated {} clusters; {} were requested",
            cluster_argbs.len(),
            cluster_count
        ));

        let mut input_pixel_to_cluster_pixel: HashMap<i64, i64> = HashMap::new();
        if return_input_pixel_to_cluster_pixel {
            let stopwatch = Stopwatch::new().start();
            for i in 0..pixels.len() {
                let input_pixel = pixels[i];
                let cluster_index = cluster_indices[i as usize];
                let cluster = &clusters[cluster_index as usize];
                let cluster_pixel = point_provider.to_int(&cluster.to_vec());
                input_pixel_to_cluster_pixel.insert(input_pixel, cluster_pixel);
            }
            Self::debug_log(format!(
                "took {} ms to create input to cluster map",
                stopwatch.elapsed_milliseconds()
            ));
        }

        let color_to_count: HashMap<i64, i64> = cluster_argbs
            .into_iter()
            .zip(cluster_populations)
            .map(|(argb, population)| (argb, population))
            .collect();
        return QuantizerResult::new(
            color_to_count,
            //Map.fromIterables(clusterArgbs, clusterPopulations),
            Some(input_pixel_to_cluster_pixel),
        );
    }
}

#[derive(Clone, Copy)]
struct Stopwatch {
    data: Instant,
}
impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            data: Instant::now(),
        }
    }
    pub fn start(&self) -> Stopwatch {
        *self
    }
    pub fn elapsed_milliseconds(&self) -> i64 {
        self.data.elapsed().as_millis() as i64
    }
}
