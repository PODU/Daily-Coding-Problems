// Fisher-Yates shuffle using a rand(k) helper returning [1,k]; O(N) time, O(1) extra space.
// Each of the N! permutations is equally likely. Fixed seed -> reproducible output.
#include <bits/stdc++.h>
using namespace std;

static mt19937 rng(12345);

// Uniform integer in [1, k] using the provided RNG.
int randk(int k) {
    return (int)(rng() % k) + 1;
}

void shuffle_deck(vector<int>& arr) {
    int n = arr.size();
    for (int i = n - 1; i >= 1; --i) {
        int j = randk(i + 1) - 1; // index in [0, i]
        swap(arr[i], arr[j]);
    }
}

int main() {
    vector<int> deck(52);
    for (int i = 0; i < 52; ++i) deck[i] = i + 1; // cards 1..52
    shuffle_deck(deck);
    for (int i = 0; i < 52; ++i) {
        cout << deck[i];
        if (i + 1 < 52) cout << ' ';
    }
    cout << endl;
    return 0;
}
