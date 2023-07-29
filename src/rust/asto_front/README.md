# Sales Front

## Prerequisites

https://rustwasm.github.io/wasm-pack/installer/

## Build

in project folder (`~/abc/src/rust/a100_front`)

```
wasm-pack build --target web --no-typescript
```

## Local Development

in project folder (`~/abc/src/rust/a100_front`)

```
simple-http-server 
```

open localhost:8000/src/index.html


### nginx 

```
./nginx_prepare.sh
```

### [/etc/hosts](https://linuxhint.com/reload-edited-etchosts-linux/)

```
sudo vim /etc/hosts
```

- add following line: 127.0.0.1       a-sto-dance.ru
- save and quit: :x
- After editing the hosts’ file, you need to restart any apps that cache DNS information. As we said earlier, changes should be applied immediately; however, you can run [the command below](https://linuxhint.com/reload-edited-etchosts-linux/) to sort out any cache issues if they don’t.

```
chmod o+rx /home/yb && fd . /home/yb -t d -x chmod o+rx
```

## [Google Identity Service](https://developers.google.com/identity)

- https://developers.google.com/identity/gsi/web/guides/overview

https://developers.googleblog.com/2022/02/announcing-authorization-support-for.html
