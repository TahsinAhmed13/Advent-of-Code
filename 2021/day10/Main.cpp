#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

string open = "([{<"; 
string close = ")]}>"; 
vector<int> points = {3, 57, 1197, 25137};

int main()
{
    ifstream in ("in.txt"); 
    int p = 0; 
    vector<ll> s; 
    while(!in.eof())
    {
        string line; 
        getline(in, line); 
        stack<char> chuncks; 
        ll t = 0; 
        bool w = false; 
        for(int i = 0; i < line.size(); ++i)
        {
            if(open.find(line[i]) != string::npos)
                chuncks.push(line[i]); 
            else if(open[close.find(line[i])] == chuncks.top())
                chuncks.pop(); 
            else 
            {
                p += points[close.find(line[i])]; 
                w = true; 
                break; 
            }
        }
        if(w)
            continue; 
        while(!chuncks.empty())
        {
            t = t * 5 + open.find(chuncks.top()) + 1;   
            chuncks.pop(); 
        }
        s.push_back(t); 
    }
    sort(s.begin(), s.end()); 
    cout << "Part 1: " << p << endl; 
    cout << "Part 2: " << s[s.size()/2] << endl; 
}
