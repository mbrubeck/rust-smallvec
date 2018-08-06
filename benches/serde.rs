#![feature(test)]

#[macro_use]
extern crate smallvec;
extern crate test;
extern crate bincode;
extern crate serde;


use bincode::{serialize, deserialize};
use test::Bencher;
use smallvec::SmallVec;

#[bench]
fn smallvec_i32_benchmark(b: &mut Bencher) {
    let data = {
        let tinyvec : SmallVec<[i32; 5]> = smallvec![1,2,3,4,5];
        let sv = tinyvec;
        serialize(&sv).unwrap()
    };

    b.iter(|| {
        let tinyvec_2 : SmallVec<[i32; 5]> = deserialize(&data[..]).unwrap();
        tinyvec_2
    });
}

#[bench]
fn vec_i32_benchmark(b: &mut Bencher) {
    let data = {
        let tinyvec : Vec<i32> = vec![1,2,3,4,5];
        let sv = tinyvec;
        serialize(&sv).unwrap()
    };

    b.iter(|| {
        let tinyvec_2 : Vec<i32> = deserialize(&data[..]).unwrap();
        tinyvec_2
    });
}

#[bench]
fn smallvec_tuple_benchmark(b: &mut Bencher) {
    let data = {
        let k1 = "hey";
        let v1 = "now";
        let k2 = "you're";
        let v2 = "an";
        let k3 = "all";
        let v3 = "star";
        let k4 = "get";
        let v4 = "your";
        let k5 = "game";
        let v5 = "on";
        let tinyvec : SmallVec<[(&str, &str); 5]> = smallvec![(k1, v1), (k2, v2), (k3, v3), (k4, v4), (k5, v5)];
        let sv = tinyvec;
        serialize(&sv).unwrap()
    };

    b.iter(|| {
        let tinyvec_2 : SmallVec<[(&str,&str); 5]> = deserialize(&data[..]).unwrap();
        tinyvec_2
    });
}

#[bench]
fn vec_tuple_benchmark(b: &mut Bencher) {
    let data = {
        let k1 = "hey";
        let v1 = "now";
        let k2 = "you're";
        let v2 = "an";
        let k3 = "all";
        let v3 = "star";
        let k4 = "get";
        let v4 = "your";
        let k5 = "game";
        let v5 = "on";
        let regvec : Vec<(&str, &str)> = vec![(k1, v1), (k2, v2), (k3, v3), (k4, v4), (k5, v5)];
        let v = regvec;
        serialize(&v).unwrap()
    };

    b.iter(|| {
        let tinyvec_2 : Vec<(&str,&str)> = deserialize(&data[..]).unwrap();
        tinyvec_2
    });
}
