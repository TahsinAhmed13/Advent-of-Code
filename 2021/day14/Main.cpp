#include <bits/stdc++.h>

#define STEPS 40

using namespace std; 
typedef long long ll; 

int main()
{
    ifstream in ("in.txt"); 
    string s; 
    in >> s; 
    map<string, string> rules; 
    while(!in.eof())
    {
        string a, b, foo; 
        in >> a >> foo >> b; 
        cout << a << " " << b << endl; 
        rules[a] = b; 
    }

    map<string, ll> pairs; 
    for(int i = 0; i < s.size()-1; ++i)
    {
        string p = s.substr(i, 2); 
        if(pairs.count(p))
            pairs[p]++; 
        else 
            pairs[p] = 1; 
    }
    for(int i = 0; i < STEPS; ++i)
    {
        map<string, ll> next; 
        for(auto it : pairs)
        {
            string m = rules[it.first]; 
            string a = it.first[0] + m; 
            string b = m + it.first[1]; 
            if(next.count(a))
                next[a] += it.second; 
            else
                next[a] = it.second; 
            if(next.count(b))
                next[b] += it.second; 
            else
                next[b] = it.second; 
        }
        pairs = next; 
    }
    for(auto it : pairs)
        cout << it.first << " : " << it.second << endl; 

    map<char, ll> f; 
    for(auto it : pairs)
    {
        if(f.count(it.first[0]))
            f[it.first[0]] += it.second; 
        else 
            f[it.first[0]] = it.second; 
    }
    if(f.count(s[s.size()-1]))
        f[s[s.size()-1]]++; 
    else 
        f[s[s.size()-1]] = 1; 
    ll h, l; 
    h = l = f.begin()->second; 
    for(auto it : f)
    {
        h = max(h, it.second); 
        l = min(l, it.second); 
    }
    cout << (h - l) << endl; 
}
