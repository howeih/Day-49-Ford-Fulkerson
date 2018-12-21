use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Edge {
    capacity: i32,
    flow: i32,
}

impl Edge {
    fn residual_flow(&self) -> i32 {
        return (self.capacity - self.flow).abs();
    }
}

struct FordFulkerson {
    start: char,
    sink: char,
    network: HashMap<char, HashMap<char, Edge>>,
}

impl FordFulkerson {
    fn new(start: char, sink: char) -> FordFulkerson {
        FordFulkerson {
            start,
            sink,
            network: HashMap::<char, HashMap<char, Edge>>::new(),
        }
    }

    fn add_edge(&mut self, from: char, to: char, capacity: i32) {
        let entry = self
            .network
            .entry(from)
            .or_insert(HashMap::<char, Edge>::new());
        entry.insert(to, Edge { capacity, flow: 0 });
        let entry_return= self.
            network.entry(to)
            .or_insert(HashMap::<char, Edge>::new());
        entry_return.insert(from, Edge{capacity:0, flow:0});
    }

    fn find_argument_path(&self, current: char, mut trace_back: HashMap<char, char>) -> HashMap<char, char> {
        let current_path = self.network.get(&current);
        if let Some(p) = current_path {
            if let Some(p_sink) = p.get(&self.sink) {
                if p_sink.residual_flow() > 0 {
                    trace_back.insert(self.sink,  current);
                    return trace_back;
                }
            }
            for (v, e) in p {
                if e.residual_flow() > 0 && trace_back.iter().fold(true, |y, x| y & (*x.0 != *v)) {
                    trace_back.insert(*v, current);
                    trace_back = self.find_argument_path(*v, trace_back);
                }
            }
        }
        trace_back
    }

    fn find_path(&self) -> Option<VecDeque<char>>{
        let track_back = HashMap::<char, char>::new();
        let path_map = self.find_argument_path(self.start, track_back);
        let mut path = VecDeque::<char>::new();
        if let Some(_) = path_map.iter().find(|k| *((*k).0)==self.sink){
            let mut p = self.sink;
            while p != self.start{
                path.push_front(p);
                p = *path_map.get(&p).unwrap();
            }
            Some(path)
        }else {
            None
        }
    }

    fn find_min_flow(&self, path: &VecDeque<char>) -> i32{
        let mut min_flow = std::i32::MAX;
        let mut from = &self.start;
        for to in path.iter(){
            if let Some(flow) = self.network.get(from).unwrap().get(to) {
                if flow.residual_flow() < min_flow {
                    min_flow = flow.residual_flow();
                }
            }
            from = &to;
        }
        min_flow
    }

    fn flow(&mut self, from: &char, to:&char, flow: i32){
        self.network.get_mut(from).unwrap().get_mut(to).unwrap().flow += flow;
        self.network.get_mut(to).unwrap().get_mut(from).unwrap().flow -= flow;
    }

    fn find_max_flow(&mut self) -> i32 {
        let mut network_flow = 0;
        while let Some(path) = self.find_path() {
            let mut from = self.start;
            let min_flow = self.find_min_flow(&path);
            for to in path.iter(){
                self.flow(&from, to, min_flow);
                from = *to;
            }
            network_flow += min_flow;
        }
        network_flow
    }
}

fn main() {
    let start = 's';
    let sink = 't';
    let mut ford_fulkerson = FordFulkerson::new(start, sink);
    ford_fulkerson.add_edge(start, 'A', 10);
    ford_fulkerson.add_edge(start, 'C', 10);
    ford_fulkerson.add_edge('A', 'B', 4);
    ford_fulkerson.add_edge('A', 'C', 2);
    ford_fulkerson.add_edge('A', 'D', 8);
    ford_fulkerson.add_edge('B', sink, 10);
    ford_fulkerson.add_edge('C', 'D', 9);
    ford_fulkerson.add_edge('D', 'B', 6);
    ford_fulkerson.add_edge('D', sink, 10);
    let max_flow = ford_fulkerson.find_max_flow();
    assert_eq!(max_flow, 19);
}
