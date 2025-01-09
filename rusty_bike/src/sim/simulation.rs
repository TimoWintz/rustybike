use crate::sim::kinematics;
use crate::sim::morton;

/// The minimum velocity constant in meters per second.
const MIN_VELOCITY: f64 = 0.1;

/// The tolerance for kinetic energy calculations.
const KINETIC_ENERGY_TOL: f64 = 2.0;

pub struct RoadSegment {
    pub length: f64,
    pub altitude: f64,
    pub slope: f64,
    pub temperature: f64,
    pub relative_wind_speed: f64,
    pub roughness: f64,
}

/// Represents the resistance model for a bicycle simulation.
///
/// # Fields
/// 
/// * `total_mass` - The total mass of the bicycle and rider in kilograms.
/// * `cda_surface` - The coefficient of drag area (CdA) in square meters.
/// * `rolling_resistance` - The coefficient of rolling resistance.
/// * `temperature` - The ambient temperature in degrees Celsius.
/// * `drivetrain_efficiency` - The efficiency of the drivetrain as a fraction (e.g., 0.95 for 95% efficiency).
pub struct BicycleResistanceModel {
    pub total_mass: f64,
    pub cda_surface: f64,
    pub rolling_resistance: f64,
    pub drivetrain_efficiency: f64,
}

/// Returns a default resistance model for a bicycle simulation.
///
/// # Returns
///
/// * `ResistanceModel` - A default resistance model with predefined values.
///
/// # Example
///
/// ```
/// let model = default_resistance_model();
/// println!("Default Resistance Model: {:?}", model);
/// ```
pub const fn default_resistance_model() -> BicycleResistanceModel {
    let model = BicycleResistanceModel {
        total_mass: 80.0,
        cda_surface: 0.3,
        rolling_resistance: 0.004,
        drivetrain_efficiency: 0.98,
    };
    return model;
}


fn compute_time_and_final_velocity(
    initial_velocity: f64,
    input_power: f64,
    road_segment: &RoadSegment,
    resistance_model: &BicycleResistanceModel,
) -> (f64, f64) {
    let mut time: f64 = 0.0;
    let mut position = 0.0;
    let mut current_velocity = initial_velocity;
    let mut step_size;
    let air_resistance_coef = road_segment.relative_wind_speed * resistance_model.cda_surface
        * kinematics::air_density(road_segment.temperature, road_segment.temperature);
    loop {
        let kinetic_energy = kinematics::kinetic_energy(current_velocity, resistance_model.total_mass);
        let force = kinematics::get_total_force(
            kinetic_energy,
            input_power * resistance_model.drivetrain_efficiency,
            road_segment.roughness * resistance_model.rolling_resistance,
            air_resistance_coef,
            road_segment.relative_wind_speed,
            road_segment.slope,
            resistance_model.total_mass,
        );
        step_size = KINETIC_ENERGY_TOL / (0.001 + f64::abs(force));
        if position + step_size > road_segment.length {
            step_size = road_segment.length - position;
        }
        let new_kinetic_energy = kinetic_energy + force * step_size;
        let new_velocity = f64::max(MIN_VELOCITY, kinematics::velocity(new_kinetic_energy, resistance_model.total_mass));

        if position + step_size >= road_segment.length {
            step_size = road_segment.length - position;
            time += step_size / (0.5 * (new_velocity + current_velocity));
            break;
        }
        time += step_size / (0.5 * (new_velocity + current_velocity));
        position += step_size;
        current_velocity = new_velocity;
    }
    return (time, current_velocity);
}



pub fn compute_all_times(
    initial_velocity: f64,
    initial_anaerobic_reserve: f64,
    input_power_vec: &Vec<f64>,
    road_segment_vec: &Vec<RoadSegment>,
    resistance_model: &BicycleResistanceModel,
    rider_model: &morton::RiderModel,
    out_duration_vec: &mut Vec<f64>,
    out_power_vec: &mut Vec<f64>,
    out_anaerobic_reserve: &mut Vec<f64>,
) -> f64 {
    let n_segments = input_power_vec.len();
    let mut velocity = initial_velocity;
    let mut current_anaerobic_reserve = initial_anaerobic_reserve;

    out_duration_vec.resize(n_segments, 0.0);
    out_anaerobic_reserve.resize(n_segments, 0.0);
    out_power_vec.copy_from_slice(input_power_vec);

    let mut total_duration = 0.0;
    for i in 0..n_segments {
        let time_and_velocity = |input_power| {
            compute_time_and_final_velocity(
                velocity,
                input_power,
                &road_segment_vec[i],
                resistance_model,
            )
        };

        let (mut new_time, mut new_velocity) = time_and_velocity( out_power_vec[i]);
        let tau = morton::time_to_exhaustion(rider_model,  out_power_vec[i], current_anaerobic_reserve);
        println!("tau = {:?}s", tau);
        if tau < new_time {
            for j in i..n_segments {
                if out_power_vec[i] < rider_model.critical_power {
                    break;
                }
                out_power_vec[j] =  rider_model.critical_power;
            }
            (new_time, new_velocity) = time_and_velocity( out_power_vec[i]);
        }
        
        current_anaerobic_reserve = morton::update_anaerobic_reserve(rider_model,  out_power_vec[i], new_time, current_anaerobic_reserve);
        println!(
        "{:?}W for {:?}s > {:?}J",
        out_power_vec[i],
        new_time,
        current_anaerobic_reserve
        );
        out_anaerobic_reserve[i] = current_anaerobic_reserve;
        out_duration_vec[i] = new_time;
        total_duration += new_time;
        velocity = new_velocity;
    }
    return  total_duration;
}