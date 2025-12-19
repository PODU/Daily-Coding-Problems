// Day 764: Arrange numbers to form the largest integer.
// Sort by comparator: a+b vs b+a (descending). Time: O(n log n * L), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string largestNumber(vector<int>& nums) {
    vector<string> s;
    for (int n : nums) s.push_back(to_string(n));
    sort(s.begin(), s.end(), [](const string& a, const string& b) {
        return a + b > b + a;
    });
    if (s[0] == "0") return "0";            // all zeros
    string out;
    for (auto& x : s) out += x;
    return out;
}

int main() {
    vector<int> nums = {10, 7, 76, 415};
    cout << largestNumber(nums) << "\n";    // 77641510
    return 0;
}
