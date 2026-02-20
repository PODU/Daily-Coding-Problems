// Day 1104: Phone digit -> letters combinations via backtracking.
// Time: O(prod of choices * len). Space: O(len) recursion.
#include <bits/stdc++.h>
using namespace std;

void dfs(const string& digits, int i, string& cur,
         unordered_map<char,vector<char>>& mp, vector<string>& out){
    if (i == (int)digits.size()) { out.push_back(cur); return; }
    for (char c : mp[digits[i]]) { cur.push_back(c); dfs(digits, i+1, cur, mp, out); cur.pop_back(); }
}

vector<string> letterCombos(const string& digits, unordered_map<char,vector<char>> mp){
    vector<string> out;
    if (digits.empty()) return out;
    string cur;
    dfs(digits, 0, cur, mp, out);
    return out;
}

int main(){
    unordered_map<char,vector<char>> mp = {{'2',{'a','b','c'}},{'3',{'d','e','f'}}};
    auto res = letterCombos("23", mp);
    cout << "[";
    for (size_t i=0;i<res.size();i++) cout << res[i] << (i+1<res.size()?", ":"");
    cout << "]\n"; // [ad, ae, af, bd, be, bf, cd, ce, cf]
    return 0;
}
