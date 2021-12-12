use std::fs::File;
use std::io::{BufRead, BufReader};

use graphlib::{Graph, VertexId};

use std::collections::HashMap;
use std::hash::Hash;

pub struct BidirectionalMap<K, V> {
    right_to_left: HashMap<K, V>,
    left_to_right: HashMap<V, K>,
}

impl<K, V> BidirectionalMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    pub fn new() -> BidirectionalMap<K, V> {
        BidirectionalMap {
            right_to_left: HashMap::new(),
            left_to_right: HashMap::new(),
        }
    }

    pub fn remove_by_key(&mut self, k: K) -> Option<V> {
        match self.right_to_left.remove(&k) {
            Some(v) => {
                let _ = self.left_to_right.remove(&v);
                return Some(v);
            }
            None => return None,
        }
    }

    fn remove_by_val(&mut self, v: V) -> Option<K> {
        match self.left_to_right.remove(&v) {
            Some(k) => {
                let _ = self.right_to_left.remove(&k);
                return Some(k);
            }
            None => return None,
        }
    }

    pub fn get_value(&self, key: K) -> Option<&V> {
        self.right_to_left.get(&key)
    }

    pub fn get_key(&self, val: V) -> Option<&K> {
        self.left_to_right.get(&val)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let _ = self.left_to_right.insert(v.clone(), k.clone());
        self.right_to_left.insert(k, v)
    }
}

fn get_or_add_vertex(
    name: &String,
    graph: &mut Graph<bool>,
    id_to_name_map: &mut BidirectionalMap<VertexId, String>,
) -> VertexId {
    match id_to_name_map.get_key(name.clone()) {
        Some(x) => *x,
        None => {
            let large = name.chars().all(char::is_uppercase);
            let id = graph.add_vertex(large);
            id_to_name_map.insert(id, name.clone());
            id
        }
    }
}

fn read_input() -> (Graph<bool>, BidirectionalMap<VertexId, String>) {
    let file = File::open("inputs/12.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut graph = Graph::new();
    let mut id_to_name_map = BidirectionalMap::new();

    for line in br.lines() {
        let line_ = line.unwrap();
        let (start, end) = match line_.split("-").collect::<Vec<_>>()[..] {
            [a, b] => (a.to_string(), b.to_string()),
            _ => panic!(),
        };
        let id1 = get_or_add_vertex(&start, &mut graph, &mut id_to_name_map);
        let id2 = get_or_add_vertex(&end, &mut graph, &mut id_to_name_map);
        graph.add_edge(&id1, &id2);
    }

    (graph, id_to_name_map)
}

fn can_visit(graph: &Graph<bool>, path: &Vec<VertexId>, vertex: &VertexId) -> bool {
    if *graph.fetch(vertex).unwrap() {
        // large
        return true;
    }
    return !path.contains(vertex);
}

fn find_paths(graph: &Graph<bool>, &start: &VertexId, &end: &VertexId) -> Vec<Vec<VertexId>> {
    let mut paths = Vec::new();
    let mut queue = vec![vec![start]];

    while !queue.is_empty() {
        let mut path = queue.pop().unwrap();
        let path_end = path.last().unwrap();
        if *path_end == end && !paths.contains(&path) {
            paths.push(path);
            continue;
        }
        for v in graph.neighbors(path_end) {
            if !can_visit(graph, &path, v) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(*v);
            queue.push(new_path);
        }
    }

    paths
}

fn print_paths(paths: &Vec<Vec<VertexId>>, id_to_name_map: &BidirectionalMap<VertexId, String>) {
    for path in paths.iter() {
        println!("{}", path.iter().map(|&v| id_to_name_map.get_value(v).unwrap().clone()).collect::<Vec<String>>().join("-"))
    }
}

fn main() {
    let (graph, id_to_name_map) = read_input();
    let start = id_to_name_map.get_key("start".to_string()).unwrap();
    let end = id_to_name_map.get_key("end".to_string()).unwrap();
    let paths = find_paths(&graph, start, end);
    print_paths(&paths, &id_to_name_map);
    println!("{}", paths.len());
}
