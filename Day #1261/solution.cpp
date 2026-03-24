// Day 1261: Palindrome pairs.
// Hashmap of reversed words + prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
#include <bits/stdc++.h>
using namespace std;

static bool isPal(const string& s, int i, int j) {
    while (i < j) { if (s[i] != s[j]) return false; ++i; --j; }
    return true;
}

vector<pair<int,int>> palindromePairs(const vector<string>& words) {
    unordered_map<string,int> idx;
    for (int i = 0; i < (int)words.size(); ++i) idx[words[i]] = i;
    vector<pair<int,int>> res;
    for (int i = 0; i < (int)words.size(); ++i) {
        const string& w = words[i];
        int n = w.size();
        for (int j = 0; j <= n; ++j) {
            // prefix w[0..j) palindrome -> rev(suffix) + w
            if (isPal(w, 0, j - 1)) {
                string back(w.begin() + j, w.end());
                reverse(back.begin(), back.end());
                auto it = idx.find(back);
                if (it != idx.end() && it->second != i) res.push_back({it->second, i});
            }
            // suffix w[j..n) palindrome -> w + rev(prefix)
            if (j != n && isPal(w, j, n - 1)) {
                string back(w.begin(), w.begin() + j);
                reverse(back.begin(), back.end());
                auto it = idx.find(back);
                if (it != idx.end() && it->second != i) res.push_back({i, it->second});
            }
        }
    }
    sort(res.begin(), res.end());
    res.erase(unique(res.begin(), res.end()), res.end());
    return res;
}

int main() {
    vector<string> words = {"code", "edoc", "da", "d"};
    auto res = palindromePairs(words);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "(" << res[i].first << ", " << res[i].second << ")";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
