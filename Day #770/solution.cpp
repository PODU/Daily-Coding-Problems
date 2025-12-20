// Day 770: Misere Nim forced-win check.
// If every heap == 1: first player wins iff count of heaps is even.
// Else: first player wins iff XOR of heaps != 0. O(N).
#include <bits/stdc++.h>
using namespace std;

bool firstPlayerWins(const vector<int>& heaps) {
    int xorSum = 0; bool allOne = true;
    for (int h : heaps) { xorSum ^= h; if (h > 1) allOne = false; }
    if (allOne) return (heaps.size() % 2) == 0;
    return xorSum != 0;
}

int main() {
    vector<int> heaps = {3, 4, 5};
    cout << boolalpha << firstPlayerWins(heaps) << endl; // true
    return 0;
}
