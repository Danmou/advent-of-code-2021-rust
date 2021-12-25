use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum AmphipodType {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Amphipod {
    class: AmphipodType,
    has_moved: bool,
}

impl Amphipod {
    fn new(class: char) -> Amphipod {
        Amphipod {
            class: match class {
                'A' => AmphipodType::A,
                'B' => AmphipodType::B,
                'C' => AmphipodType::C,
                'D' => AmphipodType::D,
                _ => panic!(),
            },
            has_moved: false,
        }
    }

    fn to_string(&self) -> &str {
        match self.class {
            AmphipodType::A => "A",
            AmphipodType::B => "B",
            AmphipodType::C => "C",
            AmphipodType::D => "D",
        }
    }

    fn weight(&self) -> u64 {
        match self.class {
            AmphipodType::A => 1,
            AmphipodType::B => 10,
            AmphipodType::C => 100,
            AmphipodType::D => 1000,
        }
    }

    fn destination(&self) -> RoomLocation {
        match self.class {
            AmphipodType::A => RoomLocation::RoomA,
            AmphipodType::B => RoomLocation::RoomB,
            AmphipodType::C => RoomLocation::RoomC,
            AmphipodType::D => RoomLocation::RoomD,
        }
    }

    fn can_enter(&self, r: RoomLocation) -> bool {
        [
            RoomLocation::HallwayLeft,
            RoomLocation::HallwayMidLeft,
            RoomLocation::HallwayMid,
            RoomLocation::HallwayMidRight,
            RoomLocation::HallwayRight,
            self.destination(),
        ]
        .contains(&r)
    }
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, Hash, Copy, Clone)]
enum RoomLocation {
    HallwayLeftLeft,
    HallwayLeft,
    HallwayMidLeft,
    HallwayMid,
    HallwayMidRight,
    HallwayRight,
    HallwayRightRight,
    RoomA,
    RoomB,
    RoomC,
    RoomD,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Room {
    capacity: usize,
    content: [Option<Amphipod>; 2],
    location: RoomLocation,
}

impl Room {
    fn single(content: Option<Amphipod>, location: RoomLocation) -> Room {
        Room {
            capacity: 1,
            content: [content, None],
            location,
        }
    }

    fn double(content: [Option<Amphipod>; 2], location: RoomLocation) -> Room {
        Room {
            capacity: 2,
            content,
            location,
        }
    }

    fn has_space(&self) -> bool {
        match self.capacity {
            1 => self.content[0].is_none(),
            2 => self.content[1].is_none(),
            _ => panic!(),
        }
    }

    fn as_vec(&self) -> Vec<Option<Amphipod>> {
        let mut v = vec![];
        for i in 0..self.capacity {
            v.push(self.content[i].clone())
        }
        v
    }

    fn is_solved(&self) -> bool {
        let all_at_dest = self.as_vec().iter().all(|a| match a {Some(a) => Some(a.destination()), None => None} == Some(self.location));
        match self.location {
            RoomLocation::RoomA
            | RoomLocation::RoomB
            | RoomLocation::RoomC
            | RoomLocation::RoomD => all_at_dest,
            _ => self.outer().is_none(),
        }
    }

    fn outer(&self) -> Option<usize> {
        if self.content[1].is_some() {
            Some(1)
        } else if self.content[0].is_some() {
            Some(0)
        } else {
            None
        }
    }

    fn next(&self) -> Option<usize> {
        let outer = self.outer();
        if outer.is_some() {
            if outer.unwrap() >= self.capacity - 1 {
                None
            } else {
                Some(outer.unwrap() + 1)
            }
        } else {
            Some(0)
        }
    }

    fn removal_distance(&self) -> u64 {
        (self.capacity - self.outer().unwrap()) as u64
    }

    fn insertion_distance(&self) -> u64 {
        (self.capacity - self.next().unwrap()) as u64
    }

    fn remove(&mut self) -> Amphipod {
        let idx = self.outer().unwrap();
        let mut ret = self.content[idx].clone().unwrap();
        self.content[idx] = None;
        ret.has_moved = true;
        ret
    }

    fn insert(&mut self, a: Amphipod) {
        self.content[self.next().unwrap()] = Some(a);
    }

    fn is_dirty(&self) -> bool {
        for a in self.as_vec().iter() {
            if a.is_some() && a.as_ref().unwrap().destination() != self.location {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    rooms: HashMap<RoomLocation, Room>,
    connections: HashMap<RoomLocation, HashSet<RoomLocation>>,
}

impl Map {
    fn new() -> Map {
        let mut rooms = HashMap::new();
        rooms.insert(
            RoomLocation::HallwayLeftLeft,
            Room::single(None, RoomLocation::HallwayLeftLeft),
        );
        rooms.insert(
            RoomLocation::HallwayLeft,
            Room::single(None, RoomLocation::HallwayLeft),
        );
        rooms.insert(
            RoomLocation::HallwayMidLeft,
            Room::single(None, RoomLocation::HallwayMidLeft),
        );
        rooms.insert(
            RoomLocation::HallwayMid,
            Room::single(None, RoomLocation::HallwayMid),
        );
        rooms.insert(
            RoomLocation::HallwayMidRight,
            Room::single(None, RoomLocation::HallwayMidRight),
        );
        rooms.insert(
            RoomLocation::HallwayRight,
            Room::single(None, RoomLocation::HallwayRight),
        );
        rooms.insert(
            RoomLocation::HallwayRightRight,
            Room::single(None, RoomLocation::HallwayRightRight),
        );
        rooms.insert(
            RoomLocation::RoomA,
            Room::double(
                // [Some(Amphipod::new('A')), Some(Amphipod::new('B'))],
                [Some(Amphipod::new('C')), Some(Amphipod::new('B'))],
                RoomLocation::RoomA,
            ),
        );
        rooms.insert(
            RoomLocation::RoomB,
            Room::double(
                // [Some(Amphipod::new('D')), Some(Amphipod::new('C'))],
                [Some(Amphipod::new('A')), Some(Amphipod::new('B'))],
                RoomLocation::RoomB,
            ),
        );
        rooms.insert(
            RoomLocation::RoomC,
            Room::double(
                // [Some(Amphipod::new('C')), Some(Amphipod::new('B'))],
                [Some(Amphipod::new('A')), Some(Amphipod::new('D'))],
                RoomLocation::RoomC,
            ),
        );
        rooms.insert(
            RoomLocation::RoomD,
            Room::double(
                // [Some(Amphipod::new('A')), Some(Amphipod::new('D'))],
                [Some(Amphipod::new('C')), Some(Amphipod::new('D'))],
                RoomLocation::RoomD,
            ),
        );

        let mut connections = HashMap::new();
        for room in rooms.keys() {
            connections.insert(*room, HashSet::new());
        }
        connections
            .get_mut(&RoomLocation::HallwayLeftLeft)
            .unwrap()
            .extend([RoomLocation::HallwayLeft]);
        connections
            .get_mut(&RoomLocation::HallwayLeft)
            .unwrap()
            .extend([RoomLocation::RoomA, RoomLocation::HallwayMidLeft]);
        connections
            .get_mut(&RoomLocation::HallwayMidLeft)
            .unwrap()
            .extend([
                RoomLocation::RoomA,
                RoomLocation::RoomB,
                RoomLocation::HallwayMid,
            ]);
        connections
            .get_mut(&RoomLocation::HallwayMid)
            .unwrap()
            .extend([
                RoomLocation::RoomB,
                RoomLocation::RoomC,
                RoomLocation::HallwayMidRight,
            ]);
        connections
            .get_mut(&RoomLocation::HallwayMidRight)
            .unwrap()
            .extend([
                RoomLocation::RoomC,
                RoomLocation::RoomD,
                RoomLocation::HallwayRight,
            ]);
        connections
            .get_mut(&RoomLocation::HallwayRight)
            .unwrap()
            .extend([RoomLocation::RoomD, RoomLocation::HallwayRightRight]);
        for (&room1, neighbors) in connections.clone().iter() {
            for room2 in neighbors.iter() {
                connections.get_mut(room2).unwrap().insert(room1.clone());
            }
        }

        Map { rooms, connections }
    }

    fn get_moves(&self) -> Vec<(RoomLocation, RoomLocation)> {
        let mut moves = vec![];
        for (&room_loc, room) in self.rooms.iter() {
            let outer = match room.outer() {
                Some(x) => x,
                None => continue,
            };
            let amphipod = room.content[outer].as_ref().unwrap();
            if amphipod.has_moved && room_loc == amphipod.destination() {
                continue;
            }
            for &neighbor_loc in self.connections[&room_loc].iter() {
                let neighbor = &self.rooms[&neighbor_loc];
                if !amphipod.can_enter(neighbor_loc) || !self.rooms[&neighbor_loc].has_space() {
                    continue;
                }
                if neighbor_loc == amphipod.destination() && neighbor.is_dirty() {
                    continue;
                }
                moves.push((room_loc, neighbor_loc));
            }
        }
        moves
    }

    fn execute_move(&mut self, src_loc: RoomLocation, dst_loc: RoomLocation) -> u64 {
        let src = self.rooms.get_mut(&src_loc).unwrap();
        let mut cost = src.removal_distance();
        let amph = src.remove();
        let dst = self.rooms.get_mut(&dst_loc).unwrap();
        cost += dst.insertion_distance();
        cost *= amph.weight();
        dst.insert(amph);
        cost
    }

    fn is_solved(&self) -> bool {
        self.rooms.values().all(|r| r.is_solved())
    }

    fn fmt_amph(&self, x: RoomLocation, y: usize) -> &str {
        match &self.rooms.get(&x).unwrap().content[y] {
            Some(x) => x.to_string(),
            None => ".",
        }
    }

    fn get_state(&self) -> [Option<AmphipodType>; 15] {
        let get_class: fn(&Option<Amphipod>) -> Option<AmphipodType> = |x| match x {
            Some(x) => Some(x.class),
            None => None,
        };
        [
            get_class(&self.rooms[&RoomLocation::HallwayLeftLeft].content[0]),
            get_class(&self.rooms[&RoomLocation::HallwayLeft].content[0]),
            get_class(&self.rooms[&RoomLocation::HallwayMidLeft].content[0]),
            get_class(&self.rooms[&RoomLocation::HallwayMid].content[0]),
            get_class(&self.rooms[&RoomLocation::HallwayMidRight].content[0]),
            get_class(&self.rooms[&RoomLocation::HallwayRight].content[0]),
            get_class(&self.rooms[&RoomLocation::HallwayRightRight].content[0]),
            get_class(&self.rooms[&RoomLocation::RoomA].content[1]),
            get_class(&self.rooms[&RoomLocation::RoomB].content[1]),
            get_class(&self.rooms[&RoomLocation::RoomC].content[1]),
            get_class(&self.rooms[&RoomLocation::RoomD].content[1]),
            get_class(&self.rooms[&RoomLocation::RoomA].content[0]),
            get_class(&self.rooms[&RoomLocation::RoomB].content[0]),
            get_class(&self.rooms[&RoomLocation::RoomC].content[0]),
            get_class(&self.rooms[&RoomLocation::RoomD].content[0]),
        ]
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
            #############\n\
            #{}{}.{}.{}.{}.{}{}#\n\
            ###{}#{}#{}#{}###\n\
            ###{}#{}#{}#{}###\n\
            #############\
            ",
            self.fmt_amph(RoomLocation::HallwayLeftLeft, 0),
            self.fmt_amph(RoomLocation::HallwayLeft, 0),
            self.fmt_amph(RoomLocation::HallwayMidLeft, 0),
            self.fmt_amph(RoomLocation::HallwayMid, 0),
            self.fmt_amph(RoomLocation::HallwayMidRight, 0),
            self.fmt_amph(RoomLocation::HallwayRight, 0),
            self.fmt_amph(RoomLocation::HallwayRightRight, 0),
            self.fmt_amph(RoomLocation::RoomA, 1),
            self.fmt_amph(RoomLocation::RoomB, 1),
            self.fmt_amph(RoomLocation::RoomC, 1),
            self.fmt_amph(RoomLocation::RoomD, 1),
            self.fmt_amph(RoomLocation::RoomA, 0),
            self.fmt_amph(RoomLocation::RoomB, 0),
            self.fmt_amph(RoomLocation::RoomC, 0),
            self.fmt_amph(RoomLocation::RoomD, 0),
        )
    }
}

fn solve_inner(
    map: &Map,
    cost_so_far: u64,
    seen_states: &mut HashMap<[Option<AmphipodType>; 15], u64>,
    previous: Vec<Vec<RoomLocation>>,
) -> Option<u64> {
    if seen_states.len() % 100 == 0 {
        println!("Seen {}", seen_states.len());
    }
    if map.is_solved() {
        return Some(cost_so_far);
    }
    let mut costs = vec![];
    for (src, dst) in map.get_moves() {
        let mut new_map = map.clone();
        let new_cost = cost_so_far + new_map.execute_move(src, dst);
        let new_state = new_map.get_state();
        if seen_states.contains_key(&new_state) && seen_states.get(&new_state).unwrap() <= &new_cost
        {
            continue;
        }
        seen_states.insert(new_state, new_cost);
        let mut moves = previous.clone();
        let last = match previous.last() {
            Some(x) => x.last(),
            None => None,
        };
        if last == Some(&src) {
            moves.last_mut().unwrap().push(dst);
        } else {
            moves.push(vec![src, dst]);
        }
        let total = solve_inner(&new_map, new_cost, seen_states, moves);
        if total.is_some() {
            costs.push(total.unwrap());
        }
    }
    costs.into_iter().min()
}

fn solve(map: &Map) -> u64 {
    let mut seen_states = HashMap::new();
    seen_states.insert(map.get_state(), 0);
    solve_inner(&map, 0, &mut seen_states, vec![]).unwrap()
}

fn main() {
    let mut map = Map::new();
    println!("{}", map);
    println!("{:?}", solve(&map));
}
