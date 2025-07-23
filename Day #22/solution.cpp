// Word Break reconstruction: DP over positions with memoization using a word set.
// Time: O(n^2) substring checks (n = string length), Space: O(n) for memo + recursion.
#include <bits/stdc++.h>
using namespace std;

vector<string> wordBreak(const string& s, const unordered_set<string>& dict) {
    int n = s.size();
    // memo[i] = -2 unknown, -1 impossible, else split length to use at i
    vector<int> memo(n + 1, -2);
    function<bool(int)> solve = [&](int i) -> bool {
        if (i == n) return true;
        if (memo[i] != -2) return memo[i] != -1;
        for (int j = i + 1; j <= n; ++j) {
            if (dict.count(s.substr(i, j - i)) && solve(j)) {
                memo[i] = j - i;
                return true;
            }
        }
        memo[i] = -1;
        return false;
    };
    if (!solve(0)) return {};
    vector<string> res;
    for (int i = 0; i < n; ) {
        res.push_back(s.substr(i, memo[i]));
        i += memo[i];
    }
    return res;
}

int main() {
    unordered_set<string> dict = {"quick", "brown", "the", "fox"};
    string s = "thequickbrownfox";
    vector<string> res = wordBreak(s, dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "'" << res[i] << "'";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
