use core::f64;
use ndarray::Array1;
use ndarray_npy::NpzReader;
use num_traits::cast::AsPrimitive;
use std::fs::File;

use crate::sim::morton;
use crate::sim::simulation;
pub mod sim;


fn convert_array_to_float<T: AsPrimitive<f64>>(b: Array1<T>) -> Vec<f64> {
    let (vec, offset) = b.into_raw_vec_and_offset();
    assert_eq!(offset, Some(0));
    return vec
        .into_iter()
        .map(|x: T| -> f64 {
            return x.as_();
        })
        .collect();
}

fn load_sample_data() -> Result<(Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    let mut npz = NpzReader::new(File::open("resources/murianette.npz")?)?;
    let distance_array: Array1<i64> = npz.by_name("distance")?;
    let elevation_array: Array1<f64> = npz.by_name("elevation")?;

    let distance_vec = convert_array_to_float(distance_array);
    let elevation_vec = convert_array_to_float(elevation_array);

    Ok((distance_vec, elevation_vec))
}

fn build_segment_vecs(
    distance_vec: &Vec<f64>,
    elevation_vec: &Vec<f64>,
) -> Vec<simulation::RoadSegment> {
    let n_segments = distance_vec.len() - 1;
    let mut road_segment_vec: Vec<simulation::RoadSegment> = Vec::with_capacity(n_segments);
   

    for i in 0..n_segments {
        let segment_length = distance_vec[i + 1] - distance_vec[i];
        let slope = (elevation_vec[i + 1] - elevation_vec[i]) / segment_length;
        let temperature = 20.0;
        road_segment_vec.insert(i, simulation::RoadSegment {
            length: segment_length,
            slope: slope,
            temperature: temperature,
            altitude: elevation_vec[i],
            relative_wind_speed: 0.0,
            roughness: 1.0,
        });
    }
    road_segment_vec
}

fn optimize_anaerobic_capacity(
    resistance_model: &simulation::BicycleResistanceModel,
    rider_model: &morton::RiderModel,
    distance_vec: &Vec<f64>,
    elevation_vec: &Vec<f64>,
) {
    let road_segments_vec= build_segment_vecs(&distance_vec, &elevation_vec);
    let n_segments = road_segments_vec.len();
    let input_power_vec: Vec<f64> = vec![rider_model.critical_power; n_segments];
    let mut output_power_vec = input_power_vec.clone();
    let mut durations = Vec::<f64>::new();
    let mut anaerobic_capacity = Vec::<f64>::new();
    let total_time = simulation::compute_all_times(
        0.0,
        rider_model.anaerobic_work_capacity,
        &input_power_vec,
        &road_segments_vec,
        &resistance_model,
        &rider_model,
        &mut durations,
        &mut output_power_vec,
        &mut anaerobic_capacity,
    );

    println!(
        "Initial time (riding at CP): {:?}",
       total_time
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (distance_vec, elevation_vec) = load_sample_data()?;
    let resistance_model = simulation::default_resistance_model();
    let rider_model = morton::default_rider_model();
    optimize_anaerobic_capacity(
        &resistance_model,
        &rider_model,
        &distance_vec,
        &elevation_vec,
    );

    Ok(())
}
