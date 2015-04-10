use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::process::Command;

use datamining::{Point, kmeans, kmedoids, dbscan};

extern crate datamining_rs as datamining;

fn load_data<S: AsRef<Path>>(filename: S) -> Vec<Point> {
  let file = BufReader::new(File::open(filename).unwrap());
  let lines : Vec<String> = file.lines().map(|x| x.unwrap()).collect();

  lines.iter().map(|x| x[..].split(',').filter_map(|x| f32::from_str(x).ok()).collect()).collect::<Vec<Point>>()
}

#[test]
fn test_kmeans() {
  let data = load_data("tests/data/iris.data");
  let (cluster, centroids) = kmeans(3, &data);

  // save plot data for gnuplot
  let mut file = File::create("tests/output/kmeans.data").unwrap();
  for (i,c) in cluster.iter().enumerate() {
    for &n in c.iter() {
      file.write_fmt(format_args!("{} {} {} {}\n", data[n][0], data[n][1], data[n][2], i+1)).ok();
    }
  }

  for c in centroids.iter() {
    file.write_fmt(format_args!("{} {} {} 4\n", c[0], c[1], c[2])).ok();
  }

  Command::new("gnuplot").current_dir("tests/gnuplot").arg("kmeans.gp").output().unwrap_or_else(|e| {
    panic!("Failed to plot data: {}", e)
  });
}

#[test]
fn test_kmedoids() {
  let data = load_data("tests/data/iris.data");
  let (cluster, medoids) = kmedoids(3, &data);

  // save plot data for gnuplot
  let mut file = File::create("tests/output/kmedoids.data").unwrap();
  for (i,c) in cluster.iter().enumerate() {
    for &n in c.iter() {
      file.write_fmt(format_args!("{} {} {} {}\n", data[n][0], data[n][1], data[n][2], i+1)).ok();
    }
  }

  for &c in medoids.iter() {
    file.write_fmt(format_args!("{} {} {} 4\n", data[c][0], data[c][1], data[c][2])).ok();
  }

  Command::new("gnuplot").current_dir("tests/gnuplot").arg("kmedoids.gp").output().unwrap_or_else(|e| {
    panic!("Failed to plot data: {}", e)
  });
}

#[test]
fn test_dbscan() {
  let data = load_data("tests/data/cassini.data");
  let clusters = dbscan(0.18, 5, &data);

  // save plot data for gnuplot
  let mut file = File::create("tests/output/dbscan.data").unwrap();
  for (i,c) in clusters.iter().enumerate() {
    for &n in c.iter() {
      file.write_fmt(format_args!("{} {} {}\n", data[n][0], data[n][1], i+1)).ok();
    }
  }

  Command::new("gnuplot").current_dir("tests/gnuplot").arg("dbscan.gp").output().unwrap_or_else(|e| {
    panic!("Failed to plot data: {}", e)
  });
}