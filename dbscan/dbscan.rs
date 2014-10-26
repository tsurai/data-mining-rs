use std::iter::AdditiveIterator;
use std::io::{BufferedReader, File};

type Point = Vec<f32>;
type Cluster = Vec<uint>;

fn distance(a: &Point, b: &Point) -> f32 {
	a.iter().zip(b.iter()).map(|(&x,&y)| (x - y) * (x - y)).sum().sqrt()
}

fn range_query(p: &Point, e: f32, data: &Vec<Point>) -> Vec<uint> {
	data.iter().enumerate().filter(|&(_,x)| distance(p, x) < e).map(|(i,_)| i).collect()
}

fn expand<'a>(p: &Point, neighbours: &Vec<uint>, e: f32, min_pts: uint, data: &Vec<Point>, visited: &mut Vec<uint>, cluster: &'a mut Cluster) -> &'a Cluster {
	for q in neighbours.iter() {
		if !visited.contains(q) {
			visited.push(q.clone());
			let new_neighbours = range_query(p, e, data);
			if new_neighbours.len() >= min_pts {
				expand(&data[*q], &new_neighbours, e, min_pts, data, visited, cluster);
			}
		}

		if !cluster.contains(q) {
			cluster.push(q.clone());
		}
	}
	cluster
}

fn dbscan(e: f32, min_pts: uint, data: &Vec<Point>) -> Vec<Cluster> {
	let mut clusters: Vec<Cluster> = Vec::new();
	let mut noise: Vec<uint> = Vec::new();
	let mut visited: Vec<uint> = Vec::new();
	
	for (i,p) in data.iter().enumerate() {
		if !visited.contains(&i) {
			visited.push(i.clone());
			let neighbours = range_query(p, e, data);
			if neighbours.len() < min_pts {
				noise.push(i);
			} else {
				let mut cluster: Cluster = Vec::new();
				cluster.push(i);
				expand(p, &neighbours, e, min_pts, data, &mut visited, &mut cluster);
				clusters.push(cluster);
			}
		}
	}

	clusters
}

fn load_data(filename: String) -> (Vec<Point>, Vec<f32>) {
	let mut file = BufferedReader::new(File::open(&Path::new(filename)));
	let lines : Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	let d : Vec<Point> = lines.iter().map(|x| x.as_slice().split(',').filter_map(from_str).collect()).collect();

	(d.iter().map(|x| x.init().to_vec()).collect(), d.iter().map(|x| *x.last().unwrap()).collect())
}

fn main() {
	let (data, _) = load_data(String::from_str("../data/cassini.data"));
	let clusters = dbscan(0.18, 5, &data);

	// save plot data for gnuplot
	let mut file = File::create(&Path::new("output.data"));
	for (i,c) in clusters.iter().enumerate() {
		for n in c.iter() {
			file.write_line(format!("{} {} {}", data[*n][0], data[*n][1], i+1).as_slice());
		}
	}
}
