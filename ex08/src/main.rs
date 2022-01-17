fn create_powerset(set: &[i32], powerset: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, it: &mut usize, i: usize) {
    
    if i == set.len() {
        powerset[*it] = subset.clone();
        powerset[*it].sort();
        *it += 1;
    }
    else {
        create_powerset(set, powerset, subset, it, i + 1);
        subset.push(set[i]);
        create_powerset(set, powerset, subset, it, i + 1);
        subset.pop();
    }
}

fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut b_set : Vec<Vec<i32>> = Vec::new();

    let base: usize = 2;
    let powerset_size: usize = base.pow(set.len().try_into().unwrap());
    b_set.resize(powerset_size, Vec::new());

    create_powerset(set, &mut b_set, &mut Vec::new(), &mut 0, 0);

    b_set.sort_by(|a, b| {
        if a.len() != b.len() {
            return a.len().cmp(&b.len());
        }
        return a.cmp(b);
    });

    return b_set;
}

fn print_powerset(powerset: Vec<Vec<i32>>) {
    for set in &powerset {
        print!("[");
        for value in set {
            print!(" {}", value);
        }
        println!(" ]");
    }

}

fn main() {
    print_powerset(powerset(&[0, 1, 2]));
}
