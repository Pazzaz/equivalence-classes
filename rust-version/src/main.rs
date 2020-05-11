fn main() {
    for i in 3..64 {
        println!("{:3} : {}", i, count_classes(i));
    }
}

// Counts the number of equivalence classes
fn count_classes(n: usize) -> usize {
    assert!(n >= 3);

    let nodes = 2_usize.pow(n as u32) as usize;

    let mut components = vec![0; nodes];
    let mut c = 0;
    for node in 0..nodes {
        if components[node] == 0 {
            c += 1;
            traverse(node, c, &mut components, n);
        }
    }
    c
}

// Non-recursive function to traverse the whole component that 'starting_node'
// is connected to and marking in 'components' which nodes were visited.
fn traverse(starting_node: usize, component_name: usize, components: &mut Vec<usize>, length: usize) {
    let mut stack: Vec<(Vec<usize>, usize)> = Vec::new();
    components[starting_node] = component_name;
    let neigh = neighbours(starting_node, length, components, component_name);
    stack.push((neigh, 0));
    let mut i = 0;
    loop {
        let top = &mut stack[i];
        if top.1 == top.0.len() {
            stack.pop();
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }

        let new_current = top.0[top.1];
        let new_neigh = neighbours(new_current, length, components, component_name);
        top.1 += 1;
        stack.push((new_neigh, 0));
        i += 1;

    }
}

fn neighbours(k: usize, length: usize, components: &mut Vec<usize>, component_name: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for i in 0..length {
        let ic = invert_circle(k, i, length);

        // Check if the node has been visited before
        if components[ic] == 0 {
            components[ic] = component_name;
            out.push(ic);
        }
    }
    out
}

// Flip 3 consecutive bits at 'index' in 'k'. 'length' specifies how long the
// bit-string is.
fn invert_circle(mut k: usize, index: usize, length: usize) -> usize {
    // First we create a mask specifying where to check
    let mask = if index + 2 == length {
        0b1 | (0b11 << index)
    } else if index + 1 == length {
        0b11 | (0b1 << index)
    } else {
        0b111 << index
    };

    // Then we check for all ones or all zeros
    if (k & mask) == mask {
        k = k & (!mask);
    } else if ((!k) & mask) == mask {
        k += mask;
    }
    k
}
