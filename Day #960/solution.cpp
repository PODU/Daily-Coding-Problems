// Day 960: jump game - can we reach the last index? Greedy furthest-reach.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool canReach(const vector<int>& a) {
    int reach = 0;
    for (int i = 0; i < (int)a.size(); ++i) {
        if (i > reach) return false;
        reach = max(reach, i + a[i]);
    }
    return true;
}

int main() {
    cout << (canReach({1, 3, 1, 2, 0, 1}) ? "true" : "false") << "\n"; // true
    cout << (canReach({1, 2, 1, 0, 0}) ? "true" : "false") << "\n";    // false
    return 0;
}
