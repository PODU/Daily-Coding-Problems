# Day 1005: Blackjack solver with perfect knowledge of the deck order.
# Cards dealt: player gets deck[0],deck[2]; dealer gets deck[1],deck[3]; the
# remaining deck is the draw pile. With perfect knowledge the player just tries
# every number of hits k and keeps the best outcome (+1 win, 0 push, -1 loss);
# the dealer then plays its fixed rule (hit while total <= 16). O(N^2) over the deck.

def best_score(deck):
    player_base = deck[0] + deck[2]
    dealer_base = deck[1] + deck[3]
    best = -1
    k = 0
    while True:
        player = player_base + sum(deck[4:4 + k])
        if player > 21:           # busting only gets worse -> stop exploring
            break
        idx = 4 + k
        dealer = dealer_base
        while dealer <= 16 and idx < len(deck):  # dealer must hit on 16 or lower
            dealer += deck[idx]
            idx += 1
        if dealer > 21 or player > dealer:
            outcome = 1
        elif player < dealer:
            outcome = -1
        else:
            outcome = 0
        best = max(best, outcome)
        if 4 + k >= len(deck):
            break
        k += 1
    return best


if __name__ == "__main__":
    deck = [10, 10, 6, 9, 5, 7, 3, 8]  # blackjack values (aces = 1)
    print("Best player score:", best_score(deck))  # 1
