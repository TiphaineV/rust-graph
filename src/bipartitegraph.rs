use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter;	

use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub type Node = i64;
pub type Edge = Vec<Node>; // bof, on n'impose pas un vec de taille 2 ? Et ça fait tjs la diff entre (u,v) et (v,u)...


pub struct BipartiteGraph {
	// scores: HashMap<Vec<i64>, f64 >,
	scores: HashMap<Edge, f64 >,
	pub neighbors_top: HashMap<i64, BTreeSet<i64> >,
	pub neighbors_bot: HashMap<i64, BTreeSet<i64> >,
}

impl BipartiteGraph {
	pub fn new() -> BipartiteGraph {
		BipartiteGraph {
			scores: HashMap::new(),
			neighbors_top: HashMap::new(),
			neighbors_bot: HashMap::new(),
		}
	}

	pub fn scores(&self) -> &HashMap<Edge, f64 > {
		&self.scores
	}

	pub fn nodes_top(&self) -> Vec<&Node> {
		let k: Vec<&Node> = self.neighbors_top.keys().collect();
		k
	}

	pub fn nodes_bot(&self) -> Vec<&Node> {
		let k: Vec<&Node> = self.neighbors_bot.keys().collect();
		k
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

			let u: Node = match contents[0].parse() {
				Ok(num) => num,
				Err(why) => panic!("panicked u: {}", why),
			};
			let v: Node = match contents[1].parse() {
				Ok(num) => num,
				Err(why) => panic!("panicked v: {}", why),
			};

			let s: f64 = match contents[2].parse() {
				Ok(num) => num,
				Err(why) => panic!("panicked score: {}", why),
			};

			let mut edge = Edge::new();
			edge.push(u);
			edge.push(v);

			self.scores.insert(edge, s as f64);

			{
				let mut n = self.neighbors_top.entry(u).or_insert_with(|| BTreeSet::new());
				n.insert(v);
			}
			{
				let mut n = self.neighbors_bot.entry(v).or_insert_with(|| BTreeSet::new());
				n.insert(u);
			}

			// if old_u != u && old_u != -1 {
			// 	println!("Changed node.");
			// }
			// old_u = u;
			// println!("{} {}", u, v);
		}

	}

	pub fn degrees_top(&self) -> HashMap<&Node, i64> {
		// Returns users degrees

		let mut d: HashMap<&Node, i64> = HashMap::new();
		// &self.neighbors_top.iter().map(|(a,b)| d.insert(a, b.len() as i64));
		
		for (node, neigh) in &self.neighbors_top {
			d.insert(node, neigh.len() as i64);
		}

		d
	}

	pub fn degrees_bot(&self) -> HashMap<&Node, i64> {
		// Returns movie degrees


		let mut d: HashMap<&Node, i64> = HashMap::new();
		// &self.neighbors_top.iter().map(|(a,b)| d.insert(a, b.len() as i64));

		for (node, neigh) in &self.neighbors_bot {
			d.insert(node, neigh.len() as i64);
		}

		d
	}

	pub fn neighbours_top(&self, u: Node) -> &BTreeSet<Node> {
		// Takes a node u in top, and returns its neighbours in bot.
		// Handle if entry does not exist ?
		&self.neighbors_top[&u]
	}

	pub fn neighbours_bot(&self, u: Node) -> &BTreeSet<Node> {
		// Takes a node u in bot, and returns its neighbours in top.
		// Handle if entry does not exist ?
		&self.neighbors_bot[&u]
	}

	pub fn cover_bot(&self, threshold: f64) -> i64 {
		// Returns percent of movies necessary to cover all users.
		// Adapt with a threshold ? i.e. percent of movies to cover 0.9 of users, etc.

		let mut nodes: Vec<(&i64, i64)> = self.degrees_bot().into_iter().collect();
		let mut covered: BTreeSet<i64> = BTreeSet::new();
		let total_nodes_to_cover = self.nodes_top().len();

		nodes.sort_by(|a, &b| a.1.cmp(&b.1).reverse());
		
		let mut i: i64 = 0;
		while (covered.len() as f64 / total_nodes_to_cover as f64) < threshold {
			// Add neighbours of node of i-th decreasing degree as "covered"
			let my_node: &i64 = nodes[i as usize].0;
			
			covered.extend(self.neighbours_bot(*my_node).iter());

			// for n in self.neighbours_bot(*my_node) {
			// 	covered.insert(*n);
			// }
			i = i + 1;
		}

		i
	}

	pub fn cover_top(&self, threshold: f64) -> i64 {
		// Returns percent of users necessary to cover all movies.
		// Adapt with a threshold ? i.e. percent of movies to cover 0.9 of users, etc.

		let mut nodes: Vec<(&i64, i64)> = self.degrees_top().into_iter().collect();
		let mut covered: BTreeSet<i64> = BTreeSet::new();
		let total_nodes_to_cover = self.nodes_bot().len();

		nodes.sort_by(|a, &b| a.1.cmp(&b.1).reverse());
		
		let mut i: i64 = 0;
		while (covered.len() as f64 / total_nodes_to_cover as f64) < threshold {
			// Add neighbours of node of i-th decreasing degree as "covered"
			let my_node: &i64 = nodes[i as usize].0;
			for n in self.neighbours_top(*my_node) {
				covered.insert(*n);
			}
			i = i + 1;
		}

		i
	}

	pub fn projection_top(&self) -> HashMap<Node, BTreeSet<(Node, i64, i64)> > {
		let mut cpt:i32 = 0;
		// Modify to make a Graph, how to store Jaccard, as weights ?
		let mut graph: HashMap<Node, BTreeSet< (Node, i64, i64) > > = HashMap::new();
		// or maybe weights: HashMap< Edge<i64,i64>, f64> ? 

		for i in self.nodes_bot().iter() {
			if cpt % 1 == 0 {
				println_stderr!("{}/{}", cpt, self.nodes_bot().len());
			}
			cpt = cpt + 1;

			let mut v: Vec<_> = Vec::new();
			
			for j in self.neighbours_bot(**i) {
				v.push(*j);
			}
			// Pour toutes les paires dans v
			for (k, l) in v.iter()
			        .enumerate()
			        .flat_map(|(i, val)| iter::repeat(val).zip(v.iter().skip(i + 1)))
			{

				let n_k = &self.neighbours_top(*k); 
				let n_l = &self.neighbours_top(*l);

				if *k < *l {
					let mut n = graph.entry(*k).or_insert_with(|| BTreeSet::new());
					n.insert((*l, n_k.intersection(&n_l).count() as i64, n_k.union(&n_l).count() as i64));
					// println!("{} {}", *k,*l);
				}
				else {
					let mut n = graph.entry(*l).or_insert_with(|| BTreeSet::new());
					n.insert((*k, n_k.intersection(&n_l).count() as i64, n_k.union(&n_l).count() as i64));
					// println!("{} {}", *k,*l);
				}
			}
		}
		// println!("{:?}", graph); // Weighted... ?
		graph
	}

	pub fn projection_top_2(&self) {
		/*
		 * Outputs the top-projection of the graph (no stored to avoid out of memory).
		 * For each node i in top, and then for all j\ne i, the function checks that 
		 * there is a path of length 2 between i and j. 
		 * Once this is done for all j, i is removed from the set of nodes, to avoid 
		 * redundancy in output.
		 */

		let mut nodes: Vec<&Node> = self.nodes_top().into_iter().collect();

		while let Some(i) = nodes.pop() {
			
			println_stderr!("Remaining nodes: {}", nodes.len());

			for j in &nodes {
				let mut are_linked = false;
				for u in self.neighbours_top(*i) {
					for w in self.neighbours_bot(*u) {
						if *w == **j {
							are_linked = true;
							break;
						}
					}

					// needed ? 
					if are_linked {
						break;
					}
				}
			
				// If there is a path of length 2 between i and j
				if are_linked {
					println!("{} {}", i, j);
				}

			}
		}
		// println!("{:?}", nodes);

	}

	pub fn density(&self) -> f64 {
		let mut m = 0;

		for deg in self.degrees_top().values() {
			m += *deg;
		}

		let n_top = self.nodes_top().len() as f64;
		let n_bot = self.nodes_bot().len() as f64;

		(m as f64) / (n_top * n_bot)
	}
}