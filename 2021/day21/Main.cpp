#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

int part1(int p, int q)
{
    int a, b; 
    a = b = 0; 
    int t = 1; 
    while(t <= 997)
    {
        if(t % 2 == 1)
        {
            p += 3 * (t + 1); 
            p = (p - 1) % 10 + 1; 
            a += p; 
        }
        else
        {
            q += 3 * (t + 1); 
            q = (q - 1) % 10 + 1; 
            b += q; 
        }
        t += 3; 
        if(a >= 1000 || b >= 1000)
            break; 
    }
    return (t-1) * min(a, b); 
}

ll a, b; 
vector<ll> weights = {1, 3, 6, 7, 6, 3, 1}; 
void step1(int p1, int p2, int s1, int s2, ll w); 
void step2(int p1, int p2, int s1, int s2, ll w); 

void step1(int p1, int p2, int s1, int s2, ll w)
{
    if(s2 >= 21)
    {
        b += w; 
        return; 
    }
    int tmp = p1; 
    for(int i = 3; i <= 9; ++i)
    {
        p1 = ((tmp + i) - 1) % 10 + 1; 
        step2(p1, p2, s1 + p1, s2, w * weights[i-3]); 
    }
}

void step2(int p1, int p2, int s1, int s2, ll w)
{
    if(s1 >= 21)
    {
        a += w; 
        return; 
    }
    int tmp = p2; 
    for(int i = 3; i <= 9; ++i)
    {
        p2 = ((tmp + i) - 1) % 10 + 1; 
        step1(p1, p2, s1, s2 + p2, w * weights[i-3]); 
    }
}

ll part2(int p, int q)
{
    a = b = 0; 
    step1(p, q, 0, 0, 1); 
    return max(a, b); 
}

int main()
{
    ifstream in ("in.txt"); 
    int p, q; 
    string s; 
    getline(in, s); 
    sscanf(s.c_str(), "Player 1 starting position: %d", &p); 
    getline(in, s); 
    sscanf(s.c_str(), "Player 2 starting position: %d", &q); 
    in.close(); 
    cout << "Part 1: " << part1(p, q) << endl; 
    cout << "Part 2: " << part2(p, q) << endl; 
}
