use std::f32::INFINITY;
use rand::{thread_rng, Rng};

pub type Point = Vec<f32>;
pub type Cluster = Vec<usize>;

fn initialize(k: usize, data: &Vec<Point>) -> Vec<usize> {
	let mut indices: Vec<usize> = (0..data.len()).collect();
	thread_rng().shuffle(&mut indices[..]);
	indices.iter().take(k).cloned().collect()
}

// manhatten distance
fn distance(a: &Point, b: &Point) -> f32 {
	a.iter().zip(b.iter()).map(|(&x,&y)| (x - y) * (x - y)).fold(0.0, |acc, x| acc + x).sqrt()
}

fn total_distance(data: &Vec<Point>, medoid: &Point) -> f32 {
	data.iter().map(|x| distance(medoid, x)).fold(0.0, |acc, x| acc + x)
}

fn assign_datapoints(k: usize, data: &Vec<Point>, medoids: &Vec<usize>) -> Vec<Cluster> {
	let indices: Vec<usize> = data.iter().map(|x| {
		let mut min_dist = INFINITY;
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

	(0..k).map(|idx| indices.iter().enumerate().filter(|&(_,&i)| i == idx).map(|(x,_)| x).collect()).collect()
}

fn compute_medoids(data: &Vec<Point>, medoids: &Vec<usize>, clusters: &Vec<Cluster>) -> Vec<usize> {
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

pub fn kmedoids(k: usize, data: &Vec<Point>) -> (Vec<Cluster>, Vec<usize>) {
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