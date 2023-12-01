//
// Created by Aelly Alwardi on 11/30/23.
//

#include <iostream>
#include <vector>
#include <map>

#ifndef DSA_K_DIM_TREE_H
#define DSA_K_DIM_TREE_H

using namespace std;

// Debating whether to put all this stuff public
// But Node is essentially just a song
struct Node
{
private:
    string id;
    string name;
    vector<string> artists;
    int year;
    bool adult;
    int duration;

public:
    Node *left, *right;
    // The songs have some weird data like 'instrumentalness', so I threw all that stuff in the map
    map<string, float> misc_data;

    Node(const string& name, const string& id, const vector<string>& artists, float valence, int year,
    float acousticness, float danceablity, int duration, float energy, bool adult,
    float instrumentalness);
};

class KDimTree
{
    Node* root;
    int k = 6;
    // This map is so we can easily know which value to compare at which dimension. e.g. dimension 0 compares energy
    // Still thinking about which stuff should be compared, will probably discuss with team
    map<int, string> dim_to_comp{
        {0, "energy"},
        {1, "danceablity"},
        {2, "valence"},
        {3, "year"},
        {4, "acousticness"},
        {5, "instrumentalness"},
    };

public:
    KDimTree();
    ~KDimTree();
    // Super ugly function, but hey the data has to go somewhere. This could be pulled out to main or wherever
    // and a node could be declared there and passed in here (sorta like helperInsert)
    Node* insertSong(const string& name, const string& id, const vector<string>& artists, float valence, int year,
                     float acousticness, float danceablity, int duration, float energy, bool adult, float instrumentalness);
    Node* helperInsert(Node* head, Node* song, int depth);
    void helperDestructor(Node* head);

};


#endif //DSA_K_DIM_TREE_H
