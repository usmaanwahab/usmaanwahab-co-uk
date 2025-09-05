function fetchMatchHistory() {
  fetch("/league/match-history")
    .then(r => r.text())
    .then(html => document.getElementById("match-history").innerHTML = html);
}

document.addEventListener("DOMContentLoaded", () => {
  fetchMatchHistory();
});

