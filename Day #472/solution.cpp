// Count decodings (a=1..z=26) with bottom-up DP keeping only two running states.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int numDecodings(const string& s) {
    if (s.empty() || s[0] == '0') return 0;
    int prev2 = 1; // ways up to i-2
    int prev1 = 1; // ways up to i-1
    for (size_t i = 1; i < s.size(); ++i) {
        int cur = 0;
        if (s[i] != '0') cur += prev1;              // single digit 1..9
        int two = (s[i - 1] - '0') * 10 + (s[i] - '0');
        if (two >= 10 && two <= 26) cur += prev2;   // two digit 10..26
        prev2 = prev1;
        prev1 = cur;
    }
    return prev1;
}

int main() {
    string msg = "111";
    cout << numDecodings(msg) << "\n";
    return 0;
}
