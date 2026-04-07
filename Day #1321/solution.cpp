// Day 1321: Roman numeral -> decimal.
// Approach: single left-to-right pass; subtract if a smaller value precedes a larger one. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int romanToInt(const string& s) {
    unordered_map<char,int> v = {{'M',1000},{'D',500},{'C',100},{'L',50},{'X',10},{'V',5},{'I',1}};
    int total = 0;
    for (size_t i = 0; i < s.size(); ++i) {
        int cur = v[s[i]];
        if (i + 1 < s.size() && cur < v[s[i+1]]) total -= cur;
        else total += cur;
    }
    return total;
}

int main() {
    string s = "XIV";
    cout << romanToInt(s) << endl; // 14
    return 0;
}
