function msToTimeString(s) {
  var ms = s % 1000;
  s = (s - ms) / 1000;
  var secs = s % 60;
  s = (s - secs) / 60;
  var mins = s % 60

  var paddedSecs = secs.toString().padStart(2, '0');
  var paddedMins = mins.toString().padStart(2, '0');

  return paddedMins + ':' + paddedSecs;
}

const trackInfoElement = document.getElementById("time-info");
let progressMs = Number(trackInfoElement.dataset.progress);
let durationMs = Number(trackInfoElement.dataset.duration);

const progressElement = document.getElementById("progress-display");
const durationElement = document.getElementById("duration-display");
const progressBarElement = document.getElementById("progress");

progressElement.textContent = msToTimeString(progressMs);
durationElement.textContent = msToTimeString(durationMs);
progressBarElement.style.width = `${(progressMs / durationMs) * 100}%`;

setInterval(() => {
  if (progressMs < durationMs) {
      progressMs += 1000;
      progressElement.textContent = msToTimeString(progressMs);
      progressBarElement.style.width = `${(progressMs / durationMs) * 100}%`;
    }
}, 1000);
