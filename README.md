watchmen
===

```toml
[watchmen]
crash_report = "C:\\temp\\watchmen.log"
passthrough_exit_code = true

[execute]
executable = "C:\\temp\\someapp.exe"
current_dir = "C:\\temp"
param = ["-p"]
env = [
  {key = "HOGE", value = "FUGA"},
  {key = "FOO", value = "BAR"}
]
log_dir = "C:\\temp\\logs"
```

## Resources
<a href="https://www.flaticon.com/free-icons/psychologist" title="psychologist icons">Psychologist icons created by Freepik - Flaticon</a>
