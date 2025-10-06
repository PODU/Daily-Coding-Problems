# Day 376: Closest coin by Manhattan distance. Linear scan.
# Time: O(n), Space: O(1).

def closest_coin(me, coins):
    return min(coins, key=lambda c: abs(c[0]-me[0]) + abs(c[1]-me[1]))

if __name__ == "__main__":
    me = (0, 2)
    coins = [(0, 4), (1, 0), (2, 0), (3, 2)]
    b = closest_coin(me, coins)
    print(f"({b[0]}, {b[1]})")
