// Day 1322: Misere Nim (last stone loses) forced win for first player.
// Theorem: first player wins iff (some heap>1 and XOR!=0) OR (all heaps<=1 and XOR==0). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool firstPlayerWins(const vector<int>& heaps) {
    int x = 0, maxHeap = 0;
    for (int h : heaps) { x ^= h; maxHeap = max(maxHeap, h); }
    if (maxHeap <= 1) return x == 0;   // all heaps size <=1
    return x != 0;                     // some heap > 1
}

int main() {
    vector<int> heaps = {3, 4, 5};
    cout << (firstPlayerWins(heaps) ? "true" : "false") << endl; // true
    return 0;
}
