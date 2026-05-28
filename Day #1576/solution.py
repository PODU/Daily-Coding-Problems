# Day 1576: Blackjack solver with perfect deck knowledge.
# Player enumerates how many cards to hit (stops if bust); dealer plays forced rules.
# Time: O(N^2) total; Space: O(1).


def dealer_play(deck, idx, dealer_total):
    while dealer_total <= 16 and idx < len(deck):
        dealer_total += deck[idx]
        idx += 1
    return dealer_total


def compare_score(player, dealer):
    if player > 21:
        return -1
    if dealer > 21:
        return 1
    if player > dealer:
        return 1
    if player < dealer:
        return -1
    return 0


def best_score(deck):
    player_total = deck[0] + deck[1]
    best = None
    k = 0
    while True:
        if player_total <= 21:
            dealer = dealer_play(deck, 4 + k, deck[2] + deck[3])
            s = compare_score(player_total, dealer)
            best = s if best is None else max(best, s)
        else:
            break
        if 4 + k >= len(deck):
            break
        player_total += deck[4 + k]
        k += 1
    return best


if __name__ == "__main__":
    deck = [10, 6, 9, 7, 5, 10, 2]
    print("Optimal player score:", best_score(deck))
