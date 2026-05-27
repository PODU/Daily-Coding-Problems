// Phone keypad letter combinations via iterative Cartesian product. O(prod*len) time, O(output) space.
#include <bits/stdc++.h>
using namespace std;

vector<string> letterCombinations(const string& digits, map<char, string>& mp) {
    if (digits.empty()) return {};
    vector<string> res = {""};
    for (char d : digits) {
        vector<string> next;
        for (const string& pre : res)
            for (char c : mp[d]) next.push_back(pre + c);
        res = next;
    }
    return res;
}

int main() {
    map<char, string> mp = {{'2', "abc"}, {'3', "def"}};
    auto res = letterCombinations("23", mp);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << "\"" << res[i] << "\"" << (i + 1 < res.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
