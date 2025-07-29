// Day 46: Longest palindromic substring via Manacher's algorithm.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string longestPalindrome(const string& s) {
    if (s.empty()) return "";
    string t = "^";
    for (char c : s) { t += '#'; t += c; }
    t += "#$";
    int n = t.size();
    vector<int> p(n, 0);
    int c = 0, r = 0;
    for (int i = 1; i < n - 1; i++) {
        if (i < r) p[i] = min(r - i, p[2 * c - i]);
        while (t[i + 1 + p[i]] == t[i - 1 - p[i]]) p[i]++;
        if (i + p[i] > r) { c = i; r = i + p[i]; }
    }
    int maxLen = 0, centerIndex = 0;
    for (int i = 1; i < n - 1; i++)
        if (p[i] > maxLen) { maxLen = p[i]; centerIndex = i; }
    int start = (centerIndex - maxLen) / 2;
    return s.substr(start, maxLen);
}

int main() {
    cout << "\"" << longestPalindrome("aabcdcb") << "\"" << endl;
    cout << "\"" << longestPalindrome("bananas") << "\"" << endl;
    return 0;
}
