// Day 509: Fewest-insertion palindrome with lexicographically earliest result.
// Memoized interval DP building the actual string. Time O(n^3), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

vector<vector<bool>> done;
vector<vector<string>> memo;
string s;

string build(int i, int j) {
    if (i > j) return "";
    if (i == j) return string(1, s[i]);
    if (done[i][j]) return memo[i][j];
    done[i][j] = true;
    string res;
    if (s[i] == s[j]) {
        res = s[i] + build(i + 1, j - 1) + s[j];
    } else {
        string a = s[i] + build(i + 1, j) + s[i]; // insert s[i] at end
        string b = s[j] + build(i, j - 1) + s[j]; // insert s[j] at front
        if (a.size() < b.size()) res = a;
        else if (b.size() < a.size()) res = b;
        else res = (a < b) ? a : b;
    }
    return memo[i][j] = res;
}

string solve(const string& in) {
    s = in;
    int n = s.size();
    done.assign(n, vector<bool>(n, false));
    memo.assign(n, vector<string>(n, ""));
    if (n == 0) return "";
    return build(0, n - 1);
}

int main() {
    cout << solve("race") << "\n";
    cout << solve("google") << "\n";
    return 0;
}
