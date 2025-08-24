function reloadAudioPlayerWidget() {
  fetch("/spotify/currently-playing")
    .then(r => r.text())
    .then(html => {
      document.getElementById("audio-player-container").innerHTML = html;
      initAudioPlayer();
    });
}

document.addEventListener("DOMContentLoaded", () => {
  reloadAudioPlayerWidget();
});
