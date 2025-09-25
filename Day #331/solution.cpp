// Min flips so all x's precede all y's. Greedy: res=min(res+flip-x, count-y).
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minFlips(const string& s) {
    int res = 0, yCount = 0;
    for (char ch : s) {
        if (ch == 'y') yCount++;
        else res = min(res + 1, yCount);
    }
    return res;
}

int main() {
    cout << minFlips("xyxxxyxyy") << "\n";
    return 0;
}
