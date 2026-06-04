// Decode ways: DP rolling two states. ways[i] = ways[i-1](if s[i] valid) + ways[i-2](if s[i-1..i] in 10..26).
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int numDecodings(const string& s) {
    if (s.empty() || s[0] == '0') return 0;
    int prev2 = 1, prev1 = 1; // ways up to i-2 and i-1
    for (size_t i = 1; i < s.size(); ++i) {
        int cur = 0;
        if (s[i] != '0') cur += prev1;
        int two = (s[i-1] - '0') * 10 + (s[i] - '0');
        if (two >= 10 && two <= 26) cur += prev2;
        prev2 = prev1;
        prev1 = cur;
    }
    return prev1;
}

int main() {
    cout << numDecodings("111") << endl;
    return 0;
}
