// Phone keypad letter combinations via backtracking. O(prod of choices) time.
#include <bits/stdc++.h>
using namespace std;

const map<char, string> M = {
    {'2', "abc"}, {'3', "def"}, {'4', "ghi"}, {'5', "jkl"},
    {'6', "mno"}, {'7', "pqrs"}, {'8', "tuv"}, {'9', "wxyz"}
};

void backtrack(const string& digits, int i, string& cur, vector<string>& out) {
    if (i == (int)digits.size()) {
        if (!digits.empty()) out.push_back(cur);
        return;
    }
    for (char c : M.at(digits[i])) {
        cur.push_back(c);
        backtrack(digits, i + 1, cur, out);
        cur.pop_back();
    }
}

int main() {
    string digits = "23";
    vector<string> out;
    string cur;
    backtrack(digits, 0, cur, out);
    cout << "[";
    for (size_t i = 0; i < out.size(); ++i) {
        if (i) cout << ", ";
        cout << out[i];
    }
    cout << "]" << endl;
    return 0;
}
