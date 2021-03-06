import { useCallback, useContext, useEffect, useRef, useState } from "react";
import { VideoPlayerContext } from "../Context";

import "./SeekBar.scss";

function VideoControls() {
  const { setCurrentTime, player, duration, currentTime, setOffset, buffer } = useContext(VideoPlayerContext);
  const [ seeking, setSeeking ] = useState(false);

  const seekBarCurrent = useRef(null);
  const bufferBar = useRef(null);

  // current time
  useEffect(() => {
    const position = (currentTime / duration) * 100;
    seekBarCurrent.current.style.width = `${position}%`;
  }, [currentTime, duration])

  // buffer
  useEffect(() => {
    const position = ((currentTime + buffer) / duration) * 100;
    bufferBar.current.style.width = `${position}%`;
  }, [buffer, currentTime, duration])

  const onSeek = useCallback(async (e) => {
    if (seeking) return;

    setSeeking(true);

    const rect = e.target.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    const videoDuration = player.duration();
    const newTime = Math.floor(percent * videoDuration);
    const newSegment = Math.floor(newTime / 5);

    player.attachSource(`http://localhost:8000/manifest.mpd?start_num=${newSegment}`);

    // setOldOffset(offset);
    setOffset(newTime);
    setCurrentTime(0);
    setSeeking(false);
  }, [player, seeking, setCurrentTime, setOffset]);

  return (
    <div className="seekBar" onClick={onSeek}>
      <div ref={bufferBar} className="buffer"/>
      <div ref={seekBarCurrent} className="current"/>
    </div>
  );
}

export default VideoControls;
