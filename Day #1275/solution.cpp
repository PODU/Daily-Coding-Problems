// Day 1275: Longest palindromic substring via Manacher's algorithm. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

string longestPalindrome(const string& s) {
    if (s.empty()) return "";
    string t = "^";
    for (char c : s) { t += '#'; t += c; }
    t += "#$";
    int n = t.size();
    vector<int> p(n, 0);
    int center = 0, right = 0;
    for (int i = 1; i < n - 1; ++i) {
        if (i < right) p[i] = min(right - i, p[2 * center - i]);
        while (t[i + p[i] + 1] == t[i - p[i] - 1]) p[i]++;
        if (i + p[i] > right) { center = i; right = i + p[i]; }
    }
    int maxLen = 0, centerIndex = 0;
    for (int i = 1; i < n - 1; ++i) if (p[i] > maxLen) { maxLen = p[i]; centerIndex = i; }
    int start = (centerIndex - maxLen) / 2;
    return s.substr(start, maxLen);
}

int main() {
    cout << "\"" << longestPalindrome("aabcdcb") << "\"" << endl; // "bcdcb"
    cout << "\"" << longestPalindrome("bananas") << "\"" << endl; // "anana"
    return 0;
}
