//
// Created by Aelly Alwardi on 11/30/23.
//

#include "KDimTree.h"
Node::Node(const std::string &name, const std::string &id, const vector<std::string> &artists, float valence, int year,
           float acousticness, float danceablity, int duration, float energy, bool adult, float instrumentalness)
{
    this->name = name;
    this->id = id;
    this->artists = artists;
    this->year = year;
    this->duration = duration;
    this->adult = adult;
    this->misc_data["valence"] = valence;
    this->misc_data["acousticness"] = acousticness;
    this->misc_data["danceablity"] = danceablity;
    this->misc_data["energy"] = energy;
    this->misc_data["instrumentalness"] = instrumentalness;
    this->left = nullptr;
    this->right = nullptr;
}
Node* KDimTree::insertSong(const string& name, const string& id, const vector<string>& artists, float valence, int year,
                 float acousticness, float danceablity, int duration, float energy, bool adult, float instrumentalness)

{
    Node* new_song = new Node(name, id, artists, valence, year, acousticness, danceablity,
                         duration, energy, adult, instrumentalness);

    root = helperInsert(root, new_song, 0);

    return root;
}

// Super basic insert function
Node* KDimTree::helperInsert(Node* head, Node* song, int depth)
{
    if(head == nullptr)
        return song;

    // This gets the current dimension
    int current_dim = depth % k;

    // Then we decide what data we want to compare based off of what dimension we are in
    string comp = dim_to_comp[current_dim];

    if (head->misc_data[comp] > song->misc_data[comp])
    {
        head->left = helperInsert(head->left, song, depth + 1);
    }
    else
    {
        head->right = helperInsert(head->right, song, depth + 1);
    }

    return head;
}


// Calls a post order traversal to destruct
KDimTree::~KDimTree()
{
    helperDestructor(root);
}

void KDimTree::helperDestructor(Node *head)
{
    if (head == nullptr)
        return;

    helperDestructor(head->left);
    helperDestructor(head->right);
    delete head;
}
