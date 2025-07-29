// Day 51: Uniform shuffle via Fisher-Yates, using only rand(1..k) and swaps.
// Each of n! permutations equally likely. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(random_device{}());
// Perfectly random integer in [1, k].
int randK(int k) { return uniform_int_distribution<int>(1, k)(rng); }

void shuffle(vector<int>& deck) {
    int n = deck.size();
    for (int i = n - 1; i > 0; i--) {
        int j = randK(i + 1) - 1;     // uniform index in [0, i]
        swap(deck[i], deck[j]);
    }
}

int main() {
    vector<int> deck(52);
    iota(deck.begin(), deck.end(), 0);   // cards 0..51
    shuffle(deck);
    for (size_t i = 0; i < deck.size(); i++)
        cout << deck[i] << (i + 1 < deck.size() ? " " : "\n");
    // Verify it is a valid permutation.
    vector<int> seen(deck);
    sort(seen.begin(), seen.end());
    bool ok = true;
    for (int i = 0; i < 52; i++) if (seen[i] != i) ok = false;
    cout << "valid permutation: " << (ok ? "true" : "false") << endl;
    return 0;
}
