#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

map<string, vector<string>> caves; 
map<string, int> visit; 
bool twice; 

bool small(string c)
{
    return 'a' <= c[0] && c[0] <= 'z'; 
}

ll traverse(string c)
{
    if(!c.compare("end"))
        return 1; 
    if(small(c) && visit[c] > 0)
    {
        if(c.compare("start") && !twice)
            twice = true; 
        else
            return 0; 
    }
    cout << "Cave: " << c << endl; 
    ll paths = 0; 
    visit[c]++; 
    for(string n : caves[c])
        paths += traverse(n);
    if(small(c) && visit[c] > 1)
        twice = false; 
    visit[c]--; 
    return paths; 
}

int main()
{
    ifstream in ("in.txt"); 
    while(!in.eof())
    {
        string line;
        getline(in, line); 
        int i = line.find('-'); 
        string first = line.substr(0, i); 
        string second = line.substr(i+1); 
        if(caves.find(first) != caves.end())
            caves[first].push_back(second); 
        else
        {
            vector<string> to = {second}; 
            caves[first] = to;
        }
        if(caves.find(second) != caves.end()) 
            caves[second].push_back(first); 
        else
        {
            vector<string> to = {first}; 
            caves[second] = to;
        }
        visit[first] = 0; 
        visit[second] = 0; 
    }
    twice = false; 
    for(auto it = caves.begin(); it != caves.end(); ++it)
    {
        cout << it->first << endl; 
        for(string c : it->second)
            cout << c << " "; 
        cout << endl; 
    }
    cout << traverse("start") << endl; 
}
