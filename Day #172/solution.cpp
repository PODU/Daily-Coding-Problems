// Substring concatenation of all words: sliding window over wordLen offsets with hash-map counts.
// Time O(n * wordLen), Space O(words * wordLen).
#include <bits/stdc++.h>
using namespace std;

vector<int> findSubstring(const string& s, const vector<string>& words) {
    vector<int> res;
    if (words.empty() || s.empty()) return res;
    int wl = words[0].size(), nw = words.size(), total = wl * nw, n = s.size();
    if (total > n) return res;
    unordered_map<string,int> need;
    for (auto& w : words) need[w]++;
    for (int i = 0; i < wl; ++i) {
        int left = i, count = 0;
        unordered_map<string,int> window;
        for (int j = i; j + wl <= n; j += wl) {
            string w = s.substr(j, wl);
            if (need.count(w)) {
                window[w]++; count++;
                while (window[w] > need[w]) {
                    string lw = s.substr(left, wl);
                    window[lw]--; count--; left += wl;
                }
                if (count == nw) { res.push_back(left); window[s.substr(left,wl)]--; count--; left += wl; }
            } else {
                window.clear(); count = 0; left = j + wl;
            }
        }
    }
    sort(res.begin(), res.end());
    return res;
}

void print(const vector<int>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i) { if (i) cout << ", "; cout << v[i]; }
    cout << "]\n";
}

int main() {
    print(findSubstring("dogcatcatcodecatdog", {"cat","dog"}));
    print(findSubstring("barfoobazbitbyte", {"dog","cat"}));
    return 0;
}
