// Day 1576: Blackjack solver with perfect deck knowledge.
// Player enumerates how many cards to hit (stops if bust); dealer plays forced rules.
// Time: O(N) over deck per stand, O(N^2) total; Space: O(1).
#include <bits/stdc++.h>
using namespace std;

// Dealer plays forced: hit while total <= 16. Returns dealer final total (>21 == bust).
int dealerPlay(const vector<int>& deck, int idx, int dealerTotal) {
    while (dealerTotal <= 16 && idx < (int)deck.size()) {
        dealerTotal += deck[idx++];
    }
    return dealerTotal;
}

// Returns +1 win, -1 loss, 0 tie comparing player vs dealer.
int compareScore(int player, int dealer) {
    bool pBust = player > 21, dBust = dealer > 21;
    if (pBust) return -1;           // player bust always loses
    if (dBust) return 1;            // dealer bust, player ok -> win
    if (player > dealer) return 1;
    if (player < dealer) return -1;
    return 0;
}

// Maximize player's score (wins - losses) with perfect knowledge.
int bestScore(const vector<int>& deck) {
    // deck[0],deck[1] -> player ; deck[2],deck[3] -> dealer ; deck[4..] extra
    int playerTotal = deck[0] + deck[1];
    int best = INT_MIN;
    int k = 0;
    while (true) {
        // player stands after k hits; dealer draws from index 4+k
        if (playerTotal <= 21) {
            int dealer = dealerPlay(deck, 4 + k, deck[2] + deck[3]);
            best = max(best, compareScore(playerTotal, dealer));
        } else {
            break; // busted, no point hitting more
        }
        if (4 + k >= (int)deck.size()) break;
        playerTotal += deck[4 + k];
        k++;
    }
    return best;
}

int main() {
    // Demo deck (card values). Player=10+6=16, dealer=9+7=16.
    vector<int> deck = {10, 6, 9, 7, 5, 10, 2};
    cout << "Optimal player score: " << bestScore(deck) << "\n";
    return 0;
}
