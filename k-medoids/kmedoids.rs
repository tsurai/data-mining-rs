use std::iter::AdditiveIterator;
use std::rand::{task_rng, Rng};
use std::io::{BufferedReader, File};

type Point = Vec<f32>;
type Cluster = Vec<uint>;

fn initialize(k: uint, data: &Vec<Point>) -> Vec<uint> {
	let mut indices: Vec<uint> = range(0, data.len()).collect();
	task_rng().shuffle(indices.as_mut_slice());
	indices.iter().take(k).map(|&x| x).collect()
}

// manhatten distance
fn distance(a: &Point, b: &Point) -> f32 {
	a.iter().zip(b.iter()).map(|(&x,&y)| (x - y) * (x - y)).sum().sqrt()
}

fn total_distance(data: &Vec<Point>, medoid: &Point) -> f32 {
		data.iter().map(|x| distance(medoid, x)).sum()
}

fn assign_datapoints(k: uint, data: &Vec<Point>, medoids: &Vec<uint>) -> Vec<Cluster> {
	let indices: Vec<uint> = data.iter().map(|x| {
		let mut min_dist = std::f32::INFINITY;
		let mut min_index = 0;

		for (i,m) in medoids.iter().enumerate() {
			let cur_dist = distance(x, &data[*m]);
			if cur_dist <= min_dist {
				min_dist = cur_dist;
				min_index = i;
			}
		}
		min_index
	}).collect();
	
	Vec::from_fn(k, |idx| indices.iter().enumerate().filter(|&(_,&i)| i == idx).map(|(x,_)| x).collect())
}

fn compute_medoids(data: &Vec<Point>, medoids: &Vec<uint>, clusters: &Vec<Cluster>) -> Vec<uint> {
	// for all medoids
	medoids.iter().map(|&m| {
		let cluster = clusters.iter().filter(|c| c.iter().any(|&x| x == m)).last().unwrap();
		let cluster_data = data.iter().enumerate().filter(|&(i,_)| cluster.iter().any(|c| *c == i)).map(|(_,x)| x.clone()).collect();
		let mut min_dist = total_distance(&cluster_data, &data[m]);
		let mut min_index = m;

		for new_m in cluster.iter() {
			let cur_dist = total_distance(&cluster_data, &data[*new_m]);
			if cur_dist < min_dist {
				min_dist = cur_dist;
				min_index = *new_m;
			}
		}
		min_index
	}).collect()
}

fn kmedoids(k: uint, data: &Vec<Point>) -> (Vec<Cluster>, Vec<uint>) {
	let mut medoids = initialize(k, data);

	loop {
		let clusters = assign_datapoints(k, data, &medoids);
		let new_medoids = compute_medoids(data, &medoids, &clusters);

		if medoids == new_medoids {
			return (clusters, medoids);
		}
		medoids = new_medoids;
	}
}

fn load_data(filename: String) -> (Vec<Point>, Vec<f32>) {
	let mut file = BufferedReader::new(File::open(&Path::new(filename)));
	let lines : Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	let d : Vec<Point> = lines.iter().map(|x| x.as_slice().split(',').filter_map(from_str).collect()).collect();

	(d.iter().map(|x| x.init().to_vec()).collect(), d.iter().map(|x| *x.last().unwrap()).collect())
}

fn main() {
	let (data, _) = load_data(String::from_str("../data/iris.data"));
	let (cluster, medoids) = kmedoids(3, &data);

	// save plot data for gnuplot
	let mut file = File::create(&Path::new("output.data"));
	for (i,c) in cluster.iter().enumerate() {
		for n in c.iter() {
			file.write_line(format!("{} {} {} {}", data[*n][0], data[*n][1], data[*n][2], i+1).as_slice());
		}
	}

	for c in medoids.iter() {
		file.write_line(format!("{} {} {} 4", data[*c][0], data[*c][1], data[*c][2]).as_slice()); 
	}
}