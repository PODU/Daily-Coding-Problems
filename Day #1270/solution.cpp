// Day 1270: Find all start indices in S that are anagrams of W.
// Fixed-size sliding window with a 26-length count and a match counter. O(|S|) time.
#include <bits/stdc++.h>
using namespace std;

vector<int> findAnagrams(const string& w, const string& s) {
    vector<int> res;
    int m = w.size(), n = s.size();
    if (m > n) return res;
    array<int,26> need{}, win{};
    for (char c : w) need[c - 'a']++;
    for (int i = 0; i < n; ++i) {
        win[s[i] - 'a']++;
        if (i >= m) win[s[i - m] - 'a']--;
        if (i >= m - 1 && win == need) res.push_back(i - m + 1);
    }
    return res;
}

int main() {
    auto res = findAnagrams("ab", "abxaba");
    for (size_t i = 0; i < res.size(); ++i) { cout << res[i]; if (i + 1 < res.size()) cout << ", "; }
    cout << endl;
    return 0;
}
