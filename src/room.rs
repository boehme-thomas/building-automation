use std::borrow::{Borrow};
use std::sync::Arc;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use rubalosim::sensor::{Sensor, SensorType};
use rubalosim::structure::{EdgeData, Edge, Node, NodeData, UnderlyingStructure};


// source: https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html
// https://github.com/petgraph/petgraph


/// Struct to represent a building. Different rooms and doors are represented as a graph,
/// where rooms are vertices and doors are edges.
pub struct Building  {
    //Arc is necessary because Graph needs the size of its parameters
    floors: Graph<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected>
}

impl Building {
    /// Creates a new Building represented as an undirected graph.
    pub fn new() -> Building {
        Building {
            floors: Graph::<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected>::new_undirected(),
        }
    }

    /// Gets the ids of all adjacent neighbours of a specific room.
    /// <br/>The vector is empty if no neighbours exist or if the node id could not be found.
    pub fn get_neighbours_ids(&mut self, id: String) -> Vec<String> {
        let mut node_indices = self.floors.node_indices();
        let node = node_indices.find(|index| self.floors[*index].get_data().get_id() == id);
        let mut ids = Vec::<String>::new();
        if node.is_some() {
            let neighbours = self.floors.neighbors(node.unwrap());
            for id in neighbours {
                ids.push(self.floors[id].get_data().get_id());
            }
        }
        return ids;
    }

    /// Gets the ids of all adjacent connection of a specific room.
    /// <br/>The vector is empty if no connection exist or if the node id could not be found.
    pub fn get_connection_ids(&mut self, id: String) -> Vec<String> {
        let mut node_indices = self.floors.node_indices();
        let node = node_indices.find(|index| self.floors[*index].get_data().get_id() == id);
        let mut ids = Vec::<String>::new();
        if node.is_some() {
            let neighbours = self.floors.edges(node.unwrap());
            for id in neighbours {
                ids.push(id.weight().get_data().get_id());
            }
        }
        return ids;
    }

    /// Gets all rooms in a building.
    pub fn get_floors(&self) -> &Graph<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected> {
        return self.floors.borrow();
    }

    /// Gets number of rooms in the whole building.
    pub fn get_number_of_rooms(&self) -> usize {
        return self.floors.node_count();
    }

    /// Creates a door connection with a specific id, between two rooms with the given ids.
    /// <br/>Returns ture if successful, false otherwise.
    /// <br/>This should be used, when two rooms are seperated with a door.
    pub fn new_door_connection(&mut self, id_room_1: String, id_room_2: String, door_id: i32) ->  bool {
        let node_indices = self.floors.node_indices();
        let mut node1 = None;
        let mut node2 = None;

        for index in node_indices {
            if node1.is_some() & node2.is_some() {
                break;
            }
            let id = self.floors[index].get_data().get_id();
            if id == id_room_1 {
                node1 = Some(index);
                continue;
            }
            if id == id_room_2 {
                node2 = Some(index);
                continue
            }
        }

        if node1.is_none() | node2.is_none() {
            return false;
        }
        let door_connection = Arc::new(Door::new(door_id, false));
        let edge = Edge::new(door_connection);
        self.floors.update_edge(node1.unwrap(), node2.unwrap(), edge);
        return true;
    }

    /// Creates a connection with a specific id, between to rooms with given ids.
    /// <br/>Returns true if successful, false otherwise.
    /// <br/>This method should be used, when two rooms are seperated, but have no doors,
    /// e.g. the connection between two hallways.
    pub fn new_no_door_connection(&mut self, id_room_1: String, id_room_2: String, door_id: i32) ->  bool {
        let node_indices = self.floors.node_indices();
        let mut node1 = None;
        let mut node2 = None;

        for index in node_indices {
            if node1.is_some() & node2.is_some() {
                break;
            }
            let id = self.floors[index].get_data().get_id();
            if id == id_room_1 {
                node1 = Some(index);
                continue;
            }
            if id == id_room_2 {
                node2 = Some(index);
                continue
            }
        }

        if node1.is_none() | node2.is_none() {
            return false;
        }

        let new_edge = Edge::new(Arc::new(NoDoor::new(door_id, false)));


        self.floors.update_edge(node1.unwrap(), node2.unwrap(), new_edge);
        return true;

    }

    /// Adds a vector of sensors to a specific room.
    pub fn add_sensors_to_room(&mut self, node_id: String, mut sensors: Vec<Sensor>) {
        let mut i = 0;
        for j in 0..sensors.len() {
            let sensor_type =sensors[j].get_sensor_type().get_id().clone();
            sensors[j].set_id("Sensor_".to_owned() + node_id.as_str() + "_no._" + i.to_string().as_str()+"_of_type_"+sensor_type.as_str());
            i = i + 1;
        }
        let indices = self.floors.node_indices();
        for node_index in indices {
            if self.floors[node_index].get_data().get_id() == node_id {
                self.floors[node_index].add_sensors(sensors.clone());
            }
        }
    }

    /// Adds a [Staircase] to the building.
    pub fn add_staircase(&mut self, id_count: i32) {
        let st = Staircase::new(id_count);
        let new_staircase = Node::new(Arc::new(st));
        let _ = self.floors.add_node(new_staircase);
    }

    /// Adds a [room with doors](RoomWithDoors) to the building.
    pub fn add_room_with_doors(&mut self, id_count: i32, what_sensor_should_be_create: Vec<(u32, SensorType)>, windows: bool, offspring_number_for_sensors:i64) -> i64 {
        let rwd = RoomWithDoors::new(id_count, false, "".to_string(), windows);
        let mut new_node = Node::new(Arc::new(rwd));
        let mut sensors = Vec::<Sensor>::new();
        let mut offspring_number = offspring_number_for_sensors;
        for pair in &what_sensor_should_be_create {
            if pair.0 == 0 {
                continue
            }
            for i in 0..pair.0 {
                let sensor_id = "Sensor_".to_owned() + new_node.get_data().get_id().as_str() + "_no._" + i.to_string().as_str()+"_of_type_"+pair.1.get_id().as_str();
                offspring_number += 1;
                let sensor = Sensor::new(sensor_id, pair.1.clone(), offspring_number);
                sensors.push(sensor);
            }
        }
        new_node.add_sensors(sensors);
        let _ = self.floors.add_node(new_node);
        return offspring_number;
    }

    /// Adds a [room without doors](RoomWithoutDoors) to the building.
    pub fn add_room_without_doors(&mut self, id_count: i32, what_sensor_should_be_create: Vec<(u32, SensorType)>, windows: bool, offspring_number_for_sensors:i64) -> i64 {
        let rwnd = RoomWithoutDoors::new(id_count, false, "".to_string(), windows);
        let mut new_node = Node::new(Arc::new(rwnd));
        let mut sensors = Vec::<Sensor>::new();
        let mut offspring_number = offspring_number_for_sensors;
        for pair in &what_sensor_should_be_create {
            if pair.0 == 0 {
                continue
            }
            for i in 0..pair.0 {
                let sensor_id = "Sensor_".to_owned() + new_node.get_data().get_id().as_str() + "_no._" + i.to_string().as_str()+"_of_type_"+pair.1.get_id().as_str();
                offspring_number += 1;
                let sensor = Sensor::new(sensor_id, pair.1.clone(), offspring_number);
                sensors.push(sensor);
            }
        }
        new_node.add_sensors(sensors);
        let _ = self.floors.add_node(new_node);
        return offspring_number;
    }

    /// Adds a specific number of "[rooms with doors](RoomWithDoors)" to a specific room,
    /// connects them with a [`Door`] transition. It also
    /// creates the specific number of [`Sensors`](rubalosim::sensor::Sensor) of the specific
    /// [`SensorType`](rubalosim::sensor::SensorType) for each room.
    /// <br/>The sensors in a room will have an id of the following pattern: _room-id_ _ sensor _ _number_ _ of_type _ _number_.
    pub fn add_sub_rooms_with_doors(&mut self, number_of_rooms: i32, parent_id: String, what_sensor_should_be_create: Vec<(u32, SensorType)>, windows: bool, offspring_number_sensors:i64) -> (bool, i64) {
        let mut node_indices = self.floors.node_indices();
        let parent_index = node_indices.find(|index| self.floors[*index].get_data().get_id() == parent_id);
        if parent_index.is_none() {
            return (false, offspring_number_sensors);
        }
        let mut offspring_number = offspring_number_sensors;
        for id in 0..number_of_rooms {
            let s_rwd = RoomWithDoors::new(id, true, parent_id.clone(), windows);
            let mut new_node = Node::new(Arc::new(s_rwd));
            let mut sensors = Vec::<Sensor>::new();
            for pair in &what_sensor_should_be_create {
                if pair.0 == 0 {
                    continue
                }
                for i in 0..pair.0 {
                    let sensor_id = "Sensor_".to_owned() + new_node.get_data().get_id().as_str() + "_no._" + i.to_string().as_str()+"_of_type_"+pair.1.get_id().as_str();
                    offspring_number += 1;
                    let sensor = Sensor::new(sensor_id, pair.1.clone(), offspring_number);
                    sensors.push(sensor);
                }
            }
            new_node.add_sensors(sensors);
            let node = self.floors.add_node(new_node);
            let edge = Edge::new(Arc::new(Door::new(id, true)));
            self.floors.add_edge(parent_index.unwrap(), node, edge);
        }
        return (true, offspring_number);
    }


    /// Adds a specific number of "[rooms without doors](RoomWithoutDoors)" to a specific room
    /// and connects them with a [NoDoor] transition.It also
    /// creates the specific number of [`Sensors`](rubalosim::sensor::Sensor) of the specific
    /// [`SensorType`](rubalosim::sensor::SensorType) for each room.
    /// <br/>The sensors in a room will have an id of the following pattern: _room-id_ _ sensor _ _number_ _ of_type _ _sensor type id_.
    pub fn add_sub_rooms_without_doors(&mut self, number_of_rooms: i32, parent_id: String, what_sensor_should_be_create: Vec<(u32, SensorType)>, windows: bool, offspring_number_sensors:i64) -> (bool, i64) {
        let mut node_indices = self.floors.node_indices();
        let parent_index = node_indices.find(|index| self.floors[*index].get_data().get_id() == parent_id);
        if parent_index.is_none() {
            return (false, offspring_number_sensors);
        }
        let mut offspring_number = offspring_number_sensors;
        for id in 0..number_of_rooms {
            let s_rwnd = RoomWithoutDoors::new(id, true, parent_id.clone(), windows);
            let mut new_node = Node::new(Arc::new(s_rwnd));

            let mut sensors = Vec::<Sensor>::new();
            for pair in &what_sensor_should_be_create {
                if pair.0 == 0 {
                    continue
                }
                for i in 0..pair.0 {
                    let sensor_id = "Sensor_".to_owned() + new_node.get_data().get_id().as_str() + "_no._" + i.to_string().as_str()+"_of_type_"+pair.1.get_id().as_str();
                    offspring_number += 1;
                    let sensor = Sensor::new(sensor_id, pair.1.clone(), offspring_number);
                    sensors.push(sensor);
                }
            }
            new_node.add_sensors(sensors);

            let node = self.floors.add_node(new_node);
            self.floors.add_edge(parent_index.unwrap(), node, Edge::new(Arc::new(NoDoor::new(id, true))));
        }
        return (true, offspring_number);
    }
}

impl UnderlyingStructure for Building {
    fn get_graph_structure(&self) -> &Graph<Node<Arc<dyn NodeData>>, Edge<Arc<dyn EdgeData>>, Undirected> {
        return self.get_floors();
    }

    // make this changeable
    // Why not with NodeIndex
    fn get_start_nodes(&self) -> Vec<NodeIndex> {
        let mut res = Vec::new();
        let node_indices = self.floors.node_indices();
        for i in node_indices {
            let id = self.floors[i].get_data().get_id();
            if id.starts_with("S") {
                res.push(i);
            }
        }
        return res;
    }

    // this is just the case in this case
    fn get_end_nodes(&self) -> Vec<NodeIndex> {
        self.get_start_nodes()
    }


    fn get_nodes_to_move_to(&self) -> Vec<NodeIndex> {
        let mut res = Vec::new();
        let node_indices = self.floors.node_indices();
        for i in node_indices {
            let id = self.floors[i].get_data().get_id();
            if id.contains("_sub") {
                res.push(i);
            }
        }
        return res;
    }
}

// maybe not necessary
// right now it is not: 22-10-28
pub struct Floor {
    id: String
}

impl NodeData for Floor {
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}


struct Staircase {
    id: String
}

impl Staircase {
    fn new(id_count: i32) -> Self {
        Staircase {
            id: "S".to_owned() + id_count.to_string().as_str(),
        }
    }
}

impl NodeData for Staircase {
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}


struct RoomWithDoors {
    id: String,
    windows: bool,
}

impl RoomWithDoors {
    fn new(id_count: i32, sub_room: bool, parent_id: String, windows: bool) -> Self {
        let sub_room_flag = "_sub";
        let mut id = "RwD".to_owned() + id_count.to_string().as_str();
        if sub_room {
            id = parent_id + "_" +  id.as_str() + sub_room_flag;
        }
        RoomWithDoors{id, windows}

    }

    pub fn has_windows(&self) -> bool {
        self.windows
    }
}

impl NodeData for RoomWithDoors {
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}


struct RoomWithoutDoors {
    id: String,
    windows: bool,
}

impl RoomWithoutDoors {
    fn new(id_count: i32, sub_room: bool, parent_id: String, windows: bool) -> Self {
        let sub_room_flag = "_sub";
        let mut id = "RwnD".to_owned() + id_count.to_string().as_str();
        if sub_room {
            id = parent_id + "_" + id.as_str() + sub_room_flag;
        }
        RoomWithoutDoors {
            id,
            windows
        }
    }

    pub fn has_windows(&self) -> bool {
        self.windows
    }
}

impl NodeData for RoomWithoutDoors {
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}

struct Door {
    id: String,
    //number_of_doors: i32
}

impl Door {
    fn new(id_count: i32, sub_door: bool) -> Self {
        let sub_flag = "_sub";
        let mut id = "Door".to_owned() + id_count.to_string().as_str();
        if sub_door {
            id = id+sub_flag;
        }
        let door = Door {
            id
        };
        return door;
    }
}

impl EdgeData for Door {
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}

struct NoDoor {
    id: String
}

impl NoDoor {
    fn new(id_count: i32, sub_door: bool) -> Self {
        let sub_flag = "_sub";
        let mut id = "NoDoor".to_owned() + id_count.to_string().as_str();
        if sub_door {
            id = id+sub_flag;
        }
        NoDoor{id}
    }
}

impl EdgeData for NoDoor {
    fn get_id(&self) -> String {
        return self.id.clone();
    }
}