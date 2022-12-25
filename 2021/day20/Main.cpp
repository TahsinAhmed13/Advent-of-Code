#include <bits/stdc++.h>

using namespace std; 

char getPixel(string key, deque<string> image, char edge, int i, int j)
{
    int b = 0; 
    for(int r = i-1; r <= i+1; ++r)
    {
        for(int c = j-1; c <= j+1; ++c)
        {
            bool inBounds = 0 <= r && r < image.size() &&
                0 <= c && c < image.size(); 
            if(inBounds)
                b += image[r][c] == '#'; 
            else 
                b += edge == '#'; 
            b *= 2; 
        }
    }
    b /= 2; 
    return key[b]; 
}

deque<string> step(string key, deque<string> image, int n)
{
    char edge = '.'; 
    while(n > 0)
    {
        string border; 
        for(int i = 0; i < image[0].size()+2; ++i)
            border += edge; 
        for(int i = 0; i < image.size(); ++i)
            image[i] = edge + image[i] + edge; 
        image.push_front(border); 
        image.push_back(border); 
      
        deque<string> copy = image; 
        for(int i = 0; i < image.size(); ++i)
            for(int j = 0; j < image[i].size(); ++j)
                image[i][j] = getPixel(key, copy, edge, i, j); 
         
        if(edge == '.')
            edge = key[0]; 
        else 
            edge = key[key.size()-1]; 
        n--; 
    }
    return image; 
}

int part1(string key, deque<string> image)
{
    image = step(key, image, 2); 
    int lights = 0; 
    for(string line : image)
    {
        for(char c : line)
            lights += c == '#'; 
    }
    return lights; 
}

int part2(string key, deque<string> image)
{
    image = step(key, image, 50); 
    int lights = 0; 
    for(string line : image)
    {
        for(char c : line)
            lights += c == '#'; 
    }
    return lights; 
}

int main()
{
    ifstream in ("in.txt"); 
    string key; 
    getline(in, key); 
    deque<string> image; 
    string line; 
    getline(in, line); 
    while(!in.eof())
    {
        getline(in, line); 
        image.push_back(line); 
    }
    cout << "Part 1: " << part1(key, image) << endl; 
    cout << "Part 2: " << part2(key, image) << endl; 
}
