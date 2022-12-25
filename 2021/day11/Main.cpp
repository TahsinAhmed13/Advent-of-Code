#include <bits/stdc++.h>

using namespace std; 

string digits = "0123456789"; 
vector<vector<int>> energy; 

void spread(int r, int c)
{
    if(energy[r][c] != 10)
        return; 
    for(int i = r-1; i <= r+1; ++i)
    {
        for(int j = c-1; j <= c+1; ++j)
        {
            bool self = (i == r) && (j == c); 
            bool valid = (0 <= i) && (i < energy.size()) && 
                (0 <= j) && (j < energy[i].size()); 
            if(!self && valid)
            {
                energy[i][j]++; 
                spread(i, j); 
            }
        }
    }
}

int main()
{
    ifstream in ("in.txt"); 
    while(!in.eof())
    {
        string line; 
        getline(in, line); 
        vector<int> row (line.size()); 
        for(int i = 0; i < line.size(); ++i)
            row[i] = digits.find(line[i]); 
        energy.push_back(row); 
    }
    // int f = 0; 
    // for(int n = 0; n < 100; ++n)
    int n = 1; 
    while(true) 
    {
        int f = 0; 
        for(int i = 0; i < energy.size(); ++i)
        {
            for(int j = 0; j < energy[i].size(); ++j)
            {
                energy[i][j]++; 
                spread(i, j); 
            }
        }
        for(int i = 0; i < energy.size(); ++i)
        {
            for(int j = 0; j < energy[i].size(); ++j)
            {
                if(energy[i][j] > 9)
                {
                    f++; 
                    energy[i][j] = 0; 
                }
                cout << energy[i][j]; 
            }
            cout << endl; 
        }
        if(f == energy.size() * energy[0].size())
            break; 
        n++; 
        cout << endl; 
    }
    // cout << f << endl; 
    cout << n << endl; 
}
