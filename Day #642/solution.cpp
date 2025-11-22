// Day 642: Step words (add one letter + anagram).
// Approach: a dict word qualifies if len == len(word)+1 and its letter counts
// minus the input's are all >= 0 with exactly one extra letter.
// Time: O(D * L), Space: O(1) (26-letter counts).
#include <bits/stdc++.h>
using namespace std;

bool isStep(const string& word, const string& cand) {
    if (cand.size() != word.size() + 1) return false;
    int cnt[26] = {0};
    for (char c : cand) cnt[c - 'A']++;
    for (char c : word) if (--cnt[c - 'A'] < 0) return false;
    int extra = 0;
    for (int i = 0; i < 26; i++) extra += cnt[i];
    return extra == 1;
}

vector<string> stepWords(const string& word, const vector<string>& dict) {
    vector<string> res;
    for (const string& w : dict) if (isStep(word, w)) res.push_back(w);
    return res;
}

int main() {
    string word = "APPLE";
    vector<string> dict = {"APPEAL", "APPEAR", "PEAR", "APPLES", "PALE"};
    auto res = stepWords(word, dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "\"" << res[i] << "\"";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n"; // ["APPEAL", "APPLES"]
    return 0;
}
