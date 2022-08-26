use BinarySearchTree::BinarySearchTree;
use rand::Rng;
use std::time::Instant;

fn main() {
    let mut tree: BinarySearchTree<u32> = BinarySearchTree::new();
    let mut vec: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..10000000 {
        let number: u32 = rng.gen();
        tree.root.insert(number);
    }
    tree.root.insert(182381);

    for _i in 0..10000000 {
        let number: u32 = rng.gen();
        vec.push(number);
    }
    vec.push(182381);

    let tree_time_start = Instant::now();
    let tree_search = tree.root.search(&182381);
    let tree_time = tree_time_start.elapsed();
    println!("Tree search took {} micro seconds, {:?}", tree_time.as_micros(), tree_search );

    let vec_time_start = Instant::now();
    let vec_search = search_vec(vec, &182381);
    let vec_time = vec_time_start.elapsed();
    println!("Linear Search took {} micro seeconds, {:?}", vec_time.as_micros(), vec_search);

}


fn search_vec(vec: Vec<u32>, target: &u32) -> bool {
    for item in vec {
        if item == *target {
            return true
        }
    }
    false
}