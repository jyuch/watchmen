watchmen
===

```toml
[watchmen]
id = "someapp"
passthrough_exit_code = true

[execute]
executable = "C:\\temp\\someapp.exe"
current_dir = "C:\\temp"
param = ["-p"]
env = [
  {key = "HOGE", value = "FUGA"},
  {key = "FOO", value = "BAR"}
]

[log]
base_dir = "C:\\temp\\logs"
remain_only_exists = true

[mail]
insecure = true
server = "localhost"
port = 25
from = "watchmen+someapp@example.com"
to = ["alice@example.com", "bob@example.com"]
```

## Resources
<a href="https://www.flaticon.com/free-icons/psychologist" title="psychologist icons">Psychologist icons created by Freepik - Flaticon</a>
