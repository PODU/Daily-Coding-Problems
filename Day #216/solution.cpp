// Day 216: Roman numeral -> decimal.
// Approach: scan left-to-right; if current value < next, subtract, else add. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int romanToInt(const string& s) {
    unordered_map<char, int> v = {{'M',1000},{'D',500},{'C',100},{'L',50},{'X',10},{'V',5},{'I',1}};
    int total = 0;
    for (size_t i = 0; i < s.size(); i++) {
        if (i + 1 < s.size() && v[s[i]] < v[s[i + 1]]) total -= v[s[i]];
        else total += v[s[i]];
    }
    return total;
}

int main() {
    cout << romanToInt("XIV") << endl; // 14
    return 0;
}
