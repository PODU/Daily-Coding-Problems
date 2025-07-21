// Decode ways: DP where ways[i] depends on 1-digit and valid 2-digit suffixes.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int numDecodings(const string& s) {
    if (s.empty() || s[0] == '0') return 0;
    int prev2 = 1, prev1 = 1; // ways before pos, ways at pos-1
    for (int i = 1; i < (int)s.size(); i++) {
        int cur = 0;
        if (s[i] != '0') cur += prev1;
        int two = (s[i - 1] - '0') * 10 + (s[i] - '0');
        if (two >= 10 && two <= 26) cur += prev2;
        prev2 = prev1;
        prev1 = cur;
    }
    return prev1;
}

int main() {
    cout << numDecodings("111") << "\n";
    return 0;
}
