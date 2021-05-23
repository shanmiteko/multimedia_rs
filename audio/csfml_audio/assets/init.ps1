# Input #0, flv, from 'live.flv':
#   Metadata:
#     Rawdata         :
#     displayWidth    : 1280
#     displayHeight   : 720
#     fps             : 30
#     profile         :
#     level           :
#     encoder         : Lavf58.29.100
#   Duration: 00:00:00.00, start: 365073.018000, bitrate: N/A
#   Stream #0:0: Audio: aac (LC), 44100 Hz, stereo, fltp, 128 kb/s
#   Stream #0:1: Video: h264 (High), yuv420p(tv, bt709, progressive), 1280x720, 1274 kb/s, 30 fps, 30 tbr, 1k tbn
# [FORMAT]
# filename=live.flv
# nb_streams=2
# nb_programs=0
# format_name=flv
# format_long_name=FLV (Flash Video)
# start_time=365073.018000
# duration=0.000000
# size=1001369
# bit_rate=N/A
# probe_score=100
# TAG:Rawdata=
# TAG:displayWidth=1280
# TAG:displayHeight=720
# TAG:fps=30
# TAG:profile=
# TAG:level=
# TAG:encoder=Lavf58.29.100
# [/FORMAT]


if (!$env:Path.Contains("ffmpeg")) {
    $env:Path += ";$env:USERPROFILE\Desktop\ffmpeg\bin"
}

.\bili_live.exe 21564812

Try {
    ffmpeg -i live.flv -an -c:v rawvideo -pixel_format yuv420p live.yuv
    ffmpeg -i live.flv -vn -ar 44100 -ac 2 -f s16le live.pcm
    echo ""
    echo ""
    echo "ffprobe -show_format live.flv"
    echo ""
    echo "ffplay -f rawvideo -video_size 1280x720 live.yuv"
    echo ""
    echo "ffplay -ar 44100 -ac 2 -f s16le -i live.pcm"
} Catch {
    echo "FFmpeg Not Found, Please Download That To Desktop"
}

pause