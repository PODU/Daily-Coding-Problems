// Day 1265: Reconstruct a sentence from a space-free string and a dictionary.
// DP over prefixes storing one valid breakpoint. O(n^2) time (avg), O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<string> wordBreak(const string& s, const unordered_set<string>& dict) {
    int n = s.size();
    vector<int> back(n + 1, -2); // back[i] = start index of word ending at i, -2 = unreachable
    back[0] = -1;
    for (int i = 1; i <= n; ++i) {
        for (int j = 0; j < i; ++j) {
            if (back[j] != -2 && dict.count(s.substr(j, i - j))) { back[i] = j; break; }
        }
    }
    if (back[n] == -2) return {"<null>"};
    vector<string> res;
    for (int i = n; i > 0; i = back[i]) res.push_back(s.substr(back[i], i - back[i]));
    reverse(res.begin(), res.end());
    return res;
}

int main() {
    unordered_set<string> dict = {"quick", "brown", "the", "fox"};
    auto res = wordBreak("thequickbrownfox", dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) { cout << "'" << res[i] << "'"; if (i + 1 < res.size()) cout << ", "; }
    cout << "]" << endl;
    return 0;
}
