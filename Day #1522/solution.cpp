// Concatenation of all equal-length words: sliding window per offset (0..L-1).
// Time O(|s| * L), Space O(words). Each word used exactly once.
#include <bits/stdc++.h>
using namespace std;

vector<int> findSubstring(const string& s, const vector<string>& words) {
    vector<int> res;
    if (words.empty()) return res;
    int L = words[0].size(), k = words.size(), n = s.size();
    int total = L * k;
    if (total > n) return res;
    unordered_map<string, int> need;
    for (auto& w : words) need[w]++;
    for (int off = 0; off < L; off++) {
        int left = off, count = 0;
        unordered_map<string, int> win;
        for (int j = off; j + L <= n; j += L) {
            string w = s.substr(j, L);
            if (need.count(w)) {
                win[w]++;
                count++;
                while (win[w] > need[w]) {
                    string lw = s.substr(left, L);
                    win[lw]--;
                    left += L;
                    count--;
                }
                if (count == k) {
                    res.push_back(left);
                    string lw = s.substr(left, L);
                    win[lw]--;
                    left += L;
                    count--;
                }
            } else {
                win.clear();
                count = 0;
                left = j + L;
            }
        }
    }
    sort(res.begin(), res.end());
    return res;
}

void printVec(const vector<int>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); i++) {
        if (i) cout << ", ";
        cout << v[i];
    }
    cout << "]\n";
}

int main() {
    printVec(findSubstring("dogcatcatcodecatdog", {"cat", "dog"}));
    printVec(findSubstring("barfoobazbitbyte", {"dog", "cat"}));
    return 0;
}
