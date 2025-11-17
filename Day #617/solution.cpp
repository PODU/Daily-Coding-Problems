// Roman numeral -> decimal. Single left-to-right pass; subtract when a smaller
// value precedes a larger one. Time O(n), Space O(1).
#include <iostream>
#include <string>
#include <unordered_map>
using namespace std;

int romanToInt(const string& s) {
    unordered_map<char, int> val = {{'M',1000},{'D',500},{'C',100},
                                    {'L',50},{'X',10},{'V',5},{'I',1}};
    int total = 0;
    for (size_t i = 0; i < s.size(); ++i) {
        int v = val[s[i]];
        if (i + 1 < s.size() && val[s[i+1]] > v) total -= v;
        else total += v;
    }
    return total;
}

int main() {
    cout << romanToInt("XIV") << endl; // 14
    return 0;
}
