

// Morton model
// A 3-parameter critical power model
// R. HUGH MORTON

pub struct RiderModel {
    pub critical_power: f64,
    pub anaerobic_work_capacity: f64,
    pub max_power: f64,
}

pub const fn default_rider_model() -> RiderModel {
    let model = RiderModel {
        critical_power: 300.0,
        anaerobic_work_capacity: 20000.0,
        max_power: 1000.0,
    };
    return model;
}

pub fn max_power(rider_model: &RiderModel, current_anaerobic_reserve: f64) -> f64 {
    return rider_model.critical_power
        + (rider_model.max_power - rider_model.critical_power) * current_anaerobic_reserve
            / rider_model.anaerobic_work_capacity;
}

pub fn time_to_exhaustion(
    rider_model: &RiderModel,
    input_power: f64,
    current_anaerobic_reserve: f64,
) -> f64 {
    if input_power <= rider_model.critical_power {
        return f64::MAX;
    }
    let delta_p = input_power - rider_model.critical_power;
    return current_anaerobic_reserve / delta_p
        + rider_model.anaerobic_work_capacity
            / (rider_model.critical_power - rider_model.max_power);
}

pub fn update_anaerobic_reserve(rider_model: &RiderModel,
    input_power: f64,
    duration: f64,
    current_anaerobic_reserve: f64) -> f64 {
        let delta_p = input_power - rider_model.critical_power;
        if delta_p > 0.0 {
            return current_anaerobic_reserve - delta_p * duration;
        }
        else {
            return current_anaerobic_reserve + (rider_model.anaerobic_work_capacity - current_anaerobic_reserve) * (1.0 - f64::exp(delta_p * duration / rider_model.anaerobic_work_capacity));
        }
}