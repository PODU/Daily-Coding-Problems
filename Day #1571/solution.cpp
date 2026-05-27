// Word break reconstruction via DP with backpointers. O(n^2) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<string> wordBreak(const string& s, const unordered_set<string>& dict) {
    int n = s.size();
    vector<int> back(n + 1, -2); // -2 unreachable, -1 start
    back[0] = -1;
    for (int i = 1; i <= n; ++i)
        for (int j = 0; j < i; ++j)
            if (back[j] != -2 && dict.count(s.substr(j, i - j))) { back[i] = j; break; }
    if (back[n] == -2) return {};
    vector<string> res;
    for (int i = n; i > 0; i = back[i]) res.push_back(s.substr(back[i], i - back[i]));
    reverse(res.begin(), res.end());
    return res;
}

int main() {
    unordered_set<string> dict = {"quick", "brown", "the", "fox"};
    string s = "thequickbrownfox";
    auto res = wordBreak(s, dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << "'" << res[i] << "'" << (i + 1 < res.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
