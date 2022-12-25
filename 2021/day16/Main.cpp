#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

map<char, string> hexMap = {
    {'0', "0000"}, 
    {'1', "0001"}, 
    {'2', "0010"}, 
    {'3', "0011"}, 
    {'4', "0100"}, 
    {'5', "0101"}, 
    {'6', "0110"}, 
    {'7', "0111"}, 
    {'8', "1000"}, 
    {'9', "1001"}, 
    {'A', "1010"}, 
    {'B', "1011"}, 
    {'C', "1100"}, 
    {'D', "1101"}, 
    {'E', "1110"}, 
    {'F', "1111"}, 
};

ll bin(string s)
{
    ll b = 0; 
    for(char c : s)
    {
        if(c == '1')
            b += 1; 
        b *= 2; 
    }
    return b / 2; 
}

ll op(int t, ll a, ll b)
{
    switch(t)
    {
        case 0: 
            return a + b; 
        case 1: 
            return a * b; 
        case 2: 
            return min(a, b); 
        case 3: 
            return max(a, b); 
        case 5: 
            return a > b; 
        case 6: 
            return a < b; 
        case 7: 
            return a == b; 
        default: 
            return -1; 
    }
}

pair<ll, int> eval(string p)
{
    ll v = bin(p.substr(0, 3)); 
    ll t = bin(p.substr(3, 3)); 
    ll id = bin(p.substr(6, 1)); 
    ll out; 
    int i; 
    switch(t)
    {
        case 1: 
            out = 1; 
            break; 
        case 2:
            out = numeric_limits<ll>::max(); 
            break; 
        case 5: 
            out = -1; 
            break; 
        case 6: 
            out = -1; 
            break; 
        case 7: 
            out = -1; 
            break; 
        default: 
            out = 0; 
            break; 
    }
    if(t == 4)
    {
        i = 6; 
        while(i < p.size() && p[i] == '1')
        {
            out += bin(p.substr(i+1, 4)); 
            out *= 16; 
            i += 5; 
        }
        out += bin(p.substr(i+1, 4)); 
        i += 5; 
    }
    else if(id == 0)
    {
        ll s = bin(p.substr(7, 15));         
        i = 22; 
        while(i - 22 < s)
        {
            pair<ll, int> pack = eval(p.substr(i));  
//            v += pack.first; 
            if(out == -1 && (t == 5 || t == 6 || t == 7))
                out = pack.first; 
            else
                out = op(t, out, pack.first); 
            i += pack.second; 
        }
    }
    else 
    {
        ll n = bin(p.substr(7, 11)); 
        i = 18; 
        while(n > 0)
        {
            pair<ll, int> pack = eval(p.substr(i)); 
//            v += pack.first; 
            if(out == -1 && (t == 5 || t == 6 || t == 7))
                out = pack.first; 
            else
                out = op(t, out, pack.first); 
            i += pack.second; 
            n--; 
        }
    }
//    return pair<int, int>(v, i); 
    return pair<ll, int>(out, i);    
}

int main()
{
    ifstream in ("in.txt"); 
    string p; 
    while(!in.eof())
    {
        char c; 
        in >> c; 
        p += hexMap[c]; 
    }
    ll total = eval(p).first; 
    cout << total << endl; 
}
