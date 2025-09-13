// Day 266: Step words. A dict word is a step word of `word` if its length is
// one greater and its letter multiset is a superset of `word`'s (diff = 1).
// Time O(D * L) over dictionary; space O(alphabet).
#include <bits/stdc++.h>
using namespace std;

bool isStepWord(const string& word, const string& cand) {
    if (cand.size() != word.size() + 1) return false;
    array<int, 26> cnt{};
    for (char c : cand) cnt[tolower(c) - 'a']++;
    for (char c : word) {
        int i = tolower(c) - 'a';
        if (--cnt[i] < 0) return false;
    }
    int extra = 0;
    for (int v : cnt) extra += v;
    return extra == 1;
}

vector<string> stepWords(const string& word, const vector<string>& dict) {
    vector<string> res;
    for (const auto& w : dict)
        if (isStepWord(word, w)) res.push_back(w);
    return res;
}

int main() {
    string word = "APPLE";
    vector<string> dict = {"APPEAL", "APPLES", "KELP", "PALE", "APPLE"};
    vector<string> res = stepWords(word, dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
