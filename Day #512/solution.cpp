// Jump Game: greedily track the furthest reachable index. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool canJump(const vector<int>& a) {
    int reach = 0;
    for (int i = 0; i < (int)a.size(); i++) {
        if (i > reach) return false;
        reach = max(reach, i + a[i]);
    }
    return true;
}

int main() {
    cout << (canJump({1, 3, 1, 2, 0, 1}) ? "true" : "false") << "\n";
    cout << (canJump({1, 2, 1, 0, 0}) ? "true" : "false") << "\n";
    return 0;
}
