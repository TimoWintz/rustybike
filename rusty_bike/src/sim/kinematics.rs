/// The gravitational acceleration constant in meters per second squared.
pub const fn gravity_acceleration() -> f64 {
    return 9.81;
}

/// Calculates the air density given the altitude and temperature.
///
/// # Arguments
///
/// * `altitude` - The altitude in meters.
/// * `temperature` - The temperature in Celsius.
///
/// # Returns
///
/// * `f64` - The air density in kg/m^3.
///
/// # Example
///
/// ```
/// let density = air_density(1000.0, 15.0);
/// println!("Air density: {}", density);
/// ```
pub fn air_density(altitude: f64, temperature: f64) -> f64 {
    let pressure_pa = 100.0 * 1013.25 * f64::powf(1.0 - 0.0065 * altitude / 288.15, 5.255);
    let r_air = 287.0;
    let kelvin = temperature + 273.0;
    return pressure_pa / kelvin / r_air;
}

/// Calculates the velocity given the kinetic energy and total mass.
///
/// # Arguments
///
/// * `kinetic_energy` - The kinetic energy in joules.
/// * `total_mass` - The total mass in kilograms.
///
/// # Returns
///
/// * `f64` - The velocity in meters per second.
///
/// # Example
///
/// ```
/// let vel = velocity(500.0, 70.0);
/// println!("Velocity: {}", vel);
/// ```
pub fn velocity(kinetic_energy: f64, total_mass: f64) -> f64 {
    return f64::sqrt(2.0 * f64::abs(kinetic_energy) / total_mass);
}

/// Calculates the kinetic energy given the velocity and total mass.
///
/// # Arguments
///
/// * `velocity` - The velocity in meters per second.
/// * `total_mass` - The total mass in kilograms.
///
/// # Returns
///
/// * `f64` - The kinetic energy in joules.
///
/// # Example
///
/// ```
/// let ke = kinetic_energy(10.0, 70.0);
/// println!("Kinetic Energy: {}", ke);
/// ```
pub fn kinetic_energy(velocity: f64, total_mass: f64) -> f64 {
    return 0.5 * total_mass * velocity * velocity;
}

/// Calculates the drag force given the velocity, wind velocity, rolling resistance, and air resistance coefficient.
///
/// # Arguments
///
/// * `velocity` - The velocity in meters per second.
/// * `wind_velocity` - The wind velocity in meters per second.
/// * `rolling_resistance` - The rolling resistance coefficient.
/// * `air_resistance_coef` - The air resistance coefficient.
///
/// # Returns
///
/// * `f64` - The drag force in newtons.
///
/// # Example
///
/// ```
/// let drag_force = get_drag_force(10.0, 2.0, 0.005, 0.3);
/// println!("Drag Force: {}", drag_force);
/// ```
pub fn get_drag_force(
    velocity: f64,
    wind_velocity: f64,
    rolling_resistance: f64,
    air_resistance_coef: f64,
    total_mass: f64
) -> f64 {
    let air_resistance =
        air_resistance_coef * f64::abs(velocity + wind_velocity) * (velocity + wind_velocity);
    return air_resistance + rolling_resistance * gravity_acceleration() * total_mass;
}

/// Calculates the total force acting on the object given the kinetic energy, input power, rolling resistance, air resistance coefficient, wind velocity, slope, and total mass.
///
/// # Arguments
///
/// * `kinetic_energy` - The kinetic energy in joules.
/// * `input_power` - The input power in watts.
/// * `rolling_resistance` - The rolling resistance coefficient.
/// * `air_resistance_coef` - The air resistance coefficient.
/// * `wind_velocity` - The wind velocity in meters per second.
/// * `slope` - The slope of the surface (dimensionless).
/// * `total_mass` - The total mass in kilograms.
///
/// # Returns
///
/// * `f64` - The total force in newtons.
///
/// # Example
///
/// ```
/// let total_force = get_total_force(500.0, 250.0, 0.005, 0.3, 2.0, 0.05, 70.0);
/// println!("Total Force: {}", total_force);
/// ```
pub fn get_total_force(
    kinetic_energy: f64,
    input_power: f64,
    rolling_resistance: f64,
    air_resistance_coef: f64,
    wind_velocity: f64,
    slope: f64,
    total_mass: f64,
) -> f64 {
    let velocity = velocity(kinetic_energy, total_mass);

    let drag_force = f64::signum(kinetic_energy)
        * get_drag_force(
            velocity,
            wind_velocity,
            rolling_resistance,
            air_resistance_coef,
            total_mass
        );

    let gravity_force = gravity_acceleration() * slope * total_mass;
    let total_force = input_power / velocity - drag_force - gravity_force;
    return total_force;
}

