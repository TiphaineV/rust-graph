extern crate graph;
extern crate rand;

use rand::{thread_rng,sample};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::env;

pub use graph::graph::Graph;
pub use graph::bipartitegraph::BipartiteGraph;


fn score(rs: &Graph, bip: &BipartiteGraph, u: i64, f:i64) -> f64 {
	// If u has seen movie, return the score assigned
	// Else return estimated score.

	// neighbours should return BTreeSet of nodes.
	// Est-ce que les noeuds dans le rs et le biparti sont numerotes pareil ... ?

	let mut res = 0.0;

	for v in bip.neighbours_bot(f) {
		if u != *v {
			let mut alpha_uv: f64 = 0.0;
			let n_u = bip.neighbours_top(u);
			let n_v = bip.neighbours_top(*v);

			// println!("N({}) vaut {:?}", u, n_u);
			// println!("N({}) vaut {:?}", v, n_v);


			// alpha_uv = rs.get_distance(u, v);
			// alpha_uv
			if rs.neighbours(u).contains(v) {
				// println!("{} est dans le voisinage de {}.", *v, u);
				alpha_uv = 1.0;
			}
			else if !rs.neighbours(u).contains(v) {// distance 2
				for x in rs.neighbours(u) {
					if rs.neighbours(*x).contains(v) {
						alpha_uv = 0.5;
						break;
					}
				}
			}
			// if alpha_uv == 0.0 {
				// continue;
			// }
			// println!("Alpha_{}{} vaut {}", u, *v, alpha_uv);

			// beta_uv
			// let inter = n_u.intersection(&n_v).count();
			let inter = n_u.intersection(&n_v);
			// let union = n_u.union(&n_v).count();

			// Jaccard
			// let beta_uv: f64 = inter as f64/ union as f64;

			// Adamic-Adar
			// let mut beta_uv = 0.0;
			let degs = bip.degrees_bot();

			let beta_uv = inter.fold(0.0, |beta_uv, w| beta_uv as f64 + 1.0/(degs[w] as f64).ln());

			// for w in inter {
				// beta_uv += 1.0/(degs[w] as f64).ln();
			// }


			let s = bip.scores()[&vec!(*v,f)];

			// Sum
			res += alpha_uv * beta_uv * s;

			// arg max
			// if (alpha_uv * beta_uv * s) > res {
				// res = alpha_uv * beta_uv * s;
			// }

		}
	}

	res
}

fn main() {
	let args: Vec<_> = env::args().collect();

	let mut rs = Graph::new();
	let mut bip = BipartiteGraph::new();

	// Tests graphs.
	rs.read_from_file(&String::from("./data/testgraph.txt"));
	// rs.read_from_file(&String::from("./data/links.txt"));
	bip.read_from_file(&String::from("./data/testbipgraph.txt"));
	// bip.read_from_file(&String::from("./data/ratings.txt"));

	if &args[1] == "scores" {
		
		for i in bip.nodes_top() {
			for j in bip.nodes_bot() {		
				let link = vec!(*i,*j);

				if !bip.scores().contains_key(&link) {
					let s = score(&rs, &bip, 650108, *j);
					if s > 0.0 {
						println!("{} {} {}", 650108, j, s);
					}
				}
				// else {
					// println!("score(user {}, movie {}) -> {}", i, j, bip.scores()[&link]);
				// }
			}
		}
	}
	else if &args[1] == "profiles" {
		// Choose users at random
		let mut rng = thread_rng();
		let users_sample = sample(&mut rng, bip.nodes_top(), 500);


		let movie_degrees = bip.degrees_bot();
		
		for u in users_sample {
	    	let f = File::create(String::from("./users-profiles/") + &u.to_string()).expect("Unable to create file");
	    	let mut f = BufWriter::new(f);
			let mut data = String::new();
			
			for m in bip.neighbours_top(*u) {
				// Write to file
				data = data + &movie_degrees[m].to_string() + "\n";
				// data.push_str("\n");

				// println!("{:#?}", movie_degrees[m]);
				// f.close(); 
			}
	    	f.write_all(data.as_bytes()).expect("Unable to write data");
		}

		// Choose movies at random
		// let mut rng = thread_rng();
		let movie_sample = sample(&mut rng, bip.nodes_bot(), 500);

		let users_degrees = bip.degrees_top();
		
		for m in movie_sample {
	    	let f = File::create(String::from("./movies-profiles/") + &m.to_string()).expect("Unable to create file");
	    	let mut f = BufWriter::new(f);
			let mut data = String::new();
			
			for u in bip.neighbours_bot(*m) {
				// Write to file
				data = data + &users_degrees[u].to_string() + "\n";
				// data.push_str("\n");

				// println!("{:#?}", movie_degrees[m]);
				// f.close(); 
			}
	    	f.write_all(data.as_bytes()).expect("Unable to write data");
		}
	} else if &args[1] == "cover" {
		// cover
		for cover in 0..11 {
			let m = bip.cover_bot(cover as f64 / 10.0);
			println!("{} ({}%) movies to cover {}% users.", m, m as f64 / bip.nodes_bot().len() as f64, (cover as f64 / 10.0) * 100.0);
		}

		for cover in 0..11 {
			let m = bip.cover_top(cover as f64 / 10.0);
			println!("{} ({}%) users to cover {}% movies", m, m as f64 / bip.nodes_top().len() as f64, (cover as f64 / 10.0) * 100.0);
		}
	}
	else {
		// println!("{:?}", rs.nodes());
		// println!("{:?}", rs.neighbours(2));

		// println!("{:?}", bip.nodes_top());
		// println!("{:?}", bip.nodes_bot());
		// println!("{:?}", bip.neighbours_bot(2));

		let proj = bip.projection_top();
		for (k,v) in &proj {
			for n in v {
				println!("{} {} {}", *k, n.0, (n.1 as f64) / (n.2 as f64));
			}
		}
	}

}