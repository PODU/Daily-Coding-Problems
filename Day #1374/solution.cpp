// Jump game: greedy track furthest reachable index. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool canReach(const vector<int>& a) {
    int reach = 0;
    for (int i = 0; i < (int)a.size(); i++) {
        if (i > reach) return false;
        reach = max(reach, i + a[i]);
    }
    return true;
}

int main() {
    cout << (canReach({2, 0, 1, 0}) ? "True" : "False") << "\n";
    cout << (canReach({1, 1, 0, 1}) ? "True" : "False") << "\n";
    return 0;
}
