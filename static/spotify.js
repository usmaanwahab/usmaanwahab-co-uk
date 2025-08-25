function fetchTopArtists() {
  fetch("/spotify/top/artists/long_term?limit=50&offset=0")
    .then(r => r.text())
    .then(html => document.getElementById("spotify-content").innerHTML = html)
}

function fetchTopTracks() {
  fetch("/spotify/top/tracks/long_term?limit=50&offset=0")
    .then(r => r.text())
    .then(html => document.getElementById("spotify-content").innerHTML = html)
}
