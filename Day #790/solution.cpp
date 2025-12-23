// Step words: a dict word qualifies if len==word.len+1 and word's letter
// multiset is a subset leaving exactly one extra letter. O(dict*wordLen) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

vector<string> stepWords(const string& word, const vector<string>& dict) {
    array<int,26> base{};
    for (char c : word) base[c - 'A']++;
    vector<string> res;
    for (const string& w : dict) {
        if ((int)w.size() != (int)word.size() + 1) continue;
        array<int,26> cnt{};
        for (char c : w) cnt[c - 'A']++;
        int extra = 0; bool ok = true;
        for (int i = 0; i < 26; i++) {
            int d = cnt[i] - base[i];
            if (d < 0) { ok = false; break; }
            extra += d;
        }
        if (ok && extra == 1) res.push_back(w);
    }
    return res;
}

int main() {
    string word = "APPLE";
    vector<string> dict = {"APPEAL", "BANANA", "ORANGE", "GRAPE"};
    vector<string> res = stepWords(word, dict);
    for (size_t i = 0; i < res.size(); i++) {
        cout << res[i];
        if (i + 1 < res.size()) cout << " ";
    }
    cout << endl;
    return 0;
}
