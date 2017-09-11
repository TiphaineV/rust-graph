use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;


pub struct Graph {
	edges: BTreeSet< BTreeSet<i64> >,
	// neighbors: HashMap<i64, i64>
	neighbors: HashMap<i64, BTreeSet<i64> >

}

impl Graph {
	pub fn new() -> Graph {
		Graph {
			edges: BTreeSet::new(),
			neighbors: HashMap::new()
		}
	}

	pub fn nodes(&self) -> Vec<&i64> {
		self.neighbors.keys().collect()
	}

	pub fn edges(&self) -> &BTreeSet< BTreeSet<i64> > {
		&self.edges
	}

	pub fn read_from_file(&mut self, fname: &String) {
		let file = match File::open(fname) {
			Ok(file) => file,
			Err(why) => panic!("panicked: {}", why)
		};

		let reader = BufReader::new(&file);

		for line in reader.lines() {
			let l = line.unwrap();
			let contents: Vec<&str> = l
									 .trim()
									 .split("	")
									 .collect();

			let u: i64 = match contents[0].parse() {
				Ok(num) => num,
				Err(why) => panic!("panicked: {}", why),
			};
			let v: i64 = match contents[1].parse() {
				Ok(num) => num,
				Err(why) => panic!("panicked: {}", why),
			};

			// self.nodes.insert(u);
			// self.nodes.insert(v);

			let mut edge = BTreeSet::new();
			edge.insert(u);
			edge.insert(v);
			self.edges.insert(edge);

			{
				let mut n = self.neighbors.entry(u).or_insert_with(|| BTreeSet::new());
				n.insert(v);
			}
			{
				let mut n = self.neighbors.entry(v).or_insert_with(|| BTreeSet::new());
				n.insert(u);
			}

			// if old_u != u && old_u != -1 {
			// 	println!("Changed node.");
			// }
			// old_u = u;
			// println!("{} {}", u, v);
		}

	}

	pub fn degrees(&self) -> HashMap<&i64,i64> {
		let mut d: HashMap<&i64, i64> = HashMap::new();

		for (node, neigh) in &self.neighbors {
			d.insert(node, neigh.len() as i64);
		}

		d
	}

	pub fn neighbours(&self, u: i64) -> &BTreeSet<i64> {
		&self.neighbors[&u]
	}

	pub fn get_distance(&self, u: i64, v: i64) -> i64 {
		// Perform BFS to find distance between u and v

		// Vérifier que u et v sont bien présents ?

		let mut f: VecDeque<i64> = VecDeque::new();
		let mut visited: BTreeSet<i64> = BTreeSet::new();
		let mut found = false;
		let mut d = 0;

		f.push_front(u);
		visited.insert(u);

		while !found && !f.is_empty() {
			// Tant qu'on a pas trouvé le noeud et qu'il y a encore à explorer
			let node = f.pop_back().unwrap();

			println!("{:?}", f);

			for n in self.neighbours(node) {
				if !visited.contains(n) {
					f.push_front(*n);
					visited.insert(*n);
				}
			}

			if self.neighbours(node).contains(&v) {
				found = true;
			}

			d += 1;
		}

		if !found {
			d = 0;
		} 

		d
	}

	pub fn average_degree(&self) -> f64 {
		let mut sum_degs: i64 = 0;
		let mut num_nodes: i64 = 0;

		for val in self.neighbors.values() {
			sum_degs = sum_degs + (val.len() as i64);
			num_nodes = num_nodes + 1;
		}

		(sum_degs as f64) / (num_nodes as f64)
	}

	pub fn density(&self) -> f64 {
		let m = self.edges().len() as f64;
		let n = self.nodes().len() as f64;

		(2.0 * m) / (n * (n - 1.0))
	}

	pub fn connected_components(&self) {

	}
}