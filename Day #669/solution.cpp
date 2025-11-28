// Day 669: Misere Nim. First player wins iff either (some heap>1 and xor!=0) or
// (all heaps<=1 and xor==0). Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool firstPlayerWins(const vector<int>& heaps) {
    int x = 0; bool anyBig = false;
    for (int h : heaps) { x ^= h; if (h > 1) anyBig = true; }
    return anyBig ? (x != 0) : (x == 0);
}

int main() {
    vector<int> heaps = {3, 4, 5};
    cout << (firstPlayerWins(heaps) ? "True" : "False") << "\n"; // True
    return 0;
}
