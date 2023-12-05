#include <iostream>
#include <cstdio>
#include <cstdlib>
#include <cstring>
//This code takes in the song title and artist and inputs them into the python code which links with spotify and prints out the associated song link and image.
int main(){
    // Replace with the actual song title and artist
    const char* song_name = "Despacito";
    const char* artist = "Luis Fonsi";
    // Construct the JSON input
    std::string json_input = R"({"song_name": ")" + std::string(song_name) + R"(", "artist": ")" + std::string(artist) + R"("})";

    // Use popen to run the Python script as a subprocess
    FILE* pipe = popen("python3  C:\\Users\\Anthony\\PycharmProjects\\SpotifyEngine\\main.py", "w");
    if (pipe) {
        // Write the JSON input to the subprocess
        fprintf(pipe, "%s\n", json_input.c_str());
        pclose(pipe);
    } else {
        std::cerr << "Error launching Python script." << std::endl;
    }

    return 0;
}
