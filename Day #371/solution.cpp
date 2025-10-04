// Day 371: Solve a system of addition-only equations over variables/constants.
// Build a linear system A x = b and run Gauss-Jordan elimination; unique integer
// solution -> mapping, otherwise null. Time O(eqs * vars^2).
#include <bits/stdc++.h>
using namespace std;

static string trim(const string& s) {
    size_t a = s.find_first_not_of(" \t");
    if (a == string::npos) return "";
    size_t b = s.find_last_not_of(" \t");
    return s.substr(a, b - a + 1);
}

static vector<string> split(const string& s, char d) {
    vector<string> out; string cur;
    for (char c : s) { if (c == d) { out.push_back(cur); cur.clear(); } else cur += c; }
    out.push_back(cur);
    return out;
}

static bool isNumber(const string& t) {
    if (t.empty()) return false;
    size_t i = (t[0] == '-') ? 1 : 0;
    if (i == t.size()) return false;
    for (; i < t.size(); i++) if (!isdigit((unsigned char)t[i])) return false;
    return true;
}

string solve(const string& text) {
    vector<pair<map<string,double>,double>> eqs;
    set<string> varset;
    for (auto& rawLine : split(text, '\n')) {
        string line = trim(rawLine);
        if (line.empty()) continue;
        auto sides = split(line, '=');
        map<string,double> coeffs; double b = 0;
        for (auto& tok : split(sides[0], '+')) {
            string t = trim(tok);
            if (isNumber(t)) b -= stol(t);
            else { coeffs[t] += 1; varset.insert(t); }
        }
        for (auto& tok : split(sides[1], '+')) {
            string t = trim(tok);
            if (isNumber(t)) b += stol(t);
            else { coeffs[t] -= 1; varset.insert(t); }
        }
        eqs.push_back({coeffs, b});
    }

    vector<string> vars(varset.begin(), varset.end());
    map<string,int> idx;
    for (int i = 0; i < (int)vars.size(); i++) idx[vars[i]] = i;
    int n = vars.size();
    vector<vector<double>> aug;
    for (auto& e : eqs) {
        vector<double> row(n + 1, 0.0);
        for (auto& kv : e.first) row[idx[kv.first]] += kv.second;
        row[n] = e.second;
        aug.push_back(row);
    }

    int m = aug.size(), pr = 0;
    vector<int> pivotCols;
    for (int col = 0; col < n; col++) {
        int sel = -1;
        for (int r = pr; r < m; r++) if (fabs(aug[r][col]) > 1e-9) { sel = r; break; }
        if (sel == -1) continue;
        swap(aug[pr], aug[sel]);
        double pv = aug[pr][col];
        for (double& x : aug[pr]) x /= pv;
        for (int r = 0; r < m; r++) if (r != pr && fabs(aug[r][col]) > 1e-9) {
            double f = aug[r][col];
            for (int c = 0; c <= n; c++) aug[r][c] -= f * aug[pr][c];
        }
        pivotCols.push_back(col);
        pr++;
    }
    for (int r = 0; r < m; r++) {
        bool allZero = true;
        for (int c = 0; c < n; c++) if (fabs(aug[r][c]) > 1e-9) allZero = false;
        if (allZero && fabs(aug[r][n]) > 1e-9) return "null";
    }
    if ((int)pivotCols.size() < n) return "null";

    map<string,long> sol;
    for (int i = 0; i < (int)pivotCols.size(); i++)
        sol[vars[pivotCols[i]]] = llround(aug[i][n]);
    string out = "{\n";
    bool first = true;
    for (auto& kv : sol) {
        if (!first) out += ",\n";
        out += "  " + kv.first + ": " + to_string(kv.second);
        first = false;
    }
    out += "\n}";
    return out;
}

int main() {
    string text = "y = x + 1\n5 = x + 3\n10 = z + y + 2";
    cout << solve(text) << "\n";
    return 0;
}
