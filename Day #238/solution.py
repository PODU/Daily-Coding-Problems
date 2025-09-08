# Day 238: Blackjack with known deck: deal alternately (player,dealer,player,dealer), then the player
# may take k>=0 hits. Try every k (until bust), simulate the forced dealer, pick the best net
# score (+1 win / 0 push / -1 loss). Time: O(deck), Space: O(1).


def best_score(deck):
    player2 = deck[0] + deck[2]
    dealer2 = deck[1] + deck[3]
    best = float("-inf")
    psum, idx = player2, 4
    while True:
        if psum > 21:
            outcome = -1
        else:
            dsum, di = dealer2, idx
            while dsum <= 16 and di < len(deck):
                dsum += deck[di]
                di += 1
            if dsum > 21 or psum > dsum:
                outcome = 1
            elif psum < dsum:
                outcome = -1
            else:
                outcome = 0
        best = max(best, outcome)
        if psum > 21 or idx >= len(deck):
            break
        psum += deck[idx]
        idx += 1
    return best


if __name__ == "__main__":
    deck = [5, 10, 6, 9, 10, 2, 3, 7, 8, 4]
    print("Best score:", best_score(deck))  # 1
