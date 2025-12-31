// Day 831: All start indices of substrings that are a concatenation of every word once.
// Sliding window per offset 0..L-1 with hashmap word counts. O(n * L) ~ O(n) total.
#include <bits/stdc++.h>
using namespace std;

vector<int> findSubstring(const string& s, const vector<string>& words) {
    vector<int> res;
    if (words.empty() || s.empty()) return res;
    int L = words[0].size(), m = words.size(), n = s.size();
    if ((long long)L * m > n) return res;
    unordered_map<string, int> need;
    for (auto& w : words) need[w]++;

    for (int offset = 0; offset < L; offset++) {
        int left = offset, count = 0;
        unordered_map<string, int> have;
        for (int right = offset; right + L <= n; right += L) {
            string w = s.substr(right, L);
            if (need.count(w)) {
                have[w]++;
                count++;
                while (have[w] > need[w]) {
                    string lw = s.substr(left, L);
                    have[lw]--;
                    left += L;
                    count--;
                }
                if (count == m) {
                    res.push_back(left);
                    string lw = s.substr(left, L);
                    have[lw]--;
                    left += L;
                    count--;
                }
            } else {
                have.clear();
                count = 0;
                left = right + L;
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
    printVec(findSubstring("dogcatcatcodecatdog", {"cat", "dog"}));  // [0, 13]
    printVec(findSubstring("barfoobazbitbyte", {"dog", "cat"}));     // []
    return 0;
}
