// Day 710: Find start indices where s contains a concatenation of all equal-length
// words exactly once. Sliding window over wordLen offsets. Time O(n*wordLen).
#include <bits/stdc++.h>
using namespace std;

vector<int> findSubstring(const string& s, const vector<string>& words) {
    vector<int> res;
    if (words.empty()) return res;
    int wl = words[0].size(), k = words.size(), total = wl * k, n = s.size();
    if (total > n) return res;
    unordered_map<string, int> need;
    for (auto& w : words) need[w]++;
    for (int off = 0; off < wl; ++off) {
        int left = off, count = 0;
        unordered_map<string, int> window;
        for (int j = off; j + wl <= n; j += wl) {
            string w = s.substr(j, wl);
            if (need.count(w)) {
                window[w]++; count++;
                while (window[w] > need[w]) {
                    string lw = s.substr(left, wl);
                    window[lw]--; left += wl; count--;
                }
                if (count == k) { res.push_back(left);
                    string lw = s.substr(left, wl);
                    window[lw]--; left += wl; count--;
                }
            } else {
                window.clear(); count = 0; left = j + wl;
            }
        }
    }
    sort(res.begin(), res.end());
    return res;
}

void report(const string& s, const vector<string>& w) {
    auto r = findSubstring(s, w);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl;
}

int main() {
    report("dogcatcatcodecatdog", {"cat", "dog"});
    report("barfoobazbitbyte", {"dog", "cat"});
    return 0;
}
