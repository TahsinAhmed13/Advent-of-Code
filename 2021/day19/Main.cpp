#include <bits/stdc++.h>

using namespace std; 

vector<int> rot(const vector<int>& v, int i)
{
    int x = v[0]; 
    int y = v[1]; 
    int z = v[2]; 
    switch(i % 24)
    {
        case 0: 
            return vector<int>{x, y, z}; 
        case 1: 
            return vector<int>{x, -y, -z}; 
        case 2: 
            return vector<int>{x, -z, y}; 
        case 3: 
            return vector<int>{x, z, -y}; 
        case 4: 
            return vector<int>{z, x, y}; 
        case 5: 
            return vector<int>{y, x, -z}; 
        case 6: 
            return vector<int>{-z, x, -y}; 
        case 7: 
            return vector<int>{-y, x, z}; 
        case 8: 
            return vector<int>{-x, y, -z}; 
        case 9: 
            return vector<int>{-x, y, -z}; 
        case 10: 
            return vector<int>{-x, z, y}; 
        case 11: 
            return vector<int>{-x, -y, z}; 
        case 12: 
            return vector<int>{-z, -x, y}; 
        case 13: 
            return vector<int>{-y, -x, -z}; 
        case 14: 
            return vector<int>{z, -x, -y}; 
        case 15: 
            return vector<int>{y, -x, z}; 
        case 16: 
            return vector<int>{-y, -z, x}; 
        case 17: 
            return vector<int>{-z, y, x}; 
        case 18: 
            return vector<int>{y, z, x}; 
        case 19: 
            return vector<int>{z, -y, x}; 
        case 20: 
            return vector<int>{y, -z, -x}; 
        case 21: 
            return vector<int>{-z, -y, -x}; 
        case 22: 
            return vector<int>{-y, z, -x}; 
        case 23: 
            return vector<int>{z, y, -x}; 
        default: 
            return vector<int>{0, 0, 0}; 
    }
}

vector<int> overlap(const set<vector<int>>& a, const set<vector<int>>& b)
{
    for(vector<int> p : a)
    {
        for(vector<int> q : b)
        {
            for(int i = 0; i < 24; ++i)
            {
                vector<int> rq = rot(q, i); 
                int x = p[0] - rq[0]; 
                int y = p[1] - rq[1]; 
                int z = p[2] - rq[2]; 
                int matches = 0; 
                for(vector<int> r : b)
                {
                    vector<int> rr = rot(r, i); 
                    int tx = x + rr[0]; 
                    int ty = y + rr[1]; 
                    int tz = z + rr[2]; 
                    matches += a.count(vector<int>{tx, ty, tz}); 
                    if(matches >= 12)
                        return vector<int>{x, y, z, i}; 
                }
            }
        }
    }
    return vector<int>{0, 0, 0, 0}; 
}

int part1(vector<set<vector<int>>>& input, vector<vector<int>>& scanners) 
{
    vector<vector<vector<int>>> centers (input.size()); 
    for(int i = 0; i < centers.size(); ++i)
        centers[i].push_back(vector<int>{0, 0, 0}); 
    while(input.size() > 1)
    {
        for(int i = 0; i < input.size(); ++i)
        {
            int j = i+1; 
            while(j < input.size())
            {
                vector<int> t = overlap(input[i], input[j]); 
                if(t[0] != 0 && t[1] != 0 && t[2] != 0)
                {
                    for(vector<int> p : input[j])
                    {
                        vector<int> rp = rot(p, t[3]); 
                        int x = rp[0] + t[0]; 
                        int y = rp[1] + t[1]; 
                        int z = rp[2] + t[2]; 
                        input[i].insert(vector<int>{x, y, z}); 
                    }
                    for(vector<int> p : centers[j])
                    {
                        vector<int> rp = rot(p, t[3]); 
                        int x = rp[0] + t[0]; 
                        int y = rp[1] + t[1]; 
                        int z = rp[2] + t[2]; 
                        centers[i].push_back(vector<int>{x, y, z}); 
                    }
                    input.erase(input.begin()+j); 
                    centers.erase(centers.begin()+j); 
                    printf("MATCH: %d %d\n", i, j); 
                    printf("SIZE: %d\n", input.size()); 
                }
                else 
                    j++; 
            }
        }
    }
    scanners = centers[0]; 
    return input[0].size(); 
}

int dist(const vector<int>& a, const vector<int>& b)
{
    int dx = abs(a[0] - b[0]); 
    int dy = abs(a[1] - b[1]); 
    int dz = abs(a[2] - b[2]); 
    return dx + dy + dz; 
}

int part2(const vector<vector<int>>& scanners)
{
    int maxDist = 0; 
    for(int i = 0; i < scanners.size(); ++i)
        for(int j = i+1; j < scanners.size(); ++j)
            maxDist = max(maxDist, dist(scanners[i], scanners[j])); 
    return maxDist; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<set<vector<int>>> input (1); 
    string s; 
    getline(in, s); 
    while(!in.eof())
    {
        getline(in, s); 
        if(s.size() < 1)
        {
            getline(in, s); 
            input.push_back(set<vector<int>>());  
        }
        else
        {
            int x, y, z; 
            sscanf(s.c_str(), "%d,%d,%d", &x, &y, &z); 
            input[input.size()-1].insert(vector<int>{x, y, z}); 
        }
    }
    in.close(); 
    vector<vector<int>> scanners; 
    cout << "Part 1: " << part1(input, scanners) << endl; 
    cout << "Part 2: " << part2(scanners) << endl; 
}
