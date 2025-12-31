// Day 830: Largest number formed by concatenating the given numbers.
// Sort strings by comparator a+b > b+a (descending). O(N log N) compares of O(L) strings.
#include <bits/stdc++.h>
using namespace std;

string largestNumber(const vector<long long>& nums) {
    vector<string> strs;
    for (long long n : nums) strs.push_back(to_string(n));
    sort(strs.begin(), strs.end(), [](const string& a, const string& b) {
        return a + b > b + a;
    });
    string result;
    for (auto& s : strs) result += s;
    if (!result.empty() && result[0] == '0') return "0";
    return result;
}

int main() {
    cout << largestNumber({10, 7, 76, 415}) << "\n";  // 77641510
    return 0;
}
