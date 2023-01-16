use std::borrow::{BorrowMut};
use std::ops::Index;
use std::sync::Arc;
use rand::{Rng, thread_rng};
use petgraph::graph::{NodeIndex};
use building_automation::room::Building;
use rubalosim::simulator::{Simulator};
use rubalosim::simulator::parameters::Parameters;
use rubalosim::simulator::event::{Event, Events};
use rubalosim::rule::Rule;
use building_automation::human::{Individual};
use chrono::{Duration, NaiveTime};
use elorapi::rules::{RefValue, Condition, Action};
use building_automation::evaluation::Evaluation;

fn startup() -> Simulator {
    let individuals = Arc::new(Individual::new(50, NaiveTime::from_hms_opt(8,0,0).unwrap(), NaiveTime::from_hms_opt(18,0,0).unwrap(), 3, Duration::seconds(40)));

    //let individuals = Arc::new(Individual::new(10, NaiveTime::from_hms_opt(8,0,0).unwrap(), NaiveTime::from_hms_opt(18,0,0).unwrap(), 3, Duration::seconds(40)));
    let mut parameters = Parameters::new(individuals);

    //parameters.add_device_profile_via_file(1203, None, Some("././specification_files/uplink_specification_file_1.json")).unwrap();
    parameters.add_device_profile_via_file(850, Some("././specification_files/downlink_specification_file_2_light.json"), Some("././specification_files/uplink_specification_file_2_light.json")).unwrap();
    parameters.add_device_profile_via_file(250, None, Some("././specification_files/uplink_specification_file_3.json")).unwrap();

    let building = create_building(parameters.borrow_mut());

    parameters.set_underlying_structure(building);
    create_rules(parameters.borrow_mut());

    let simulator = Simulator::new(parameters);

    return simulator;
}

fn create_building(parameters: &mut Parameters) -> Arc<Building> {
    let sensor_types_for_sub_rooms = vec![(1 as u32, parameters.get_sensor_types()[0].clone()),(2 as u32, parameters.get_sensor_types()[1].clone())];
    let sensor_types_for_rooms = vec![(1 as u32, parameters.get_sensor_types()[0].clone()), (2 as u32, parameters.get_sensor_types()[1].clone())];

   // let sensor_types_for_sub_rooms = vec![(1 as u32, parameters.get_sensor_types()[0].clone()),(1 as u32, parameters.get_sensor_types()[1].clone())];
   // let sensor_types_for_rooms = vec![(1 as u32, parameters.get_sensor_types()[0].clone()), (1 as u32, parameters.get_sensor_types()[1].clone())];


    let mut building = Building::new();
    let mut offspring_number:i64 = -1;
    for i in 0..12 {
        if i <= 1 {
            offspring_number= building.add_room_with_doors(i, sensor_types_for_rooms.clone(), false, offspring_number);
        }
        if i <= 4 {
            building.add_staircase(i);
        }
        offspring_number = building.add_room_without_doors(i, sensor_types_for_rooms.clone(), false, offspring_number);
    }
    let vec1 = vec![("RwnD0", "RwD0"), ("RwD0", "RwnD1"), ("RwnD1", "S0"), ("RwnD1", "RwD1"), ("RwnD1","RwnD2"), ("RwnD2", "RwnD3"), ("RwnD2", "RwnD4"), ("RwnD4", "S1"), ("RwnD4", "RwnD5"), ("RwnD5", "RwnD0"), ("RwnD5", "S2"), ("RwnD5", "RwnD6"), ("RwnD6", "RwnD7"), ("RwnD7", "S3"), ("RwnD7", "RwnD8"), ("RwnD8", "RwnD4"), ("RwnD8", "RwnD9"), ("RwnD9", "RwnD10"), ("RwnD10", "RwnD7"), ("RwnD10", "S4"), ("RwnD10", "RwnD11"), ("RwnD11", "RwD1")];
    let mut ok = false;
    let mut i = 0;
    for pair in vec1 {
        if pair.0.contains("RwD") | pair.1.contains("RwD") {
            ok = building.new_door_connection(pair.0.to_string(), pair.1.to_string(), i);
        } else {
            ok = building.new_no_door_connection(pair.0.to_string(), pair.1.to_string(), i);
        }
        i += 1;
    }
    let mut ok = false;

    (ok, offspring_number) = building.add_sub_rooms_with_doors(14, "RwnD0".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);

    let _ = building.new_door_connection("RwnD0_RwD10_sub".to_string(), "RwnD0_RwD11_sub".to_string(), 0);
    let _ = building.new_door_connection("RwnD0_RwD10_sub".to_string(), "RwnD0_RwD11_sub".to_string(), 1);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(7, "RwnD1".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);

    (ok, offspring_number) = building.add_sub_rooms_with_doors(7, "RwnD5".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(3, "RwnD6".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(6, "RwnD7".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(3, "RwnD8".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(5, "RwnD9".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(10, "RwnD10".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(9, "RwnD11".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(5, "RwD0".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);
    let _ = building.new_door_connection("RwD0_RwD0_sub".to_string(), "RwD0_RwD1_sub".to_string(), 0);
    let _ = building.new_door_connection("RwD0_RwD2_sub".to_string(), "RwD0_RwD3_sub".to_string(), 1);
    (ok, offspring_number) = building.add_sub_rooms_with_doors(3, "RwD1".to_string(), sensor_types_for_sub_rooms.clone(), true, offspring_number);


    let count = building.get_number_of_rooms();
    let count_e = building.get_floors().edge_count();
    println!("Number of rooms: {}, Number of edges: {}", count, count_e);
    println!("Offspring number {}", offspring_number);
    // let neighbours = building.get_neighbours_ids("RwnD4".to_string());
    //println!("{:?}", neighbours);

    let connections = building.get_connection_ids("RwnD4".to_string());
    //println!("{:?}", connections);

    let connections = building.get_connection_ids("RwnD0_RwD10_sub".to_string());
    //println!("{:?}", connections);

    parameters.set_number_of_sensors(offspring_number);
    Arc::new(building)
}

/* rules:
 *       for all rooms: When someone entering a room (so when the occupancy sensor detects a person) then the light should go on
 *       sub room: When someone is leaving a sub-room (when the occupancy sensor does not detect a motion) the lights should go off
 *       not sub room: When someone is leaving the room (when the occupancy sensor does not detect a motion) the light should be:
            - dimmed: when the time is between 8pm and 5:59:59am
            - turned off: when the time is between 6 am and 7:59:59pm
 *
 */
fn create_rules(parameters: &mut Parameters) {
    // lights should go out
    let start_time_condition_for_turning_lights_off = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
    let end_time_condition_for_turning_lights_off = NaiveTime::from_hms_opt(6, 29, 59).unwrap();

    // lights should be dimmed times:
    let start_time_condition_for_turning_lights_dim = NaiveTime::from_hms_opt(6, 30, 0).unwrap();
    let end_time_condition_for_turning_lights_dim = NaiveTime::from_hms_opt(17, 59, 59).unwrap();

    // lights should only go on when
    let start_time_condition_for_turning_lights_on_sub = NaiveTime::from_hms_opt(16, 30, 0).unwrap();
    let end_time_condition_for_turning_lights_on_sub = NaiveTime::from_hms_opt(8, 30, 0).unwrap();

    let underlying_structure = parameters.get_underlying_structure();
    let node_indices = underlying_structure.get_graph_structure().node_indices();
    let mut rules = Vec::<Rule>::new();

    let mut rule_1 = 0;
    let mut rule_2 = 0;
    let mut rule_3 = 0;
    let mut rule_4 = 0;
    let mut rule_5 = 0;

    for index in node_indices {
        let node = underlying_structure.get_graph_structure().index(index);
        let node_id = node.get_data().get_id();

        let sensors = node.get_sensors();
        let mut sensors_id_type_3 = Vec::<(String, i64)>::new();
        let mut sensors_id_type_2 = Vec::<(String, i64)>::new();

        for i in sensors {
            let id = i.get_sensor_type().get_id();
            if id.contains("SensorType_1") {
                sensors_id_type_3.push((i.get_id(), i.get_number()));
            } else if id.contains("SensorType_0") {
                sensors_id_type_2.push((i.get_id(), i.get_number()));
            };
        }
        if !sensors_id_type_3.is_empty() & !sensors_id_type_2.is_empty() {
            let mut type_3_sensor_measures_occupancy_conditions = Vec::<Condition>::new();
            for sensor_id_type_3 in &sensors_id_type_3 {
                    type_3_sensor_measures_occupancy_conditions.push(Condition::Device(Rule::create_device_condition(sensor_id_type_3.0.clone(), sensor_id_type_3.1.clone(),  0, "==".to_string(), RefValue::String("true".to_string()))));
                }
            let mut type_2_sensor_turn_lights_on_actions = Vec::<Action>::new();
            for sensor_id_type_2 in &sensors_id_type_2 {
                    type_2_sensor_turn_lights_on_actions.push(Rule::create_device_action(sensor_id_type_2.0.clone(),sensor_id_type_2.1.clone(), vec![0]));
            }
            let mut bool_ops = Vec::<String>::new();

            let len = sensors_id_type_3.len();

            if len > 1 {
                for _ in 0..len-1 {
                    bool_ops.push("|".to_string());
                }
            }

            let turn_lights_on_when_arriving_rule = Rule::create_rule("Turn_lights_on_when_arriving_rule_node_".to_owned() + node_id.as_str(), type_3_sensor_measures_occupancy_conditions, bool_ops, type_2_sensor_turn_lights_on_actions);
            rules.push(turn_lights_on_when_arriving_rule);
            rule_1 += 1;

            if node_id.contains(&"_sub".to_string()) {
                let mut type_3_sensor_measures_no_occupancy_conditions = Vec::<Condition>::new();
                for sensor_id_type_3 in &sensors_id_type_3 {
                    type_3_sensor_measures_no_occupancy_conditions.push(Condition::Device(Rule::create_device_condition(sensor_id_type_3.0.clone(), sensor_id_type_3.1.clone(), 0, "==".to_string(), RefValue::String("false".to_string()))));
                }

                let mut type_2_sensor_turn_lights_off_actions = Vec::<Action>::new();
                for sensor_id_type_2 in &sensors_id_type_2 {
                    type_2_sensor_turn_lights_off_actions.push(Rule::create_device_action(sensor_id_type_2.0.clone(), sensor_id_type_2.1.clone(), vec![2]));
                }
                let mut bool_ops = Vec::<String>::new();

                let len = sensors_id_type_3.len();

                if len > 1 {
                    for _ in 0..len-1 {
                        bool_ops.push("|".to_string());
                    }
                }
                let turn_lights_off_when_leaving_rule = Rule::create_rule("Turn_lights_off_when_leaving_rule_node_".to_owned() + node_id.as_str(), type_3_sensor_measures_no_occupancy_conditions, bool_ops, type_2_sensor_turn_lights_off_actions);
                rules.push(turn_lights_off_when_leaving_rule);
                rule_2 += 1;

                /*
                // turn lights only on when its in between 16 and 10
                let mut type_3_sensor_measures_occupancy_conditions_on_sub = Vec::<Condition>::new();
                for sensor_id_type_3 in &sensors_id_type_3 {
                    type_3_sensor_measures_occupancy_conditions_on_sub.push(Condition::Device(Rule::create_device_condition(sensor_id_type_3.0.clone(), sensor_id_type_3.1.clone(),  0, "==".to_string(), RefValue::String("true".to_string()))));
                }

                let time_condition_for_turning_lights_on_cub = Condition::Time(Rule::create_time_condition(None, start_time_condition_for_turning_lights_on_sub, end_time_condition_for_turning_lights_on_sub));
                type_3_sensor_measures_occupancy_conditions_on_sub.push(time_condition_for_turning_lights_on_cub);

                let mut type_2_sensor_turn_lights_on_actions = Vec::<Action>::new();
                for sensor_id_type_2 in &sensors_id_type_2 {
                    type_2_sensor_turn_lights_on_actions.push(Rule::create_device_action(sensor_id_type_2.0.clone(),sensor_id_type_2.1.clone(), vec![0]));
                }
                let mut bool_ops = Vec::<String>::new();

                let len = sensors_id_type_3.len();

                if len > 1 {
                    for _ in 0..len-1 {
                        bool_ops.push("|".to_string());
                    }
                }

                bool_ops.push("&".to_string());

                let turn_lights_on_when_arriving_rule_sub = Rule::create_rule("Turn_lights_on_when_arriving_rule_node_".to_owned() + node_id.as_str(), type_3_sensor_measures_occupancy_conditions_on_sub, bool_ops, type_2_sensor_turn_lights_on_actions);
                rules.push(turn_lights_on_when_arriving_rule_sub);
                rule_5
                 += 1;*/

            } else {
                let mut type_3_sensor_measures_no_occupancy_conditions = Vec::<Condition>::new();
                for sensor_id_type_3 in &sensors_id_type_3 {
                    type_3_sensor_measures_no_occupancy_conditions.push(Condition::Device(Rule::create_device_condition(sensor_id_type_3.0.clone(), sensor_id_type_3.1.clone(), 0, "==".to_string(), RefValue::String("false".to_string()))));
                }

                let time_condition_for_turning_lights_dim = Condition::Time(Rule::create_time_condition(None, start_time_condition_for_turning_lights_dim, end_time_condition_for_turning_lights_dim));
                type_3_sensor_measures_no_occupancy_conditions.push(time_condition_for_turning_lights_dim);

                let mut type_2_sensor_dim_lights_actions = Vec::<Action>::new();
                for sensor_id_type_2 in &sensors_id_type_2 {
                    type_2_sensor_dim_lights_actions.push(Rule::create_device_action(sensor_id_type_2.0.clone(), sensor_id_type_2.1.clone(), vec![1]));
                }
                let mut bool_ops = Vec::<String>::new();

                let len = sensors_id_type_3.len();
                if len > 1 {
                    for _ in 0..len-1 {
                        bool_ops.push("|".to_string());
                    }
                }
                bool_ops.push("&".to_string());
                let dim_lights_when_leaving_rule = Rule::create_rule("Dim_lights_when_leaving_+_time_rule_node_".to_owned() + node_id.as_str(), type_3_sensor_measures_no_occupancy_conditions, bool_ops, type_2_sensor_dim_lights_actions);
                rules.push(dim_lights_when_leaving_rule);
                rule_3 += 1;



                let mut type_3_sensor_measures_no_occupancy_conditions = Vec::<Condition>::new();
                for sensor_id_type_3 in &sensors_id_type_3 {
                    type_3_sensor_measures_no_occupancy_conditions.push(Condition::Device(Rule::create_device_condition(sensor_id_type_3.0.clone(), sensor_id_type_3.1.clone(), 0, "==".to_string(), RefValue::String("false".to_string()))));
                }

                let time_condition_for_turning_lights_off = Condition::Time(Rule::create_time_condition(None, start_time_condition_for_turning_lights_off, end_time_condition_for_turning_lights_off));
                type_3_sensor_measures_no_occupancy_conditions.push(time_condition_for_turning_lights_off);

                let mut type_2_sensor_turn_lights_off_actions = Vec::<Action>::new();
                for sensor_id_type_2 in &sensors_id_type_2 {
                    type_2_sensor_turn_lights_off_actions.push(Rule::create_device_action(sensor_id_type_2.0.clone(), sensor_id_type_2.1.clone(), vec![2]));
                }
                let mut bool_ops = Vec::<String>::new();

                let len = sensors_id_type_3.len();
                if len > 1 {
                    for _ in 0..len-1 {
                        bool_ops.push("|".to_string());
                    }
                }
                bool_ops.push("&".to_string());
                let turn_lights_off_when_leaving_rule_with_time_condition = Rule::create_rule("Turn_lights_off_when_leaving_+_time_rule_node".to_owned() + node_id.as_str(), type_3_sensor_measures_no_occupancy_conditions, bool_ops, type_2_sensor_turn_lights_off_actions);
                rules.push(turn_lights_off_when_leaving_rule_with_time_condition);

                rule_4 += 1;
            }
        }
    }
    println!("rule_1:{}",rule_1);
    println!("rule_2:{}",rule_2);
    println!("rule_3:{}",rule_3);
    println!("rule_4:{}",rule_4);
    parameters.set_rule(rules);
}

// this works as planed
fn create_uplink_message(simulator: &mut Simulator) {
    let matrix = simulator.get_matrix_of_nodes_of_movable_objects();
    let eventlist = simulator.get_event_list().get_event_list_copy();
    let structure = simulator.get_parameters().get_underlying_structure();
    let graph = structure.get_graph_structure();
    let mut new_events = Vec::<Event>::new();
    let eventlist_len = eventlist.len();
    let mut vec_tuple_of_changed_eventlist = Vec::<(usize, Event)>::new();
    let mut rng = thread_rng();
    'event_list: for index in 0..eventlist_len {
        let event = eventlist.get(index).unwrap();
        let action = event.get_action();
        let time = event.get_time();
        match action {
            Events::Move(node_index) => {
                let mut mov_object_number = event.get_id();
                let mut number_of_move = mov_object_number.clone();
                // get number of move
                number_of_move = number_of_move.replace("_", "");
                let dot_index = number_of_move.find(".").unwrap();
                number_of_move.replace_range(0..dot_index+1, "");
                let move_number = number_of_move.parse::<usize>().unwrap();
                //println!("{}", move_number);

                // get number of movable object
                mov_object_number = mov_object_number.trim_start_matches("Movable_object_").to_string();
                let index_of_ = mov_object_number.find("_");
                mov_object_number.replace_range(index_of_.unwrap().., "");
                let movable_object_number = mov_object_number.parse::<usize>().unwrap();

                // and then its nodes_vector
                let vecs_of_mov_obj = matrix[movable_object_number].clone();

                let node = graph.node_weight(node_index).unwrap();
                let res:Option<&(usize, NaiveTime, NodeIndex)> = vecs_of_mov_obj.get(move_number+1);

                let node_sensors = node.get_sensors();

                //sensor
                for sensor in &node_sensors {
                    let sensor_type = sensor.get_sensor_type();
                    //let sensor_id = sensor.get_id();
                    if sensor_type.get_id() == "SensorType_1" {
                        let id = "Message_of_".to_string() + sensor.get_number().to_string().as_str() + "_" + sensor.get_id().as_str();
                        //time plus 1 sec
                        let range = rng.gen_range(0..1000);
                        let new_time = time + Duration::milliseconds(range);
                        let message = "Uplink_Message_occupancy:true,".to_string();
                        let new_action = Events::Message(message);
                        let new_event = Event::new(id.clone(), new_time, new_action);
                        new_events.push(new_event);

                        let end = match res {
                            None => {
                                continue 'event_list
                            },
                            Some(tuple) => {
                                tuple.1
                            }
                        };

                        // look at the following events, if future events contain the sensor id then change that message
                        let mut sec_index = index.clone()+1;
                        let mut future_event = eventlist.get(sec_index).unwrap();
                        while future_event.get_time() <= end {
                            let sensor_id = future_event.get_id();
                            if sensor_id.contains("SensorType_1") & (id == sensor_id) {
                                let new_action = Events::Message("Uplink_Message_occupancy:true,".to_string());
                                let new_event = Event::new(future_event.get_id().clone(), future_event.get_time(), new_action);
                                vec_tuple_of_changed_eventlist.push((sec_index, new_event));
                            }
                            sec_index +=1;
                            if sec_index < eventlist_len {
                                future_event = eventlist.get(sec_index).unwrap();
                            } else {
                                break;
                            }
                        }

                    }
                }

            }
            _ => {}
        };
    }
    let eventlist = simulator.get_event_list_mut();
    let actual_eventlist = eventlist.get_event_list_mut();
    for tuple in vec_tuple_of_changed_eventlist {
        actual_eventlist[tuple.0] = tuple.1;
    }
    for event in new_events {
        eventlist.add_event(event);
    }
}



fn main() {
    let mut simulation = startup();
    let event_list = simulation.get_event_list();

    let _ = simulation.start_up_simulation(10);
    simulation.add_standard_values_to_uplink_messages(vec!["0".to_string(), "1".to_string()], vec![vec!["Off".to_string()], vec!["false".to_string()]]);
    create_uplink_message(simulation.borrow_mut());
    simulation.rule_execution();


    //simulation.print_event_list_sensor("Sensor_RwnD0_no._0_of_type_SensorType_3".to_string());
    //simulation.write_events_downlink_message("./".to_string()).unwrap();
    //simulation.write_events_of_movable_object("./".to_string(), "Movable_object_".to_string()).unwrap();
    let event_list = simulation.ending_simulation("./".to_string());
    let mut evaluation = Evaluation::new(vec![1.0, 1.0], vec![vec![45.0, 0.0, 0.0], vec![40.0, 20.0, 0.0]]);
    evaluation.calculate_and_write_consumption(event_list, "./".to_string());
    simulation.write_event_list("./".to_string()).unwrap();
}