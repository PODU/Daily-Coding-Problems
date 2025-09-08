// Blackjack with known deck: deal alternately (player,dealer,player,dealer), then the player
// may take k>=0 hits. Try every k (until bust), simulate the forced dealer, pick the best net
// score (+1 win / 0 push / -1 loss). Time: O(deck), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int bestScore(const vector<int>& deck) {
    int player2 = deck[0] + deck[2];
    int dealer2 = deck[1] + deck[3];
    int best = INT_MIN;
    int psum = player2, idx = 4, k = 0;
    while (true) {
        // outcome for taking exactly k hits (already applied to psum/idx)
        int outcome;
        if (psum > 21) {
            outcome = -1; // bust
        } else {
            int dsum = dealer2, di = idx;
            while (dsum <= 16 && di < (int)deck.size()) dsum += deck[di++];
            if (dsum > 21 || psum > dsum) outcome = 1;
            else if (psum < dsum) outcome = -1;
            else outcome = 0;
        }
        best = max(best, outcome);
        if (psum > 21 || idx >= (int)deck.size()) break; // no point hitting after bust
        psum += deck[idx++];
        k++;
    }
    return best;
}

int main() {
    // player: 5,6 (=11); dealer: 10,9 (=19). Hitting once draws 10 -> 21 beats 19.
    vector<int> deck = {5, 10, 6, 9, 10, 2, 3, 7, 8, 4};
    cout << "Best score: " << bestScore(deck) << "\n"; // 1
}
