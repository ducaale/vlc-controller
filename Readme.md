# vlc-controller

This is a WIP VLC controller. the plan is for this program to read commands from a yaml file
that has the same name as the currently playing video file.

Commands will be something like this:

```yml
# videoplayback.yml

- action: skip
  start: '04:40'
  end: '05:12'

- action: mute
  start: '07:30'
  end: '07:32'
  
- action: set_volume
  amount: 20
  at: '10:00'

```