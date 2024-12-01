use super::{Speed, Vehicle};
use core::f32;
use std::{i32, time::Duration, u64};

pub struct Stats {
    vehicle_count: usize,
    max_velocity: i32,
    min_velocity: i32,
    max_speed: f32,
    min_speed: f32,
    max_time: Duration,
    min_time: Duration,
    pub close_calls: u32,
    pub collisions: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            vehicle_count: 0,
            max_velocity: 0,
            min_velocity: i32::MAX,
            max_speed: 0.0,
            min_speed: f32::MAX,
            max_time: Duration::from_secs_f32(0.0),
            min_time: Duration::from_secs(u64::MAX),
            collisions: 0,
            close_calls: 0,
        }
    }

    pub fn get(&self) -> Vec<String> {
        vec![
            format!("Vehicle Passed:    {}", self.vehicle_count),
            format!("Max Velocity:      {}", self.max_velocity),
            format!("Min Velocity:      {}", self.min_velocity),
            format!("Max AVG Speed:     {:.2} m/s", self.max_speed / 20.0),
            format!("Min AVG Speed:     {:.2} m/s", self.min_speed / 20.0),
            format!("Max Time:          {:.2} s", self.max_time.as_secs_f32()),
            format!("Min Time:          {:.2} s", self.min_time.as_secs_f32()),
            format!("Close calls:       {}", self.close_calls),
            format!("Collisions:        {}", self.collisions),
        ]
    }

    pub fn update(&mut self, vehicles: &Vec<Vehicle>) {
        self.vehicle_count += vehicles
            .iter()
            .map(|vehicle| {
                let velocity = Speed::velocity(&vehicle.speed);

                self.max_velocity = self.max_velocity.max(velocity);
                self.min_velocity = self.min_velocity.min(velocity);

                vehicle
            })
            .filter(|vehicle| !vehicle.is_visible())
            .map(|vehicle| {
                let elapsed = vehicle.time.elapsed();
                let speed = vehicle.distance / elapsed.as_secs_f32();

                self.max_speed = self.max_speed.max(speed);
                self.min_speed = self.min_speed.min(speed);
                self.max_time = self.max_time.max(elapsed);
                self.min_time = self.min_time.min(elapsed);

                vehicle
            })
            .count();
    }
}
