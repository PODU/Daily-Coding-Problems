// Word break reconstruction via memoized DP: for each suffix, try each prefix
// word and recurse. Time: O(n^2 * L) with memo, Space: O(n^2).
#include <iostream>
#include <unordered_set>
#include <unordered_map>
#include <vector>
#include <string>
using namespace std;

bool solve(const string& s, int start, const unordered_set<string>& dict,
           unordered_map<int, vector<string>>& memo, vector<string>& out) {
    if (start == (int)s.size()) return true;
    auto it = memo.find(start);
    if (it != memo.end()) {
        if (it->second.empty()) return false;
        out = it->second;
        return true;
    }
    for (int end = start + 1; end <= (int)s.size(); ++end) {
        string word = s.substr(start, end - start);
        if (dict.count(word)) {
            vector<string> rest;
            if (solve(s, end, dict, memo, rest)) {
                vector<string> res;
                res.push_back(word);
                for (auto& w : rest) res.push_back(w);
                memo[start] = res;
                out = res;
                return true;
            }
        }
    }
    memo[start] = {};
    return false;
}

void run(const unordered_set<string>& dict, const string& s) {
    unordered_map<int, vector<string>> memo;
    vector<string> out;
    if (solve(s, 0, dict, memo, out)) {
        cout << '[';
        for (size_t i = 0; i < out.size(); ++i) {
            if (i) cout << ", ";
            cout << '\'' << out[i] << '\'';
        }
        cout << "]\n";
    } else {
        cout << "null\n";
    }
}

int main() {
    run({"quick", "brown", "the", "fox"}, "thequickbrownfox");
    run({"bed", "bath", "bedbath", "and", "beyond"}, "bedbathandbeyond");
    return 0;
}
