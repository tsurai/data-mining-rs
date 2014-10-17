use std::iter::AdditiveIterator;
use std::rand::{task_rng, Rng};
use std::io::{BufferedReader, File};

type Point = Vec<f32>;
type Cluster = Vec<uint>;

fn initialize(k: uint, data: &Vec<Point>) -> Vec<Point> {
		let mut indices: Vec<uint> = range(0, data.len()).collect();
		task_rng().shuffle(indices.as_mut_slice());
		indices.iter().take(k).map(|&x| data[x].clone()).collect()
}

fn distance(a: &Point, b: &Point) -> f32 {
		a.iter().zip(b.iter()).map(|(&x,&y)| (x - y) * (x - y)).sum().sqrt()
}

fn assign_datapoints(k: uint, data: &Vec<Point>, centroids: &Vec<Point>) -> Vec<Cluster> {
		let indices: Vec<uint> = data.iter().map(|x| {
				let mut min_dist = std::f32::INFINITY;
				let mut min_index = 0;
				
				for (i,c) in centroids.iter().enumerate() {
						let cur_dist = distance(x, c);
						if cur_dist <= min_dist {
								min_dist = cur_dist;
								min_index = i;
						}
				}
				min_index
		}).collect();

		Vec::from_fn(k, |idx| indices.iter().enumerate().filter(|&(_,&i)| i == idx).map(|(x,_)| x).collect()) 
}

fn compute_centroids(data: &Vec<Point>, clusters: &Vec<Cluster>) -> Vec<Point> {
		clusters.iter().map(|c| range(0, data[0].len()).map(|n| c.iter().map(|&i| data[i][n]).sum() / c.len() as f32).collect()).collect()
}

fn kmeans(k: uint, data: &Vec<Point>) -> Vec<Cluster> {
	let mut centroids = initialize(k, data);
				
	loop {
		let clusters = assign_datapoints(k, data, &centroids);
		let new_centroids = compute_centroids(data, &clusters);
		
		if centroids == new_centroids {
		    return clusters;
		}
		centroids = new_centroids;
	}
}

fn load_data(filename: String) -> (Vec<Point>, Vec<f32>) {
	let mut file = BufferedReader::new(File::open(&Path::new(filename)));
	let lines : Vec<String> = file.lines().map(|x| x.unwrap()).collect();
	let d : Vec<Point> = lines.iter().map(|x| x.as_slice().split(',').filter_map(from_str).collect()).collect();

	(d.iter().map(|x| x.init().to_vec()).collect(), d.iter().map(|x| *x.last().unwrap()).collect())
}

fn main() {
	let (data, _) = load_data(String::from_str("./iris.data"));
	println!("{}", kmeans(3, &data));
}