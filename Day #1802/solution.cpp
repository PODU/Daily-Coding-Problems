// Palindrome pairs: map word->index, split each word, match palindromic halves.
// Time O(N*L^2), Space O(N*L).
#include <bits/stdc++.h>
using namespace std;

bool isPal(const string& s, int l, int r) {
    while (l < r) { if (s[l] != s[r]) return false; l++; r--; }
    return true;
}

vector<pair<int,int>> palindromePairs(const vector<string>& words) {
    unordered_map<string,int> idx;
    for (int i = 0; i < (int)words.size(); i++) idx[words[i]] = i;
    vector<pair<int,int>> res;
    for (int i = 0; i < (int)words.size(); i++) {
        const string& w = words[i];
        int n = w.size();
        for (int j = 0; j <= n; j++) {
            // case 1: prefix [0,j) palindrome -> reverse(suffix) + w
            if (isPal(w, 0, j - 1)) {
                string rs(w.begin() + j, w.end());
                reverse(rs.begin(), rs.end());
                auto it = idx.find(rs);
                if (it != idx.end() && it->second != i) res.push_back({it->second, i});
            }
            // case 2: suffix [j,n) palindrome (non-empty) -> w + reverse(prefix)
            if (j != n && isPal(w, j, n - 1)) {
                string rp(w.begin(), w.begin() + j);
                reverse(rp.begin(), rp.end());
                auto it = idx.find(rp);
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
    for (size_t i = 0; i < res.size(); i++) {
        cout << "(" << res[i].first << ", " << res[i].second << ")";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
