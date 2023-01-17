use std::fs;
use std::io::Write;
use chrono::{Local, Timelike};
use rubalosim::simulator::event::{EventList, Events};

pub struct Evaluation {
    // as Wh per year
    energy_consumption_sensor_type: Vec<f64>,
    // as Watt
    energy_consumption_light_in_different_states_of_different_rooms: Vec<Vec<f64>>,
}

impl Evaluation {
    pub fn new(energy_consumption_sensor_type: Vec<f64>, energy_consumption_light_in_different_states_of_different_rooms: Vec<Vec<f64>>) -> Self {
        return Evaluation {
            energy_consumption_sensor_type,
            energy_consumption_light_in_different_states_of_different_rooms
        }
    }

    pub fn calculate_and_write_consumption(&mut self, event_list: &EventList, path:String) {
        let mut energy_consumption_sub_rooms_average = 0.0;
        let mut energy_consumption_rooms_average = 0.0;

        let mut vec_consumption_per_room:Vec<(String, f64)>= Vec::new();

        let mut vec_consumption_per_sub_room:Vec<(String, f64)>= Vec::new();

        // first vector of energy_consumption_light_in_different_states_of_different_rooms is for subrooms the second for normal rooms
        let event_list_copy = event_list.get_event_list_copy();
        let len = event_list_copy.len();
        let start = event_list_copy.first().unwrap().get_time();
        let end = event_list_copy.last().unwrap().get_time();

        let length = (end-start).num_milliseconds() as f64;

        'event_list: for ev_index in 0..len {
            let event = event_list_copy.get(ev_index).unwrap();

            match event.get_action() {
                Events::Message(message) => {
                    if event.get_id().contains("SensorType_1") {
                        continue
                    }
                    let mut message_id = event.get_id();
                    let index = message_id.find("Sensor_").unwrap()+7;
                    message_id.replace_range(..index, "");
                    let index = message_id.find("_no.").unwrap();
                    message_id.replace_range(index.., "");

                    let mut action_message = message.clone();
                    let index = action_message.find(":");
                    if index.is_none() {
                        continue
                    }
                    let index = index.unwrap()+1;
                    action_message = action_message.replace(",", "");
                    action_message.replace_range(..index, "");

                    let start = event.get_time();

                    let mut new_index = ev_index + 1;
                    while new_index < len {
                        let next_event = event_list_copy.get(new_index).unwrap();
                        if next_event.get_id() != event.get_id() {
                            new_index += 1;
                            continue
                        }
                        let end = next_event.get_time();
                        let duration = (end-start).num_milliseconds();


                        if message_id.contains("sub") {
                            let result:f64 = match action_message.as_str() {
                                "On" => {
                                    duration as f64 / 1000.0 * self.energy_consumption_light_in_different_states_of_different_rooms[0][0]
                                },
                                "Dim" => {
                                    duration as f64 / 1000.0 * self.energy_consumption_light_in_different_states_of_different_rooms[0][1]
                                },
                                "Off" => {
                                    duration as f64 / 1000.0 * self.energy_consumption_light_in_different_states_of_different_rooms[0][2]
                                },
                                _=> {
                                    0.0
                                }
                            };

                            energy_consumption_sub_rooms_average += result;

                            for i in 0..vec_consumption_per_sub_room.len() {
                                if vec_consumption_per_sub_room[i].0 == message_id {
                                    vec_consumption_per_sub_room[i].1 += result;
                                    continue 'event_list
                                }
                            }
                            vec_consumption_per_sub_room.push((message_id, result));
                            continue 'event_list

                        } else {
                            let result:f64 = match action_message.as_str() {
                                "On" => {
                                    duration as f64 / 1000.0 * self.energy_consumption_light_in_different_states_of_different_rooms[1][0]
                                },
                                "Dim" => {
                                    duration as f64 / 1000.0 * self.energy_consumption_light_in_different_states_of_different_rooms[1][1]
                                },
                                "Off" => {
                                    duration as f64 / 1000.0 * self.energy_consumption_light_in_different_states_of_different_rooms[1][2]
                                },
                                _ => 0.0
                            };

                            energy_consumption_rooms_average += result.clone();

                            for i in 0..vec_consumption_per_room.len() {
                                if vec_consumption_per_room[i].0 == message_id {
                                    vec_consumption_per_room[i].1 += result;
                                    continue 'event_list
                                }
                            }
                            vec_consumption_per_room.push((message_id, result));
                            continue 'event_list
                        }
                    }
                }
                _ => {
                    continue
                }

            }
        }
        energy_consumption_sub_rooms_average = (energy_consumption_sub_rooms_average / vec_consumption_per_sub_room.len() as f64) / 60.0 / 60.0;
        energy_consumption_rooms_average = (energy_consumption_rooms_average / vec_consumption_per_room.len() as f64) / 60.0 / 60.0;

        let number_of_sensors = (vec_consumption_per_sub_room.len() + vec_consumption_per_room.len()) as f64;

        let energy_consumption_sensor_type0 = self.energy_consumption_sensor_type[0] * number_of_sensors / 365.0;
        let energy_consumption_sensor_type1 = self.energy_consumption_sensor_type[1] * 2.0 * number_of_sensors / 365.0;

        let date = Local::now();
        let path = path + "Energy_evaluation_" + date.date_naive().to_string().as_str() + "_" + date.time().hour().to_string().as_str() + "_" + date.time().minute().to_string().as_str() + "_" +date.time().second().to_string().as_str() + ".txt";
        let mut f = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path).unwrap();

        let data = "Energy consumption of all sensors of type 0 in Wh: ".to_owned() + energy_consumption_sensor_type0.to_string().as_str() + "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Energy consumption of all sensors of type 1 in Wh: ".to_owned() + energy_consumption_sensor_type1.to_string().as_str() + "\n";
        f.write(data.as_bytes()).unwrap();


        let data = "Average energy consumption of rooms in Wh: ".to_owned() + energy_consumption_rooms_average.to_string().as_str() + "\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Average energy consumption of sub rooms in Wh: ".to_owned() + energy_consumption_sub_rooms_average.to_string().as_str() + "\n\n";
        f.write(data.as_bytes()).unwrap();

        let data = "Energy consumption per room in Wh: ";
        f.write(data.as_bytes()).unwrap();
        for value in vec_consumption_per_room {
            let wh = value.1 / 60.0 / 60.0;
            let data = "\n\t".to_owned() + value.0.as_str() + ": " + wh.to_string().as_str();
            f.write(data.as_bytes()).unwrap();
        }

        let data = "\n\nEnergy consumption per sub room in Wh: ";
        f.write(data.as_bytes()).unwrap();
        for value in vec_consumption_per_sub_room {
            let wh = value.1 / 60.0 / 60.0;
            let data = "\n\t".to_owned() + value.0.as_str() + ": " + wh.to_string().as_str();
            f.write(data.as_bytes()).unwrap();
        }

    }

}