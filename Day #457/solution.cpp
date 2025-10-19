// Day 457: All start indices in S that are anagrams of W.
// Fixed-size sliding window of char counts. Time O(|S|), Space O(1).
#include <iostream>
#include <vector>
#include <string>
using namespace std;

vector<int> anagramIndices(const string& w, const string& s) {
    vector<int> res;
    int m = w.size(), n = s.size();
    if (m == 0 || m > n) return res;
    int need[256] = {0}, win[256] = {0};
    for (char c : w) need[(unsigned char)c]++;
    for (int i = 0; i < n; i++) {
        win[(unsigned char)s[i]]++;
        if (i >= m) win[(unsigned char)s[i - m]]--;
        if (i >= m - 1) {
            bool ok = true;
            for (int c = 0; c < 256; c++) if (need[c] != win[c]) { ok = false; break; }
            if (ok) res.push_back(i - m + 1);
        }
    }
    return res;
}

int main() {
    auto r = anagramIndices("ab", "abxaba");
    for (size_t i = 0; i < r.size(); i++)
        cout << r[i] << (i + 1 < r.size() ? ", " : "\n"); // 0, 3, 4
    return 0;
}
