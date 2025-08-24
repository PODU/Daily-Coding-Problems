// Palindrome pairs: hash map of reversed words; for each word check prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
#include <bits/stdc++.h>
using namespace std;

bool isPalin(const string& s, int i, int j) {
    while (i < j) { if (s[i++] != s[j--]) return false; }
    return true;
}

int main() {
    vector<string> words = {"code", "edoc", "da", "d"};
    unordered_map<string, int> rev;
    for (int i = 0; i < (int)words.size(); i++) {
        string r = words[i];
        reverse(r.begin(), r.end());
        rev[r] = i;
    }

    vector<pair<int,int>> res;
    for (int i = 0; i < (int)words.size(); i++) {
        const string& w = words[i];
        int n = w.size();
        for (int cut = 0; cut <= n; cut++) {
            // left part w[0..cut-1] is palindrome -> need reversed(w[cut..]) before w
            if (isPalin(w, 0, cut - 1)) {
                string suf = w.substr(cut);
                auto it = rev.find(suf);
                if (it != rev.end() && it->second != i)
                    res.push_back({it->second, i});
            }
            // right part w[cut..] is palindrome -> need reversed(w[0..cut-1]) after w
            if (cut < n && isPalin(w, cut, n - 1)) {
                string pre = w.substr(0, cut);
                auto it = rev.find(pre);
                if (it != rev.end() && it->second != i)
                    res.push_back({i, it->second});
            }
        }
    }

    sort(res.begin(), res.end());
    res.erase(unique(res.begin(), res.end()), res.end());

    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        if (i) cout << ", ";
        cout << "(" << res[i].first << ", " << res[i].second << ")";
    }
    cout << "]" << endl;
    return 0;
}
