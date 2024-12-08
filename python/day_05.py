
ORDERING_RULES= [
    "47|53",
    "97|13",
    "97|61",
    "97|47",
    "75|29",
    "61|13",
    "75|53",
    "29|13",
    "97|29",
    "53|29",
    "61|53",
    "97|53",
    "61|29",
    "47|13",
    "75|47",
    "97|75",
    "47|61",
    "75|61",
    "47|29",
    "75|13",
    "53|13",
        ]

PAGE_NUMBERS = [
    [75,47,61,53,29],
    [97,61,53,29,13],
    [75,29,13],
    [75,97,47,61,53],
    [61,13,29],
    [97,13,75,29,47],
        ]


def _parse_ordering_rules(ordering_rules):
    rules = {}
    for r in ordering_rules:
        print(r)
        l, r = r.split('|')
        l = int(l)
        r = int(r)
        if l in rules:
            rules[l].append(r)
        else:
            rules[l] = [r, ]
    return rules

def _is_correct_ordering(page_numbers, ordering_rules):
    for i, n in enumerate(page_numbers):
        if i == 0:
            continue
        before = page_numbers[:i]
        for b in before:
            if n in ordering_rules and b in ordering_rules[n]:
                return False
    return True
        



def day_01_part_1(ordering_rules, page_numbers):
    rules = _parse_ordering_rules(ordering_rules)
    correct_ordering = list(filter(lambda pn: _is_correct_ordering(pn, rules), page_numbers))
    print(correct_ordering)
    middle_numbers = [pn[len(pn) // 2] for pn in correct_ordering]
    print(middle_numbers)
    return sum(middle_numbers)


if __name__ == '__main__':

    # Test
    print('--- Test Data')
    # d01_1 = day_01_part_1(ORDERING_RULES, PAGE_NUMBERS)
    # print(d01_1)
    print('--- Prod Data')
    with open('../data/day_05.txt') as f:
        data_str = f.read()

    order_rules, page_numbers = data_str.split('\n\n')
    order_rules = order_rules.strip().split('\n')

    page_numbers = page_numbers.strip().split('\n')
    page_numbers = [[int(n) for n in pn.split(',')] for pn in page_numbers]
    d01_1 = day_01_part_1(order_rules, page_numbers)
    print(d01_1)

