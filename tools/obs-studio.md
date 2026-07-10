# OBS Studio

> https://obsproject.com/

Free, open-source software for screen recording and live streaming. Runs on Linux, macOS, and
Windows. These notes focus on a Linux desktop workflow: hardware-accelerated recording, a
studio-quality microphone filter chain, webcam effects, and manual plugin installation.

## Install (Linux)

- **Flatpak** (self-contained, recommended for hassle-free codec/VA-API support):

  ```bash
  flatpak install flathub com.obsproject.Studio
  ```

- **Native package** (distro repo) — e.g. `obs-studio` on Arch/openSUSE/Debian. Native builds pick
  up system GStreamer/VA-API plugins directly, which the recording setup below relies on.

User config, scenes, profiles, and manually-installed plugins live under:

```text
~/.config/obs-studio/
```

## Recording & encoders

For recording (as opposed to streaming), prefer a **hardware encoder** when your GPU has one — it
offloads work from the CPU so gameplay/capture doesn't lag. On Linux the usual options are VA-API
(Intel/AMD), NVENC (NVIDIA), and the CPU-based `x264`. `x264` gives the best quality-per-bitrate but
is the heaviest on the CPU; hardware encoders trade a little quality for a much lighter load. Choose
via **Settings → Output → Output Mode: Advanced → Recording**.

### VA-API via GStreamer (Intel/AMD)

Path from the LoftyPancake walkthrough (see References):

1. **Settings → Output → Output Mode: Advanced**.
2. **Video Encoder: GStreamer**.
3. **Encoder type: VA-API**.

For general "no-lag" recording settings, see the SlurpTech and Agent (low-end PC) guides in
References.

## Audio: mic filter chain

Goal: make a cheap mic sound expensive. First set levels, then stack filters in order. Add filters
under **Audio Mixer → (mic) → gear → Filters**. Filters process **top to bottom**, so order matters.

**Level the input first:**

- Raise the OBS mic volume toward max.
- Adjust the **system** mic volume so that speaking normally lands in the **yellow** range on the
  mic's audio-track meter.
- To hear yourself: **Audio Mixer → Advanced Audio Properties → Audio Monitoring → Monitor and
  Output**.

**Then add the filter chain, in this order:**

1. **Gain** — only if max system volume + max OBS volume still falls short of the yellow range. Add
   just enough gain to reach it.
2. **Noise Suppression** — method: **Speex**.
3. **3-Band Equalizer** — reduce the **mid** a little (sounds more professional), optionally add a
   touch to the **high**, then fine-tune the high. Toggle the filter on/off to compare raw vs.
   adjusted.
4. **Expander** — quiets the mic when you're not talking (gates background hiss):
   - Ratio: **3**
   - Attack: **1 ms**
   - Release: **100 ms**
   - Threshold: start fully **left**.
   - Speak as quietly as you would at your minimum volume; watch the mixer and raise **Output Gain**
     to reach the yellow mark.
   - Then raise the **Threshold** to a low spot where your speech reliably opens the gate.
5. **Compressor** — evens out loud vs. quiet passages:
   - Ratio: **3**
   - Threshold: start fully **right**.
   - Attack: **1 ms**
   - Release: **100 ms**
   - Shout into the mic and lower the **Threshold** until the shout stops peaking — aim for the
     shout to land around **-1 dB**.
6. **Limiter** — hard ceiling to stop clipping:
   - Threshold: fully **right**, then one click down (about **-0.1 dB**).

### Monitoring / virtual mic

Route processed mic audio into other apps (Discord, browser, etc.) using a PulseAudio virtual cable
(see NapoleonWils0n in References):

1. **Settings → Audio → Advanced → Monitoring Device →** choose the virtual cable.
2. Make sure the mic's monitoring is set to **Monitor and Output**.
3. In the other app, select the **virtual mic** as its input device.

## Effects & masks

Apply these under a source's **Filters** (webcam / video capture device unless noted).

### Advanced Masks

> https://obsproject.com/forum/resources/advanced-masks.1856/

- **Camera source → Filters → Advanced Masks** for animated masking using only OBS.

### Stroke / Glow / Shadow

> Plugin: Stroke Glow Shadow — https://obsproject.com/forum/resources/stroke-glow-shadow.1800/

- **Camera source → Filters → Stroke/Shadow/Glow**.
- **Blur type: Dual Kawase**.
- Use the **stroke** section for outlines.
- If you also use a mask filter, it must sit **before** the shadow filter in the chain.

See [Troubleshooting](#troubleshooting) for the crop/pad gotchas that cut off shadows.

### Circle webcam scene

Crop a webcam into a circle using a nested scene:

1. Create a new scene, e.g. `cam-circle-scene`.
2. In it, add a **Video Capture Device** source for the webcam.
3. On the scene: **Filters → Effect Filters → + → Image Mask/Blend →** browse to `circle-mask.png`.
4. Resize the source to fit the circle; **lock** the source (padlock).
5. In the main scene: **Sources → + → Scene → `cam-circle-scene`**.

## Plugins (manual install)

> https://wiki.archlinux.org/title/Open_Broadcaster_Software#Encoding_using_GStreamer

Plugins can be dropped into `~/.config/obs-studio/plugins/` using this folder layout:

```text
~/.config/obs-studio/plugins/plugin_name/bin/64-bit/plugin_name.so
~/.config/obs-studio/plugins/plugin_name/data/locale/en-US.ini
```

## Troubleshooting

**Resizing a source cuts off its shadow/glow.** Don't use `Alt`+drag to resize the camera — it clips
the shadow. Instead:

- Add a **Crop/Pad** filter and put it at the **top** (first) of the filter chain.
- Positive numbers crop the borders; use it to change the effective camera size.

**Shadow doesn't show at all.** Give the effect room to draw:

- Add a **Crop/Pad** filter with **negative** values on left/top/right/bottom (acts like padding).
- Drag that Crop/Pad filter **above** the shadow filter.

## References

**Plugins overview**

- [Top 10 OBS Plugins Of All Time (2024) — nutty](https://www.youtube.com/watch?v=kO8VJuIzCJA)

**Recording / encoding**

- [Gstreamer VA-API OBS Debian Flatpak, under 6min — LoftyPancake](https://www.youtube.com/watch?v=Xpi0uo3UAFQ)
- [Best OBS Settings for Recording 2024 — NO LAG — SlurpTech](https://www.youtube.com/watch?v=0eITm_XGELg)
- [BEST OBS Recording Settings For LOW END PC 2024 (NO LAG) — Agent](https://www.youtube.com/watch?v=b0LtsJY9NNI)
- [Encoding using GStreamer — Arch Wiki](https://wiki.archlinux.org/title/Open_Broadcaster_Software#Encoding_using_GStreamer)

**Audio**

- [Make Any Mic Sound Expensive In OBS | Mic Settings & Filters (2023) — The Video Nerd](https://www.youtube.com/watch?v=G1VzeT9t24Y)
- [Create a Virtual Microphone on Linux with Pulseaudio for OBS Studio — NapoleonWils0n](https://www.youtube.com/watch?v=Goeucg7A9qE)

**Effects & masks**

- [ANIMATED Masks Using Nothing But OBS Studio! — nutty](https://www.youtube.com/watch?v=btzlExdrsg4)
  · [Advanced Masks resource](https://obsproject.com/forum/resources/advanced-masks.1856/)
- [DROP SHADOWS Using Only OBS Studio! (Also Outlines & Glows) — nutty](https://www.youtube.com/watch?v=dsVQ_LnUQNM)
  · [Stroke Glow Shadow resource](https://obsproject.com/forum/resources/stroke-glow-shadow.1800/)
- [How to Make Circle Webcam in OBS Studio — Northern Viking Everyday](https://www.youtube.com/watch?v=LgrDeVKXQII)
