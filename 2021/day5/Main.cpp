#include <bits/stdc++.h>

using namespace std; 

int safe(vector<vector<int>> l)
{
    vector<vector<int>> g (1000); 
    for(int i = 0; i < 1000; ++i)
        g[i].resize(1000); 
    int s = 0; 
    for(vector<int> p : l)
    {
        int a = p[0]; 
        int b = p[1]; 
        int c = p[2]; 
        int d = p[3]; 
        if(a == c)
        {
            for(int j = min(b, d); j <= max(b, d); ++j)
            {
                g[a][j]++; 
                if(g[a][j] == 2)
                    s++; 
            }
        }
        else if(b == d)
        {
            for(int j = min(a, c); j <= max(a, c); ++j)
            {
                g[j][b]++; 
                if(g[j][b] == 2)
                    s++; 
            }
        }
        else
        {
            for(int j = 0; j <= abs(c-a); ++j)
            {
                int x, y; 
                x = min(a, c) + j; 
                if(a < c)
                    y = b < d ? b + j : b - j; 
                else 
                    y = d < b ? d + j : d - j; 
                g[x][y]++; 
                if(g[x][y] == 2)
                    s++; 
            }
        }
    }
    return s; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<vector<int>> l; 
    while(!in.eof())
    {
        string s; 
        getline(in, s); 
        int a, b, c, d; 
        sscanf(s.c_str(), "%d,%d -> %d,%d", &a, &b, &c, &d); 
        vector<int> p = {a, b, c, d}; 
        l.push_back(p); 
    }
    cout << safe(l) << endl;  
}
