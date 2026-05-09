// Day 1497: Step words. A dict word w is a step word of input s if
// len(w)==len(s)+1 and multiset(s) is a subset of multiset(w).
// Approach: char-count comparison. Time O(D*L), Space O(1) (26 alphabet).
#include <bits/stdc++.h>
using namespace std;

vector<string> stepWords(const string& word, const vector<string>& dict) {
    array<int,26> base{}; base.fill(0);
    for (char c : word) base[c - 'A']++;
    vector<string> res;
    for (const string& w : dict) {
        if (w.size() != word.size() + 1) continue;
        array<int,26> cnt{}; cnt.fill(0);
        for (char c : w) cnt[c - 'A']++;
        int extra = 0; bool ok = true;
        for (int i = 0; i < 26; i++) {
            int diff = cnt[i] - base[i];
            if (diff < 0) { ok = false; break; }
            extra += diff;
        }
        if (ok && extra == 1) res.push_back(w);
    }
    return res;
}

int main() {
    string input = "APPLE";
    vector<string> dict = {"APPEAL", "APPLE", "BANANA", "PLEASE", "APPEALS"};
    vector<string> res = stepWords(input, dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "\"" << res[i] << "\"";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
