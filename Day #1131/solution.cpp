// Blackjack with known deck. DFS over deck index: player hits/stands to maximize
// net (+1/0/-1); dealer then plays fixed rule (hit while <=16). O(deck) states.
#include <bits/stdc++.h>
using namespace std;

vector<int> deck;

// dealer plays deterministically from pointer p with fixed initial dealer total.
int dealerPlay(int p, int playerTotal){
    int dealerTotal = deck[1] + deck[3];
    while(dealerTotal <= 16 && p < (int)deck.size()){
        dealerTotal += deck[p]; p++;
    }
    if(dealerTotal > 21) return 1;          // dealer bust -> player wins
    if(playerTotal > dealerTotal) return 1;
    if(playerTotal < dealerTotal) return -1;
    return 0;
}

int playerPlay(int p, int playerTotal){
    int stand = dealerPlay(p, playerTotal);
    int best = stand;
    if(p < (int)deck.size()){
        int nt = playerTotal + deck[p];
        int hit = (nt > 21) ? -1 : playerPlay(p + 1, nt);
        best = max(best, hit);
    }
    return best;
}

int main(){
    deck = {10, 7, 5, 9, 6, 4, 10, 2};
    int playerTotal = deck[0] + deck[2]; // player's two cards
    cout << playerPlay(4, playerTotal) << "\n";
    return 0;
}
