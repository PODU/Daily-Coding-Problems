// Palindrome pairs: hash words; for each split test palindromic remainder + reversed counterpart.
// Time O(n*k^2), Space O(n*k).
#include <bits/stdc++.h>
using namespace std;

bool isPal(const string& s, int i, int j) {
    while (i < j) { if (s[i] != s[j]) return false; i++; j--; }
    return true;
}

int main() {
    vector<string> words = {"code", "edoc", "da", "d"};
    unordered_map<string, int> d;
    for (int i = 0; i < (int)words.size(); i++) d[words[i]] = i;
    set<pair<int,int>> res;
    for (int i = 0; i < (int)words.size(); i++) {
        const string& w = words[i];
        int L = w.size();
        for (int j = 0; j <= L; j++) {
            if (isPal(w, 0, j - 1)) { // left part palindrome
                string r(w.begin() + j, w.end()); reverse(r.begin(), r.end());
                auto it = d.find(r);
                if (it != d.end() && it->second != i) res.insert({it->second, i});
            }
            if (j != L && isPal(w, j, L - 1)) { // right part palindrome
                string l(w.begin(), w.begin() + j); reverse(l.begin(), l.end());
                auto it = d.find(l);
                if (it != d.end() && it->second != i) res.insert({i, it->second});
            }
        }
    }
    cout << "[";
    bool first = true;
    for (auto& p : res) { if (!first) cout << ", "; cout << "(" << p.first << ", " << p.second << ")"; first = false; }
    cout << "]\n";
}
