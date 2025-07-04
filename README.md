# Git User
CLI tool for managing different git user profiles, useful if you have multiple repo under different accounts. This works by updating the local `.git/config` file's `user` section, so any future actions will be done as the new user.

## Installing
Installs the binary to `~/.cargo/bin`.
```
cargo install --path .
```

## Testing
```
cargo test --release
```

## Usage
```
Usage: git-user [OPTIONS] <COMMAND>

Commands:
  add     Create a new user
  delete  Deletes a user
  export  Export config
  list    List users
  use     Use user
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Custom config file path [default: ~/.gitusers]
  -h, --help             Print help
```

## Examples
### Add User
Add a new user profile to this tool.
```
git user add user123 user123@test.com
```

Profile name can be set using `-p`. By default, it is the same name as the username.
```
git user add user123 user123@test.com -p 'personal'
```

Signing key can be set using `-k`.
```
git user add user123 user123@test.com -k '~/.ssh/key.pub'
```

SSH command can be set using `-s`.
```
git user add user123 user123@test.com -s 'ssh -i ~/.ssh/key'
```

### Delete User
Delete a user profile from this tool.
```
git user delete user123
```

### List Users
List all users tracked by this tool.
```
git user list
```

### Apply User
Sets the user in `.git/config` so they are recorded as the author of all future changes.
```
git user use user123
```

The repo path can be specified using `-r`. Defaults to the current directory.
```
git user use user123 -r 'repos/my-repo'
```
Verify user using `git config -l --local`.

### Export Config
Exports the config for sharing.
```
git user export
```

### Custom Config Location
Use a custom config location using `-c`.
```
git user list -c 'configs/my-config'
```
