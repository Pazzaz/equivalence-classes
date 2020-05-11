# Counts the number of equivalence classes
def count_classes(n):
    assert(n >= 3)

    components = []
    for i in range(2**n):
        components.append(0)

    c = 0
    for node in range(2**n):
        if components[node] == 0:
            c += 1
            traverse(node, c, components, n)
    return c


# Non-recursive function to traverse the whole component that 'starting_node'
# is connected to and marking in 'components' which nodes were visited.
def traverse(starting_node, component_name, components, length):
    stack = []
    components[starting_node] = component_name
    neigh = neighbours(starting_node, length, components, component_name)
    stack.append([neigh, 0])
    i = 0
    while(True):
        top = stack[i]
        if top[1] == len(top[0]):
            stack.pop()
            if i == 0:
                break;
            i -= 1
            continue

        new_current = top[0][top[1]]
        new_neigh = neighbours(new_current, length, components, component_name)
        stack.append([new_neigh, 0])
        top[1] += 1
        i += 1


def neighbours(k, length, components, component_name):
    out = []
    for i in range(length):
        ic = invert_circle(k, i, length)

        # Check if the node has been visited before
        if components[ic] == 0:
            components[ic] = component_name
            out.append(ic)
    return out


# Flip 3 consecutive bits at 'index' in 'k'. 'length' specifies how long the
# bit-string is.
def invert_circle(k, index, length):
    # First we create a mask specifying where to check
    if index + 2 == length:
        mask = 0b1 | (0b11 << index)
    elif index + 1 == length:
        mask = 0b11 | (0b1 << index)
    else:
        mask = 0b111 << index

    # Then we check for all ones or all zeros
    if (k & mask) == mask:
        k = k & (~mask)
    elif ((~k) & mask) == mask:
        k += mask
    return k


if __name__ == "__main__":
    for i in range(3, 40):
        print(i, count_classes(i))

