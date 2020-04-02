mod tree;

fn main() {
    let mut t = tree::Tree::new(&502);
    t.add(&10);
    t.add(&123);
    t.add(&352);
    t.add(&436);
    t.add(&4322);
    t.add(&32424);
    t.add(&02934);
    t.add(&23410);
    t.add(&9340);
    t.add(&123);
    t.add(&132);
    t.add(&345);
    t.add(&765);
    t.add(&87);
    t.add(&7686);
    t.add(&876);
    t.add(&4646);
    t.add(&45353);
    t.add(&2424);

    println!("Does 352 exist? {:?}", t.find(&352));
    println!("Does 2424 exist? {:?}", t.find(&2424));
    println!("Does 24233 exist? {:?}", t.find(&24233));

    println!("{:?}", t);
}
