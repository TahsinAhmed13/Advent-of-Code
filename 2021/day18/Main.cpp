#include <bits/stdc++.h>

using namespace std; 

string digits = "0123456789"; 

bool isNum(char c)
{
    return digits.find(c) != string::npos; 
}

vector<pair<int, int>> fromString(string s)
{
    vector<pair<int, int>> v; 
    int d = 0; 
    for(int i = 0; i < s.size(); ++i)
    {
        d += s[i] == '['; 
        d -= s[i] == ']'; 
        string nstr; 
        while(isNum(s[i]))
        {
            nstr += s[i]; 
            i++; 
        }
        if(nstr.size() > 0)
        {
            v.push_back(pair<int, int>(stoi(nstr), d)); 
            i--; 
        }
    }
    return v; 
}

void printNum(vector<pair<int, int>> v)
{
    for(auto p : v)
        printf("(%d, %d) ", p.first, p.second); 
    cout << endl; 
}

vector<pair<int, int>> add(vector<pair<int, int>> a, vector<pair<int, int>> b)
{
    vector<pair<int, int>> sum; 
    for(int i = 0; i < a.size(); ++i)
        sum.push_back(pair<int, int>(a[i].first, a[i].second+1)); 
    for(int i = 0; i < b.size(); ++i)
        sum.push_back(pair<int, int>(b[i].first, b[i].second+1)); 
    return sum; 
}

vector<pair<int, int>> explode(vector<pair<int, int>> v)
{
    for(int i = 0; i < v.size()-1; ++i)
    {
        if(v[i].second >= 5 && v[i].second == v[i+1].second)
        {
            if(i > 0)
                v[i-1].first += v[i].first; 
            if(i < v.size()-2)
                v[i+2].first += v[i+1].first; 
            int d = v[i].second; 
            v.erase(v.begin()+i); 
            v.erase(v.begin()+i); 
            v.insert(v.begin()+i, pair<int, int>(0, d-1)); 
        }
    }
    return v; 
}

vector<pair<int, int>> split(vector<pair<int, int>> v)
{
    int i = 0; 
    while(i < v.size())
    {
        if(v[i].first > 9)
        {
            int d = v[i].second; 
            int l = v[i].first / 2; 
            int r = (v[i].first + 1) / 2; 
            v.erase(v.begin()+i); 
            v.insert(v.begin()+i, pair<int, int>(l, d+1)); 
            v.insert(v.begin()+i+1, pair<int, int>(r, d+1)); 
            v = explode(v); 
            i = 0; 
        }
        else 
            i++; 
    }
    return v; 
}

vector<pair<int, int>> reduce(vector<pair<int, int>> v)
{
    return split(explode(v)); 
}

int mag(vector<pair<int, int>> v)
{
    if(v.size() == 0)
        return 0; 
    if(v.size() == 1)
        return v[0].first; 
    for(int i = 0; i < v.size()-1; ++i)
    {
        if(v[i].second == v[i+1].second)
        {
            int m = 3 * v[i].first + 2 * v[i+1].first; 
            int d = v[i].second - 1; 
            v.erase(v.begin()+i); 
            v.erase(v.begin()+i); 
            v.insert(v.begin()+i, pair<int, int>(m, d)); 
            break; 
        }
    }
    for(int i = v.size()-1; i > 0; --i)
    {
        if(v[i-1].second == v[i].second)
        {
            int m = 3 * v[i-1].first + 2 * v[i].first; 
            int d = v[i-1].second - 1; 
            v.erase(v.begin()+i-1); 
            v.erase(v.begin()+i-1); 
            v.insert(v.begin()+i-1, pair<int, int>(m, d)); 
            break; 
        }
    }
    return mag(v); 
}

int part1(vector<string> input)
{
    auto n = fromString(input[0]); 
    for(int i = 1; i < input.size(); ++i)
        n = reduce(add(n, fromString(input[i]))); 
    return mag(n); 
}

int part2(vector<string> input)
{
    int m = 0; 
    for(int i = 0; i < input.size(); ++i)
    {
        auto x = fromString(input[i]); 
        for(int j = i+1; j < input.size(); ++j)
        {
            auto y = fromString(input[j]); 
            int a = mag(reduce(add(x, y))); 
            int b = mag(reduce(add(y, x))); 
            m = max(m, max(a, b)); 
        }
    }
    return m; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<string> input; 
    while(!in.eof())
    {
        string s; 
        getline(in, s); 
        input.push_back(s); 
    }
    cout << "Part 1: " << part1(input) << endl; 
    cout << "Part 2: " << part2(input) << endl; 
}
