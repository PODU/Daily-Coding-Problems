// Day 757: Optimal blackjack with a fully known deck.
// Player draws the next cards off the top; try every stand point, dealer then
// follows the fixed rule. Pick the player's best net score. Time: O(n^2), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

// deck values: 2..10, face=10, ace=1. deck[0],deck[1]=player; deck[2],deck[3]=dealer;
// deck[4..]=draw pile. Returns best net score (+1 win, -1 loss, 0 push).
int bestScore(const vector<int>& deck) {
    int n = deck.size();
    int player = deck[0] + deck[1];
    int dealerStart = deck[2] + deck[3];
    int best = INT_MIN;
    int ptot = player, idx = 4;
    while (true) {
        if (ptot > 21) break;                  // would be bust; cannot stand here
        // dealer plays from current idx onward
        int dtot = dealerStart, di = idx;
        while (dtot <= 16 && di < n) dtot += deck[di++];
        int result;
        if (dtot > 21) result = 1;             // dealer busts
        else if (ptot > dtot) result = 1;
        else if (ptot < dtot) result = -1;
        else result = 0;
        best = max(best, result);
        if (idx >= n) break;                   // no card left to hit
        ptot += deck[idx++];                    // player hits next card
    }
    return best;
}

int main() {
    // Player 19 vs dealer 5; draw pile [10]. Best: stand on 19, dealer reaches 15.
    vector<int> deck = {10, 9, 2, 3, 10};
    cout << bestScore(deck) << "\n";  // 1
    return 0;
}
