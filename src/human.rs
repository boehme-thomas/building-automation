use chrono::{Duration, NaiveTime};
use rubalosim::movable_object::MovableObjects;

/// Representation of a individual. It can move to specific points in the building
/// with a specific static speed.
pub struct Individual {
    number_of_movable_objects: u32,
    time_of_creation: NaiveTime,
    time_of_deletion: NaiveTime,
    number_of_random_moves: u32,
    speed: Duration,
}

impl Individual {
    pub fn new(number_of_movable_objects: u32, time_of_creation: NaiveTime, time_of_deletion: NaiveTime, number_of_random_moves: u32, speed: Duration) -> Individual {
        return Individual {
            number_of_movable_objects,
            time_of_creation,
            time_of_deletion,
            number_of_random_moves,
            speed,
        }
    }
}

impl MovableObjects for Individual {
    fn get_number_of_movable_objects(&self) -> u32 {
        self.number_of_movable_objects.clone()
    }

    fn get_number_of_random_moves(&self) -> u32 {
        return self.number_of_random_moves;
    }

    fn get_time_of_creation(&self) -> NaiveTime {
        self.time_of_creation.clone()
    }

    fn get_time_of_deletion(&self) -> NaiveTime {
        self.time_of_deletion.clone()
    }

    fn get_speed(&self) -> Duration {
        self.speed
    }

}