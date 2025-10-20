// Day 460: Min flips so every 'x' precedes every 'y'.
// One-pass DP: dp = min(flip this x->y, flip all prior y->x). Time O(n), Space O(1).
#include <iostream>
#include <string>
using namespace std;

int minFlips(const string& s) {
    int dp = 0, y = 0;
    for (char c : s) {
        if (c == 'y') y++;
        else dp = min(dp + 1, y);
    }
    return dp;
}

int main() {
    cout << minFlips("xyxxxyxyy") << endl; // 2
    return 0;
}
