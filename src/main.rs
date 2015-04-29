extern crate shiplift;
extern crate jed;
extern crate rustc_serialize;

use rustc_serialize::json;
use shiplift::Docker;
use shiplift::rep::Stats;

fn main() {
  let mut docker = Docker::new();
  for i in docker.images().list().unwrap() {
    println!("{:?}", i.RepoTags);
  }
  //let data = docker.containers().get("160bbff9ff12e10f73c16a4f20d5ac785bf43066017e28cb24d53cc1c128ee36").stats().unwrap();
  //for e in jed::Iter::new(data) {
  //  let s = json::encode(&e).unwrap();
  //  println!("\n -> {:?}", json::decode::<Stats>(&s).unwrap());
  //};

  //println!("-> {}", docker.containers().get("160bbff9ff12e10f73c16a4f20d5ac785bf43066017e28cb24d53cc1c128ee36").inspect().unwrap());
  //let data = docker.info().unwrap();
  //println!("changes {:?}", data);
  //println!("start {:?}", docker.containers().get("4a3cd446f5fbc3e1f0f6ecc00508ddf9b34d294371335744d5d712836058f311").start().unwrap());

  //let read = docker.images().create("redis:3.0.0").unwrap();
  //for e in jed::Iter::new(read) {
  //  println!("\n -> {:?}", e);
  //}  
}
