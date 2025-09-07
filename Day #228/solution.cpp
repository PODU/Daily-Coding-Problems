// Largest number: sort by comparator a+b > b+a (concatenation), then join.
// Time: O(n log n * L), Space: O(n * L).
#include <bits/stdc++.h>
using namespace std;

string largestNumber(vector<int> nums) {
    vector<string> s;
    for (int n : nums) s.push_back(to_string(n));
    sort(s.begin(), s.end(), [](const string& a, const string& b) {
        return a + b > b + a;
    });
    if (s[0] == "0") return "0";
    string res;
    for (auto& x : s) res += x;
    return res;
}

int main() {
    vector<int> nums = {10, 7, 76, 415};
    cout << largestNumber(nums) << "\n"; // 77641510
}
