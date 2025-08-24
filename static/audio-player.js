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

function setVisualizerPlaying(isPlaying) {
  const bars = document.querySelectorAll(".bar");
  bars.forEach(bar => {
    bar.style.animationPlayState = isPlaying ? "running" : "paused";
  });
}

function initAudioPlayer() {
    const trackInfoElement = document.getElementById("time-info");
    if (!trackInfoElement) return;

    let progressMs = Number(trackInfoElement.dataset.progress);
    let durationMs = Number(trackInfoElement.dataset.duration);
    
    let interval;
    if (progressMs + durationMs < 0) {
      interval = 30000;
      setVisualizerPlaying(false);
    } else {
      interval = 1000;
      setVisualizerPlaying(true);
    }

    const progressElement = document.getElementById("progress-display");
    const durationElement = document.getElementById("duration-display");
    const progressBarElement = document.getElementById("progress");

    progressElement.textContent = msToTimeString(progressMs);
    durationElement.textContent = msToTimeString(durationMs);
    progressBarElement.style.width = `${(progressMs / durationMs) * 100}%`;

    const intervalId = setInterval(() => {
        if (progressMs < durationMs) {
            progressMs += 1000;
            progressElement.textContent = msToTimeString(progressMs);
            progressBarElement.style.width = `${(progressMs / durationMs) * 100}%`;
        } else {
            clearInterval(intervalId);
            reloadAudioPlayerWidget();
        }
    }, interval);
}


