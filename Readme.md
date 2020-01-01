# vlc-controller
This is a VLC controller. It reads commands from a yaml file that has the
same name as the currently playing video file and is stored in the same directory.

Communication happens through vlc http interface. To enable it, you can use this
[Guide](https://hobbyistsoftware.com/vlcsetup-win-manual).
Password is not currently configurable so you will need to set it to 12345.

3 commands are currently supported which are ``skip``, ``mute`` and ``set_volume``.

## Commands file Example
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

## Compiling from source
You will need rust 1.39 or later. To compile run ``cargo build --release``.

## Todos
- [ ] implement root Error Enum and replace most ``wrap()``s with proper error handling.
- [ ] validate commands file and let the user know what is wrong without crashing.
- [ ] pass http interface password as a parameter.
- [ ] pass ipaddress and port as a parameter.