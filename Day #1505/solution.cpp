// Jump game: greedy, track furthest reachable index.
// Time O(n), Space O(1). Prints "True"/"False" (capitalized) per spec.
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
    vector<int> a = {2, 0, 1, 0};
    cout << (canJump(a) ? "True" : "False") << "\n";
    return 0;
}
