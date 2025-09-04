function fetchTopArtists() {
  fetch("/spotify/top/artists/long_term?limit=50&offset=0")
    .then(r => r.text())
    .then(html => document.getElementById("top-artists").innerHTML = html)
}

function fetchTopTracks() {
  fetch("/spotify/top/tracks/long_term?limit=50&offset=0")
    .then(r => r.text())
    .then(html => document.getElementById("top-tracks").innerHTML = html)
}

function reloadAudioPlayerWidget() {
  fetch("/spotify/currently-playing")
    .then(r => r.text())
    .then(html => {
      document.getElementById("audio-player-container").innerHTML = html;
      initAudioPlayer();
    });
}

document.addEventListener("DOMContentLoaded", () => {
  fetchTopTracks();
  fetchTopArtists();
  reloadAudioPlayerWidget()
})
