// Fisher-Yates shuffle: uniform random permutation using only swaps.
// rand(k) gives a uniform int in [1,k]; each of N! orderings is equally likely.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(12345);
// uniform random number between 1 and k inclusive
int randk(int k) { return uniform_int_distribution<int>(1, k)(rng); }

void shuffle(vector<int>& deck) {
    int n = deck.size();
    for (int i = n - 1; i > 0; i--) {
        int j = randk(i + 1) - 1; // index in [0, i]
        swap(deck[i], deck[j]);
    }
}

int main() {
    vector<int> deck(52);
    iota(deck.begin(), deck.end(), 1);
    shuffle(deck);
    for (int i = 0; i < (int)deck.size(); i++)
        cout << deck[i] << (i + 1 < (int)deck.size() ? " " : "\n");
    return 0;
}
