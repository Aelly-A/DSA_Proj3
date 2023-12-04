import requests
import sys
import json

def get_access_token(client_id, client_secret): #Gets authentication Token
    # OAuth 2.0 Client Credentials Flow
    token_url = "https://accounts.spotify.com/api/token"
    data = {"grant_type": "client_credentials"}
    auth = (client_id, client_secret)
    response = requests.post(token_url, data=data, auth=auth)

    if response.status_code == 200:
        access_token = response.json().get("access_token")
        return access_token
    else:
        print("Error:", response.status_code, response.text)
        return None


def search_spotify(song_name, artist, access_token): # Searches for song and prints out the link,album,artist, and artwork
    search_url = "https://api.spotify.com/v1/search"
    params = {"q": f"{song_name} {artist}", "type": "track", "limit": 1}
    headers = {"Authorization": f"Bearer {access_token}"}
    response = requests.get(search_url, params=params, headers=headers)

    if response.status_code == 200:
        data = response.json()
        if "tracks" in data and "items" in data["tracks"] and data["tracks"]["items"]:
            track_info = data["tracks"]["items"][0]
            spotify_link = track_info["external_urls"]["spotify"]
            album_name = track_info["album"]["name"]
            album_artists = [artist["name"] for artist in track_info["album"]["artists"]]
            album_image_url = track_info["album"]["images"][0]["url"] if "images" in track_info["album"] and \
                                                                         track_info["album"]["images"] else None

            print("Spotify Link:", spotify_link)
            print("Album:", album_name)
            print("Artists:", ", ".join(album_artists))

            if album_image_url:
                print("Album Artwork:", album_image_url)
            else:
                print("No Album Artwork available.")
        else:
            print("No results found on Spotify.")
    else:
        print("Error:", response.status_code, response.text)


if __name__ == "__main__":
    # Read input from standard input (JSON format)
    input_data = json.loads(sys.stdin.readline())

    # Extract song title and artist
    song_name = input_data.get("song_name", "")
    artist = input_data.get("artist", "")

    # Replace with your actual client ID and client secret
    CLIENT_ID = "82aa2a1cfab645bb860aa93edc7415d5"
    CLIENT_SECRET = "fa68a401d2bb4cf19996f7755a5a41d0"

    # Get access token
    access_token = get_access_token(CLIENT_ID, CLIENT_SECRET)

    if access_token:
        # Search Spotify
        search_spotify(song_name, artist, access_token)

