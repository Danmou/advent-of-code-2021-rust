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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum AmphipodState {
    NotStarted,
    Active,
    Locked,
    Final,
    Done,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Amphipod {
    class: AmphipodType,
    state: AmphipodState,
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
            state: AmphipodState::NotStarted,
        }
    }

    fn activate(&mut self) {
        self.state = match self.state {
            AmphipodState::NotStarted | AmphipodState::Active => AmphipodState::Active,
            AmphipodState::Locked | AmphipodState::Final => AmphipodState::Final,
            _ => panic!(),
        };
    }

    fn deactivate(&mut self, loc: Loc) -> bool {
        match (self.state, loc) {
            (_, x) if x == self.destination() => {self.state = AmphipodState::Done; true},
            (AmphipodState::Active, Loc::Hallway(_)) => {self.state = AmphipodState::Locked; true},
            _ => false,
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

    fn destination(&self) -> Loc {
        match self.class {
            AmphipodType::A => Loc::Room(RoomLoc::A),
            AmphipodType::B => Loc::Room(RoomLoc::B),
            AmphipodType::C => Loc::Room(RoomLoc::C),
            AmphipodType::D => Loc::Room(RoomLoc::D),
        }
    }

    fn can_enter(&self, r: Loc) -> bool {
        match r {
            Loc::Hallway(_) => true,
            x if x == self.destination() => true,
            _ => false,
        }
    }
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, Hash, Copy, Clone)]
enum HallwayLoc {
    LeftLeft,
    Left,
    MidLeft,
    Mid,
    MidRight,
    Right,
    RightRight,
}

#[derive(Debug, Display, EnumString, PartialEq, Eq, Hash, Copy, Clone)]
enum RoomLoc {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Display, PartialEq, Eq, Hash, Copy, Clone)]
enum Loc {
    Hallway(HallwayLoc),
    Room(RoomLoc),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Room {
    capacity: usize,
    content: [Option<Amphipod>; 4],
    location: Loc,
}

impl Room {
    fn single(content: Option<Amphipod>, location: Loc) -> Room {
        Room {
            capacity: 1,
            content: [content, None, None, None],
            location,
        }
    }

    fn quad(content: [Option<Amphipod>; 4], location: Loc) -> Room {
        Room {
            capacity: 4,
            content,
            location,
        }
    }

    fn has_space(&self) -> bool {
        match self.capacity {
            1 => self.content[0].is_none(),
            2 => self.content[1].is_none(),
            4 => self.content[3].is_none(),
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
            Loc::Room(_) => all_at_dest,
            _ => self.outer().is_none(),
        }
    }

    fn outer(&self) -> Option<usize> {
        for i in (0..self.capacity).rev() {
            if self.content[i].is_some() {
                return Some(i);
            }
        }
        None
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
        match self.location {
            Loc::Hallway(HallwayLoc::LeftLeft) | Loc::Hallway(HallwayLoc::RightRight) => 0,
            _ => (self.capacity - self.outer().unwrap()) as u64,
        }
    }

    fn insertion_distance(&self) -> u64 {
        match self.location {
            Loc::Hallway(HallwayLoc::LeftLeft) | Loc::Hallway(HallwayLoc::RightRight) => 0,
            _ => (self.capacity - self.next().unwrap()) as u64,
        }
    }

    fn remove(&mut self) -> Amphipod {
        let idx = self.outer().unwrap();
        let mut ret = self.content[idx].clone().unwrap();
        self.content[idx] = None;
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
    rooms: HashMap<Loc, Room>,
    connections: HashMap<Loc, HashSet<Loc>>,
    last_moved: Option<Loc>,
}

impl Map {
    fn new() -> Map {
        let mut rooms = HashMap::new();
        rooms.insert(
            Loc::Hallway(HallwayLoc::LeftLeft),
            Room::single(None, Loc::Hallway(HallwayLoc::LeftLeft)),
        );
        rooms.insert(
            Loc::Hallway(HallwayLoc::Left),
            Room::single(None, Loc::Hallway(HallwayLoc::Left)),
        );
        rooms.insert(
            Loc::Hallway(HallwayLoc::MidLeft),
            Room::single(None, Loc::Hallway(HallwayLoc::MidLeft)),
        );
        rooms.insert(
            Loc::Hallway(HallwayLoc::Mid),
            Room::single(None, Loc::Hallway(HallwayLoc::Mid)),
        );
        rooms.insert(
            Loc::Hallway(HallwayLoc::MidRight),
            Room::single(None, Loc::Hallway(HallwayLoc::MidRight)),
        );
        rooms.insert(
            Loc::Hallway(HallwayLoc::Right),
            Room::single(None, Loc::Hallway(HallwayLoc::Right)),
        );
        rooms.insert(
            Loc::Hallway(HallwayLoc::RightRight),
            Room::single(None, Loc::Hallway(HallwayLoc::RightRight)),
        );
        rooms.insert(
            Loc::Room(RoomLoc::A),
            Room::quad(
                [
                    Some(Amphipod::new('C')),
                    // Some(Amphipod::new('A')),
                    Some(Amphipod::new('D')),
                    Some(Amphipod::new('D')),
                    Some(Amphipod::new('B')),
                    // Some(Amphipod::new('B')),
                ],
                Loc::Room(RoomLoc::A),
            ),
        );
        rooms.insert(
            Loc::Room(RoomLoc::B),
            Room::quad(
                [
                    Some(Amphipod::new('A')),
                    // Some(Amphipod::new('D')),
                    Some(Amphipod::new('B')),
                    Some(Amphipod::new('C')),
                    Some(Amphipod::new('B')),
                    // Some(Amphipod::new('C')),
                ],
                Loc::Room(RoomLoc::B),
            ),
        );
        rooms.insert(
            Loc::Room(RoomLoc::C),
            Room::quad(
                [
                    Some(Amphipod::new('A')),
                    // Some(Amphipod::new('C')),
                    Some(Amphipod::new('A')),
                    Some(Amphipod::new('B')),
                    Some(Amphipod::new('D')),
                    // Some(Amphipod::new('B')),
                ],
                Loc::Room(RoomLoc::C),
            ),
        );
        rooms.insert(
            Loc::Room(RoomLoc::D),
            Room::quad(
                [
                    Some(Amphipod::new('C')),
                    // Some(Amphipod::new('A')),
                    Some(Amphipod::new('C')),
                    Some(Amphipod::new('A')),
                    Some(Amphipod::new('D')),
                    // Some(Amphipod::new('D')),
                ],
                Loc::Room(RoomLoc::D),
            ),
        );

        let mut connections = HashMap::new();
        for room in rooms.keys() {
            connections.insert(*room, HashSet::new());
        }
        connections
            .get_mut(&Loc::Hallway(HallwayLoc::LeftLeft))
            .unwrap()
            .extend([Loc::Hallway(HallwayLoc::Left)]);
        connections
            .get_mut(&Loc::Hallway(HallwayLoc::Left))
            .unwrap()
            .extend([Loc::Room(RoomLoc::A), Loc::Hallway(HallwayLoc::MidLeft)]);
        connections
            .get_mut(&Loc::Hallway(HallwayLoc::MidLeft))
            .unwrap()
            .extend([
                Loc::Room(RoomLoc::A),
                Loc::Room(RoomLoc::B),
                Loc::Hallway(HallwayLoc::Mid),
            ]);
        connections
            .get_mut(&Loc::Hallway(HallwayLoc::Mid))
            .unwrap()
            .extend([
                Loc::Room(RoomLoc::B),
                Loc::Room(RoomLoc::C),
                Loc::Hallway(HallwayLoc::MidRight),
            ]);
        connections
            .get_mut(&Loc::Hallway(HallwayLoc::MidRight))
            .unwrap()
            .extend([
                Loc::Room(RoomLoc::C),
                Loc::Room(RoomLoc::D),
                Loc::Hallway(HallwayLoc::Right),
            ]);
        connections
            .get_mut(&Loc::Hallway(HallwayLoc::Right))
            .unwrap()
            .extend([Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::RightRight)]);
        for (&room1, neighbors) in connections.clone().iter() {
            for room2 in neighbors.iter() {
                connections.get_mut(room2).unwrap().insert(room1.clone());
            }
        }

        Map {
            rooms,
            connections,
            last_moved: None,
        }
    }

    fn get_moves(&self) -> Vec<(Loc, Loc)> {
        let mut moves = vec![];
        for (&room_loc, room) in self.rooms.iter() {
            let outer = match room.outer() {
                Some(x) => x,
                None => continue,
            };
            let amphipod = room.content[outer].as_ref().unwrap();
            if amphipod.state == AmphipodState::Done {
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

    fn execute_move(&mut self, src_loc: Loc, dst_loc: Loc) -> u64 {
        if !self.get_moves().contains(&(src_loc, dst_loc)) {
            println!("{:?}", self.get_moves());
            panic!("Invalid move: {:?}->{:?}", src_loc, dst_loc);
        }
        let src = self.rooms.get_mut(&src_loc).unwrap();
        let mut cost = src.removal_distance();
        let mut amph = src.remove();
        let dst = self.rooms.get_mut(&dst_loc).unwrap();
        cost += dst.insertion_distance();
        cost *= amph.weight();
        dst.insert(amph);
        self.last_moved = Some(dst_loc);
        cost
    }

    fn execute_moves(&mut self, steps: Vec<Loc>) -> u64 {
        let mut cost = 0;
        self.activate(*steps.first().unwrap());
        for (&src, &dst) in steps[..steps.len() - 1].iter().zip(steps[1..].iter()) {
            cost += self.execute_move(src, dst);
        }
        if !self.deactivate(*steps.last().unwrap()) {
            panic!("Invalid move sequence: {:?}", steps);
        }
        cost
    }

    fn is_solved(&self) -> bool {
        self.rooms.values().all(|r| r.is_solved())
    }

    fn fmt_amph(&self, x: Loc, y: usize) -> &str {
        match &self.rooms.get(&x).unwrap().content[y] {
            Some(x) => x.to_string(),
            None => ".",
        }
    }

    fn get_state(&self) -> [Option<Amphipod>; 23] {
        [
            self.rooms[&Loc::Hallway(HallwayLoc::LeftLeft)].content[0].clone(),
            self.rooms[&Loc::Hallway(HallwayLoc::Left)].content[0].clone(),
            self.rooms[&Loc::Hallway(HallwayLoc::MidLeft)].content[0].clone(),
            self.rooms[&Loc::Hallway(HallwayLoc::Mid)].content[0].clone(),
            self.rooms[&Loc::Hallway(HallwayLoc::MidRight)].content[0].clone(),
            self.rooms[&Loc::Hallway(HallwayLoc::Right)].content[0].clone(),
            self.rooms[&Loc::Hallway(HallwayLoc::RightRight)].content[0].clone(),
            self.rooms[&Loc::Room(RoomLoc::A)].content[3].clone(),
            self.rooms[&Loc::Room(RoomLoc::B)].content[3].clone(),
            self.rooms[&Loc::Room(RoomLoc::C)].content[3].clone(),
            self.rooms[&Loc::Room(RoomLoc::D)].content[3].clone(),
            self.rooms[&Loc::Room(RoomLoc::A)].content[2].clone(),
            self.rooms[&Loc::Room(RoomLoc::B)].content[2].clone(),
            self.rooms[&Loc::Room(RoomLoc::C)].content[2].clone(),
            self.rooms[&Loc::Room(RoomLoc::D)].content[2].clone(),
            self.rooms[&Loc::Room(RoomLoc::A)].content[1].clone(),
            self.rooms[&Loc::Room(RoomLoc::B)].content[1].clone(),
            self.rooms[&Loc::Room(RoomLoc::C)].content[1].clone(),
            self.rooms[&Loc::Room(RoomLoc::D)].content[1].clone(),
            self.rooms[&Loc::Room(RoomLoc::A)].content[0].clone(),
            self.rooms[&Loc::Room(RoomLoc::B)].content[0].clone(),
            self.rooms[&Loc::Room(RoomLoc::C)].content[0].clone(),
            self.rooms[&Loc::Room(RoomLoc::D)].content[0].clone(),
        ]
    }

    fn activate(&mut self, loc: Loc) {
        let room = self.rooms.get_mut(&loc).unwrap();
        let idx = room.outer().unwrap();
        room.content.get_mut(idx).unwrap().as_mut().unwrap().activate();
    }

    fn deactivate(&mut self, loc: Loc) -> bool {
        let room = self.rooms.get_mut(&loc).unwrap();
        let idx = room.outer().unwrap();
        room.content.get_mut(idx).unwrap().as_mut().unwrap().deactivate(loc)
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
            ###{}#{}#{}#{}###\n\
            ###{}#{}#{}#{}###\n\
            #############\
            ",
            self.fmt_amph(Loc::Hallway(HallwayLoc::LeftLeft), 0),
            self.fmt_amph(Loc::Hallway(HallwayLoc::Left), 0),
            self.fmt_amph(Loc::Hallway(HallwayLoc::MidLeft), 0),
            self.fmt_amph(Loc::Hallway(HallwayLoc::Mid), 0),
            self.fmt_amph(Loc::Hallway(HallwayLoc::MidRight), 0),
            self.fmt_amph(Loc::Hallway(HallwayLoc::Right), 0),
            self.fmt_amph(Loc::Hallway(HallwayLoc::RightRight), 0),
            self.fmt_amph(Loc::Room(RoomLoc::A), 3),
            self.fmt_amph(Loc::Room(RoomLoc::B), 3),
            self.fmt_amph(Loc::Room(RoomLoc::C), 3),
            self.fmt_amph(Loc::Room(RoomLoc::D), 3),
            self.fmt_amph(Loc::Room(RoomLoc::A), 2),
            self.fmt_amph(Loc::Room(RoomLoc::B), 2),
            self.fmt_amph(Loc::Room(RoomLoc::C), 2),
            self.fmt_amph(Loc::Room(RoomLoc::D), 2),
            self.fmt_amph(Loc::Room(RoomLoc::A), 1),
            self.fmt_amph(Loc::Room(RoomLoc::B), 1),
            self.fmt_amph(Loc::Room(RoomLoc::C), 1),
            self.fmt_amph(Loc::Room(RoomLoc::D), 1),
            self.fmt_amph(Loc::Room(RoomLoc::A), 0),
            self.fmt_amph(Loc::Room(RoomLoc::B), 0),
            self.fmt_amph(Loc::Room(RoomLoc::C), 0),
            self.fmt_amph(Loc::Room(RoomLoc::D), 0),
        )
    }
}

fn solve_inner(
    map: &Map,
    cost_so_far: u64,
    seen_states: &mut HashMap<[Option<Amphipod>; 23], u64>,
    previous: Vec<Vec<Loc>>,
    best: &mut Option<(u64, Vec<Vec<Loc>>)>,
) {
    if map.is_solved() {
        if best.is_none() || cost_so_far < best.as_ref().unwrap().0 {
            println!("New best: {}", cost_so_far);
            *best = Some((cost_so_far, previous));
        }
        return;
    }
    let last = match previous.last() {
        Some(x) => x.last(),
        None => None,
    };
    for (src, dst) in map.get_moves() {
        let mut new_map = map.clone();
        if last != Some(&src) {
            if last.is_some() && !new_map.deactivate(*last.unwrap()) {
                continue;
            }
            new_map.activate(src);
        }
        let new_cost = cost_so_far + new_map.execute_move(src, dst);
        if best.is_some() && new_cost >= best.as_ref().unwrap().0 {
            continue;
        }
        let new_state = new_map.get_state();
        if seen_states.contains_key(&new_state) {
            if seen_states.get(&new_state).unwrap() <= &new_cost {
                continue;
            }
        } else if seen_states.len() % 1000 == 0 {
            println!("Seen {}", seen_states.len());
        }
        seen_states.insert(new_state, new_cost);
        let mut moves = previous.clone();
        if last == Some(&src) {
            moves.last_mut().unwrap().push(dst);
        } else {
            moves.push(vec![src, dst]);
        }
        solve_inner(&new_map, new_cost, seen_states, moves, best);
    }
}

fn solve_initial_guess(map: &Map, initial_guess: Option<u64>) -> (u64, Vec<Vec<Loc>>) {
    let mut seen_states = HashMap::new();
    seen_states.insert(map.get_state(), 0);
    let mut best = if initial_guess.is_none() {
        None
    } else {
        Some((initial_guess.unwrap(), vec![]))
    };
    solve_inner(&map, 0, &mut seen_states, vec![], &mut best);
    best.unwrap()
}

fn solve(map: &Map) -> (u64, Vec<Vec<Loc>>) {
    solve_initial_guess(&map, None)
}

fn main() {
    let mut map = Map::new();
    println!("{}", map);
    let mut cost = 0;

    // cost += map.execute_move(Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::Right));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Right), Loc::Hallway(HallwayLoc::RightRight));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Hallway(HallwayLoc::MidLeft));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Hallway(HallwayLoc::Left));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Left), Loc::Hallway(HallwayLoc::LeftLeft));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::C), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Hallway(HallwayLoc::Right));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::C), Loc::Hallway(HallwayLoc::MidRight));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::C), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Hallway(HallwayLoc::MidLeft));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Hallway(HallwayLoc::Left));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::B), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Room(RoomLoc::C));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::B), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Room(RoomLoc::C));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::B), Loc::Hallway(HallwayLoc::Mid));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::B), Loc::Hallway(HallwayLoc::MidLeft));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Room(RoomLoc::B));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Room(RoomLoc::B));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Right), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Room(RoomLoc::B));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Room(RoomLoc::C));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::Right));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Room(RoomLoc::D));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::A), Loc::Hallway(HallwayLoc::MidLeft));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Room(RoomLoc::B));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::A), Loc::Hallway(HallwayLoc::MidLeft));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Room(RoomLoc::D));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Room(RoomLoc::A), Loc::Hallway(HallwayLoc::MidLeft));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Left), Loc::Room(RoomLoc::A));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::LeftLeft), Loc::Hallway(HallwayLoc::Left));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Left), Loc::Room(RoomLoc::A));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Room(RoomLoc::D));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Right), Loc::Hallway(HallwayLoc::MidRight));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidRight), Loc::Hallway(HallwayLoc::Mid));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Mid), Loc::Hallway(HallwayLoc::MidLeft));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::MidLeft), Loc::Room(RoomLoc::A));
    // println!("{}", map);
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::RightRight), Loc::Hallway(HallwayLoc::Right));
    // cost += map.execute_move(Loc::Hallway(HallwayLoc::Right), Loc::Room(RoomLoc::D));
    // println!("{}", map);
    // println!("{:?}", map.get_moves());
    // println!("{:?}", map.rooms[&Loc::Hallway(HallwayLoc::LeftLeft)].content);
    // println!("{:?}", map.rooms[&Loc::Hallway(HallwayLoc::LeftLeft)].has_space());

    // let moves = vec![
    //     vec![Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::Right)],
    //     vec![Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::MidRight)],
    //     vec![Loc::Room(RoomLoc::A), Loc::Hallway(HallwayLoc::Left)],
    //     vec![
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //     ],
    //     vec![
    //         Loc::Room(RoomLoc::D),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Mid),
    //     ],
    //     vec![Loc::Room(RoomLoc::D), Loc::Hallway(HallwayLoc::MidRight)],
    //     vec![Loc::Hallway(HallwayLoc::Right), Loc::Room(RoomLoc::D)],
    //     vec![
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Right),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Right),
    //         Loc::Hallway(HallwayLoc::RightRight),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Right),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Left),
    //         Loc::Hallway(HallwayLoc::LeftLeft),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Hallway(HallwayLoc::Left),
    //     ],
    //     vec![
    //         Loc::Room(RoomLoc::A),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //     ],
    //     vec![
    //         Loc::Room(RoomLoc::A),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Hallway(HallwayLoc::Mid),
    //     ],
    //     vec![Loc::Hallway(HallwayLoc::MidRight), Loc::Room(RoomLoc::D)],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Room(RoomLoc::D),
    //     ],
    //     vec![Loc::Hallway(HallwayLoc::Left), Loc::Room(RoomLoc::A)],
    //     vec![Loc::Room(RoomLoc::C), Loc::Hallway(HallwayLoc::MidRight)],
    //     vec![
    //         Loc::Room(RoomLoc::C),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Hallway(HallwayLoc::Left),
    //     ],
    //     vec![
    //         Loc::Room(RoomLoc::C),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Room(RoomLoc::A),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Mid),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Right),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Room(RoomLoc::C),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Right),
    //     ],
    //     vec![
    //         Loc::Room(RoomLoc::B),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Room(RoomLoc::C),
    //     ],
    //     vec![
    //         Loc::Room(RoomLoc::B),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Room(RoomLoc::C),
    //     ],
    //     vec![Loc::Room(RoomLoc::B), Loc::Hallway(HallwayLoc::MidLeft)],
    //     vec![
    //         Loc::Room(RoomLoc::B),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //     ],
    //     vec![Loc::Hallway(HallwayLoc::MidLeft), Loc::Room(RoomLoc::B)],
    //     vec![Loc::Hallway(HallwayLoc::MidRight), Loc::Room(RoomLoc::D)],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Right),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Room(RoomLoc::B),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::Left),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Room(RoomLoc::B),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::LeftLeft),
    //         Loc::Hallway(HallwayLoc::Left),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Room(RoomLoc::B),
    //     ],
    //     vec![
    //         Loc::Hallway(HallwayLoc::RightRight),
    //         Loc::Hallway(HallwayLoc::Right),
    //         Loc::Hallway(HallwayLoc::MidRight),
    //         Loc::Hallway(HallwayLoc::Mid),
    //         Loc::Hallway(HallwayLoc::MidLeft),
    //         Loc::Room(RoomLoc::A),
    //     ],
    // ];
    // for m in moves.into_iter() {
    //     cost += map.execute_moves(m);
    //     println!("{}", cost);
    //     println!("{}", map);
    // }

    // let (c, m) = solve_initial_guess(&map, Some(45000));
    let (c, m) = solve(&map);
    println!("{}", cost + c);
    println!("{:?}", m);
}
