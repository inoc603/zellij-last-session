## zellij-lats-session

A simple [zellij][zellij] plugin to quickly switch to last active session.

Load the plugin

```kdl
plugins {
    switch-to-last-session location="file:/path/to/zellij-lats-session.wasm"
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
