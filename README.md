## zellij-lats-session

A simple [zellij][zellij] plugin to quickly switch to last active session.

Load the plugin

```kdl
plugins {
    switch-to-last-session location="https://github.com/inoc603/zellij-last-session/releases/download/0.1.0/zellij-last-session.wasm"
}

load_plugins {
    switch-to-last-session
}
```

Add key binding

```kdl
    bind "r" {
        MessagePlugin "switch-to-last-session" {
            payload "switch-to-last-session"
        };
    SwitchToMode "Normal";
    }
```
