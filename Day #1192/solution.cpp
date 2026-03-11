// Step word: dict word of length len(input)+1 that contains all input letters plus exactly one extra,
// AND is a genuine anagram (rearrangement), not the input with a letter merely appended (no input prefix).
// Compare 26-letter frequency counts: every count >= input's and total diff == 1. O(D * (L + 26)).
#include <bits/stdc++.h>
using namespace std;

static bool hasPrefix(const string& w, const string& p) {
    return w.size() >= p.size() && w.compare(0, p.size(), p) == 0;
}

vector<string> stepWords(const string& word, const vector<string>& dict) {
    array<int,26> base{};
    for (char c : word) base[c - 'A']++;
    vector<string> res;
    for (const string& w : dict) {
        if ((int)w.size() != (int)word.size() + 1) continue;
        array<int,26> cnt{};
        for (char c : w) cnt[c - 'A']++;
        bool ok = true;
        int diff = 0;
        for (int i = 0; i < 26; i++) {
            if (cnt[i] < base[i]) { ok = false; break; }
            diff += cnt[i] - base[i];
        }
        if (ok && diff == 1 && !hasPrefix(w, word)) res.push_back(w);
    }
    return res;
}

int main() {
    string word = "APPLE";
    vector<string> dict = {"APPEAL","APPLE","PEAR","PALE","APPEALS","PAPER","APPLES","LAPEL"};
    for (const string& w : stepWords(word, dict)) cout << w << "\n";
    return 0;
}
