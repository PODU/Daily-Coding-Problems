// Min flips so all x precede all y. DP: at each char, flips = min(yCount, flips+1):
// flipping the current 'y'->'x' costs all prior y's; flipping the current 'x'->'y' costs flips+1. Time O(n), space O(1).
#include <bits/stdc++.h>
using namespace std;

int minFlips(const string& s) {
    int flips = 0, y = 0;
    for (char c : s) {
        if (c == 'y') y++;
        else flips = min(y, flips + 1);
    }
    return flips;
}

int main() {
    cout << minFlips("xyxxxyxyy") << endl; // 2
    return 0;
}
