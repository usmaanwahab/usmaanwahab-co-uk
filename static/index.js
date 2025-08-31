function reloadAudioPlayerWidget() {
  fetch("/spotify/currently-playing")
    .then(r => r.text())
    .then(html => {
      document.getElementById("audio-player-container").innerHTML = html;
      initAudioPlayer();
    });
}

function fetchRankedStats() {
  fetch("/league")
    .then(r => r.text())
    .then(html => {
      document.getElementById("league-container").innerHTML = html;
    });
}

document.addEventListener("DOMContentLoaded", () => {
  reloadAudioPlayerWidget();
  fetchRankedStats();
});
