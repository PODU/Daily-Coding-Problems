// Day 767: Find all start indices in S that are anagrams of W.
// Sliding window of size |W| with a 26-count match counter. O(|S|) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> findAnagrams(const string& s, const string& w) {
    vector<int> res;
    int n = s.size(), m = w.size();
    if (m > n) return res;
    array<int, 26> need{}, win{};
    for (char c : w) need[c - 'a']++;
    for (int i = 0; i < n; i++) {
        win[s[i] - 'a']++;
        if (i >= m) win[s[i - m] - 'a']--;
        if (i >= m - 1 && win == need) res.push_back(i - m + 1);
    }
    return res;
}

int main() {
    auto r = findAnagrams("abxaba", "ab");
    for (size_t i = 0; i < r.size(); i++)
        cout << r[i] << (i + 1 < r.size() ? ", " : "\n"); // 0, 3, 4
    return 0;
}
