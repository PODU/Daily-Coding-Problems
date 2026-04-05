// Fisher-Yates shuffle using a uniform rand(1..k). Each of N! permutations
// equally likely. Time O(N), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

static mt19937 rng(12345);
// uniform random integer in [1, k]
int randK(int k) { return (int)(rng() % k) + 1; }

void shuffleDeck(vector<int>& deck) {
    int n = deck.size();
    for (int i = n - 1; i > 0; --i) {
        int j = randK(i + 1) - 1;     // uniform in [0, i]
        swap(deck[i], deck[j]);
    }
}

int main() {
    vector<int> deck(52);
    iota(deck.begin(), deck.end(), 1);
    shuffleDeck(deck);
    for (int i = 0; i < (int)deck.size(); ++i)
        cout << deck[i] << (i + 1 < (int)deck.size() ? ' ' : '\n');
    return 0;
}
