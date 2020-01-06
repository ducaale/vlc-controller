# vlc-controller
This is a VLC controller. It reads commands from a yaml file that has the
same name as the currently playing video file and is stored in the same directory.

Communication happens through vlc http interface. To enable it, you can use this
[Guide](https://wiki.videolan.org/Documentation:Modules/http_intf/).
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
  # note that percent sign should be used for 0-200 scale
  # otherwise it will be in the scale 0-512
  amount: 20%
  at: '10:00'

```

## Installing
If you are a windows user, then you can download it from releases page, otherwise
you need to build it from source.

## Building from source
You will need rust 1.39 or later. To compile run ``cargo build --release``.

## Usage
```
vlc-controller 0.1.0
ducaale <sharaf.13@hotmail.com>

USAGE:
    vlc-controller.exe [OPTIONS] --password <password>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --http-host <http-host>    vlc http intf address [default: localhost]
        --http-port <http-port>    vlc http intf port [default: 8080]
    -p, --password <password>      vlc http intf password
```