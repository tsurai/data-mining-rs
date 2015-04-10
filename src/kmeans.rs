use std::f32::INFINITY;
use rand::{thread_rng, Rng};

pub type Point = Vec<f32>;
pub type Cluster = Vec<usize>;

fn initialize(k: usize, data: &Vec<Point>) -> Vec<Point> {
	let mut indices: Vec<usize> = (0..data.len()).collect();
	thread_rng().shuffle(&mut indices[..]);
	indices.iter().take(k).map(|&x| data[x].clone()).collect()
}

fn distance(a: &Point, b: &Point) -> f32 {
	a.iter().zip(b.iter()).map(|(&x,&y)| (x - y) * (x - y)).fold(0.0, |acc, x| acc + x).sqrt()
}

fn assign_datapoints(k: usize, data: &Vec<Point>, centroids: &Vec<Point>) -> Vec<Cluster> {
	let indices: Vec<usize> = data.iter().map(|x| {
		let mut min_dist = INFINITY;
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

	(0..k).map(|idx| indices.iter().enumerate().filter(|&(_,&i)| i == idx).map(|(x,_)| x).collect()).collect()
}

fn compute_centroids(data: &Vec<Point>, clusters: &Vec<Cluster>) -> Vec<Point> {
	clusters.iter().map(|c| (0..data[0].len()).map(|n| c.iter().map(|&i| data[i][n]).fold(0.0, |acc,x| acc + x) / c.len() as f32).collect()).collect()
}

pub fn kmeans(k: usize, data: &Vec<Point>) -> (Vec<Cluster>, Vec<Point>) {
	let mut centroids = initialize(k, data);

	loop {
		let clusters = assign_datapoints(k, data, &centroids);
		let new_centroids = compute_centroids(data, &clusters);

		if centroids == new_centroids {
			return (clusters, centroids);
		}
		centroids = new_centroids;
	}
}