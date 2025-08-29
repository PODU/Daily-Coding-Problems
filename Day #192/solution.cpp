// Day 192: Jump game -- can we reach the end (each value caps the jump length).
// Greedy farthest-reach. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool canReachEnd(const vector<int>& a) {
    int reach = 0;
    for (int i = 0; i < (int)a.size(); i++) {
        if (i > reach) return false;
        reach = max(reach, i + a[i]);
    }
    return true;
}

int main() {
    cout << boolalpha << canReachEnd({1, 3, 1, 2, 0, 1}) << "\n";
    cout << boolalpha << canReachEnd({1, 2, 1, 0, 0}) << "\n";
    return 0;
}
