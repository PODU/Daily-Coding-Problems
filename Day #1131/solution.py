# Day 1131: Blackjack with known deck. DFS over deck index: player hits/stands to maximize
# net (+1/0/-1); dealer then plays fixed rule (hit while <=16). O(deck) states.

deck = [10, 7, 5, 9, 6, 4, 10, 2]

def dealer_play(p, player_total):
    dealer_total = deck[1] + deck[3]
    while dealer_total <= 16 and p < len(deck):
        dealer_total += deck[p]
        p += 1
    if dealer_total > 21:
        return 1
    if player_total > dealer_total:
        return 1
    if player_total < dealer_total:
        return -1
    return 0

def player_play(p, player_total):
    best = dealer_play(p, player_total)  # stand
    if p < len(deck):
        nt = player_total + deck[p]
        hit = -1 if nt > 21 else player_play(p + 1, nt)
        best = max(best, hit)
    return best

def main():
    player_total = deck[0] + deck[2]
    print(player_play(4, player_total))

if __name__ == "__main__":
    main()
