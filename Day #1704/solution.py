# Day 1704: Lazy bartender = min set cover (NP-hard). Greedy: repeatedly pick drink covering
# most uncovered customers. Time O(D^2*C), Space O(D*C).
def fewest_drinks(preferences):
    drink_to_cust = {}
    uncovered = set(preferences.keys())
    for cust, drinks in preferences.items():
        for d in drinks:
            drink_to_cust.setdefault(d, set()).add(cust)
    learned = 0
    while uncovered:
        best_drink, best_count = None, 0
        for drink, custs in drink_to_cust.items():
            cnt = len(custs & uncovered)
            if cnt > best_count:
                best_count, best_drink = cnt, drink
        if best_drink is None:
            break
        uncovered -= drink_to_cust[best_drink]
        learned += 1
    return learned

def main():
    preferences = {0:[0,1,3,6], 1:[1,4,7], 2:[2,4,7,5], 3:[3,2,5], 4:[5,8]}
    print(fewest_drinks(preferences))

if __name__ == "__main__":
    main()
