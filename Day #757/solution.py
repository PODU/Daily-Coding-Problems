# Day 757: Optimal blackjack with a fully known deck.
# Player draws the next cards off the top; try every stand point, dealer then
# follows the fixed rule. Pick the player's best net score. Time: O(n^2), Space: O(1).


def best_score(deck):
    # deck values: 2..10, face=10, ace=1.
    # deck[0],deck[1]=player; deck[2],deck[3]=dealer; deck[4:]=draw pile.
    n = len(deck)
    player = deck[0] + deck[1]
    dealer_start = deck[2] + deck[3]
    best = None
    ptot, idx = player, 4
    while True:
        if ptot > 21:
            break
        dtot, di = dealer_start, idx
        while dtot <= 16 and di < n:
            dtot += deck[di]
            di += 1
        if dtot > 21:
            result = 1
        elif ptot > dtot:
            result = 1
        elif ptot < dtot:
            result = -1
        else:
            result = 0
        best = result if best is None else max(best, result)
        if idx >= n:
            break
        ptot += deck[idx]
        idx += 1
    return best


if __name__ == "__main__":
    deck = [10, 9, 2, 3, 10]
    print(best_score(deck))  # 1
