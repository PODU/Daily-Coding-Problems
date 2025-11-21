// Day 639: Letter combinations of a phone number.
// Approach: iterative/backtracking Cartesian product over digit->letters map.
// Time: O(4^n * n), Space: O(4^n).
#include <bits/stdc++.h>
using namespace std;

vector<string> letterCombinations(const string& digits,
                                  const map<char, vector<string>>& m) {
    if (digits.empty()) return {};
    vector<string> res = {""};
    for (char d : digits) {
        vector<string> next;
        for (const string& prefix : res)
            for (const string& ch : m.at(d))
                next.push_back(prefix + ch);
        res = next;
    }
    return res;
}

int main() {
    map<char, vector<string>> m = {
        {'2', {"a","b","c"}}, {'3', {"d","e","f"}}, {'4', {"g","h","i"}},
        {'5', {"j","k","l"}}, {'6', {"m","n","o"}}, {'7', {"p","q","r","s"}},
        {'8', {"t","u","v"}}, {'9', {"w","x","y","z"}}
    };
    auto res = letterCombinations("23", m);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "\"" << res[i] << "\"";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
