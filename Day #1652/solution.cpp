// Shortest palindrome by insertions, lexicographically earliest: memoized DP on
// (i,j) building best palindrome for s[i..j]. Time O(n^2) states, Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

string s;
vector<vector<string>> memo;
vector<vector<bool>> done;

string solve(int i, int j) {
    if (i > j) return "";
    if (i == j) return string(1, s[i]);
    if (done[i][j]) return memo[i][j];
    string res;
    if (s[i] == s[j]) {
        res = s[i] + solve(i + 1, j - 1) + s[i];
    } else {
        string opt1 = s[i] + solve(i + 1, j) + s[i];
        string opt2 = s[j] + solve(i, j - 1) + s[j];
        if (opt1.size() < opt2.size()) res = opt1;
        else if (opt2.size() < opt1.size()) res = opt2;
        else res = (opt1 < opt2) ? opt1 : opt2;
    }
    done[i][j] = true;
    return memo[i][j] = res;
}

string shortestPalindrome(const string& input) {
    s = input;
    int n = s.size();
    memo.assign(n, vector<string>(n));
    done.assign(n, vector<bool>(n, false));
    return solve(0, n - 1);
}

int main() {
    cout << shortestPalindrome("race") << endl;
    cout << shortestPalindrome("google") << endl;
    return 0;
}
