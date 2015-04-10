pub type Point = Vec<f32>;
pub type Cluster = Vec<usize>;

fn distance(a: &Point, b: &Point) -> f32 {
	a.iter().zip(b.iter()).map(|(&x,&y)| (x - y) * (x - y)).fold(0.0, |acc, x| acc + x).sqrt()
}

fn range_query(p: &Point, e: f32, data: &Vec<Point>) -> Vec<usize> {
	data.iter().enumerate().filter(|&(_,x)| distance(p, x) < e).map(|(i,_)| i).collect()
}

fn expand<'a>(p: &Point, neighbours: &Vec<usize>, e: f32, min_pts: usize, data: &Vec<Point>, visited: &mut Vec<usize>, cluster: &'a mut Cluster) -> &'a Cluster {
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

pub fn dbscan(e: f32, min_pts: usize, data: &Vec<Point>) -> Vec<Cluster> {
	let mut clusters: Vec<Cluster> = Vec::new();
	let mut noise: Vec<usize> = Vec::new();
	let mut visited: Vec<usize> = Vec::new();

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