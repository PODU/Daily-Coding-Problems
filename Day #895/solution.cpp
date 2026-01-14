// Palindrome by deleting at most k chars: min deletions = n - LPS(s).
// LPS via interval DP (space-optimized to O(n)). Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool canMakePalindrome(const string& s, int k) {
    int n = s.size();
    vector<int> prev(n, 0), cur(n, 0);
    for (int i = n - 1; i >= 0; --i) {
        cur.assign(n, 0);
        cur[i] = 1;
        for (int j = i + 1; j < n; ++j) {
            if (s[i] == s[j]) cur[j] = prev[j - 1] + 2;
            else cur[j] = max(prev[j], cur[j - 1]);
        }
        prev = cur;
    }
    int lps = n ? cur[n - 1] : 0;
    return (n - lps) <= k;
}

int main() {
    cout << (canMakePalindrome("waterrfetawx", 2) ? "True" : "False") << "\n";
    return 0;
}
