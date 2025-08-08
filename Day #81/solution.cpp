// Day 81: Phone-number letter combinations via iterative cartesian product.
// Time O(prod of letters * len), Space O(output).
#include <bits/stdc++.h>
using namespace std;

vector<string> letterCombinations(map<char, vector<string>>& mapping, const string& digits) {
    vector<string> res = {""};
    for (char d : digits) {
        vector<string> next;
        for (auto& prefix : res)
            for (auto& letter : mapping[d])
                next.push_back(prefix + letter);
        res = next;
    }
    if (digits.empty()) return {};
    return res;
}

int main() {
    map<char, vector<string>> mapping = {
        {'2', {"a","b","c"}}, {'3', {"d","e","f"}}, {'4', {"g","h","i"}},
        {'5', {"j","k","l"}}, {'6', {"m","n","o"}}, {'7', {"p","q","r","s"}},
        {'8', {"t","u","v"}}, {'9', {"w","x","y","z"}}};
    auto res = letterCombinations(mapping, "23");
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << "\"" << res[i] << "\"" << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    return 0;
}
