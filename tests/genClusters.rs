use openbabel::fingerprint::Kind;
use similarity_metrics::{cluster, load};

#[test]
fn test_cluster() {
    // println!("Generating fingerprints...");
    // let fps = load::gen_fps(Kind::FP2 { nbits: 2048 }, "tests/large.mol");
    // println!("Saving fingerprints...");
    // let json = serde_json::to_string(&fps).unwrap();
    // std::fs::write("tests/fps.json", json).unwrap();

    println!("Loading fingerprints...");
    let fps = serde_json::from_str::<Vec<Vec<u8>>>(&std::fs::read_to_string("tests/fps.json").unwrap()).unwrap();

    println!("Clustering...");
    let clusters = cluster::cluster(fps, 10000, f64::MAX, 5);

    // Output to clusters.json
    println!("Saving clusters...");
    let json = serde_json::to_string(&clusters).unwrap();
    std::fs::write("tests/clusters.json", json).unwrap();
}
