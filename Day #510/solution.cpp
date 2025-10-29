// Day 510: All index pairs (i,j) where words[i]+words[j] is a palindrome.
// Hash map of reversed words + prefix/suffix palindrome checks. Time O(N*L^2).
#include <bits/stdc++.h>
using namespace std;

bool isPal(const string& s, int l, int r) {
    while (l < r) if (s[l++] != s[r--]) return false;
    return true;
}

vector<pair<int,int>> palindromePairs(const vector<string>& words) {
    unordered_map<string,int> rev;
    for (int i = 0; i < (int)words.size(); i++) {
        string r = words[i];
        reverse(r.begin(), r.end());
        rev[r] = i;
    }
    set<pair<int,int>> result;
    for (int i = 0; i < (int)words.size(); i++) {
        const string& w = words[i];
        int n = w.size();
        for (int cut = 0; cut <= n; cut++) {
            // left part is palindrome -> need reversed(right) prefix word
            if (isPal(w, 0, cut - 1)) {
                string suf = w.substr(cut);
                auto it = rev.find(suf);
                if (it != rev.end() && it->second != i)
                    result.insert({it->second, i});
            }
            // right part is palindrome -> need reversed(left) suffix word
            if (cut != n && isPal(w, cut, n - 1)) {
                string pre = w.substr(0, cut);
                auto it = rev.find(pre);
                if (it != rev.end() && it->second != i)
                    result.insert({i, it->second});
            }
        }
    }
    return vector<pair<int,int>>(result.begin(), result.end());
}

int main() {
    vector<string> words = {"code", "edoc", "da", "d"};
    auto pairs = palindromePairs(words);
    cout << "[";
    for (size_t k = 0; k < pairs.size(); k++) {
        cout << "(" << pairs[k].first << ", " << pairs[k].second << ")";
        if (k + 1 < pairs.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
