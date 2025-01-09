# Schedule - pre-commit hooks

[badge-workflow]: https://img.shields.io/github/actions/workflow/status/oxwazz/schedule_pre_commit_hooks/release.yml

[link-workflow]: https://github.com/oxwazz/schedule_pre_commit_hooks/actions/workflows/release.yml

[badge-twitter]: https://img.shields.io/twitter/follow/oxwazz

[link-twitter]: https://x.com/oxwazz

[![badge-workflow]][link-workflow]
[![badge-twitter]][link-twitter]

📆 This is custom [pre-commit](https://pre-commit.com/) hooks for scheduling your commit!

### Contents

- [Usage](#usage)
- [Why](#why)
- [Contributing](#contributing)
- [Credit](#credit)
- [License](#license)

## Usage

🎩 Easily schedule your commit based on allowed dates! Follow the steps below:

Copy the code snippet below and paste it into your `.pre-commit-config.yaml` file.

```yaml
default_install_hook_types: [ pre-commit, post-commit ]
repos:
  - repo: https://github.com/oxwazz/schedule_pre_commit_hooks
    rev: v0.1.1
    hooks:
      - id: schedule
        stages:
          - post-commit
        always_run: true
        args:
          # my timezone
          - --timezone=+0700
          # on sunday your commit date always between 05:00-10:00
          - --schedule=sunday|05:00-10:00
          # on monday your commit date always between 05:00-07:00 or 20:00-23:00
          - --schedule=monday|05:00-07:00,20:00-23:00
```

> [!TIP]
> You can adjust the `--timezone` and `--schedule` with your needs.

Congratulation! 🎉 You are now schedule your commit based on allowed dates!

## Why

¯\\\_(ツ)_/¯

or you can have...

<details>

<summary>1. ...a 'work-life balance' commit</summary>

```yaml
default_install_hook_types: [ pre-commit, post-commit ]
repos:
  - repo: https://github.com/oxwazz/schedule_pre_commit_hooks
    rev: v0.1.1
    hooks:
      - id: schedule
        stages:
          - post-commit
        always_run: true
        args:
          - --timezone=+0700
          - --schedule=monday|08:00-17:00
          - --schedule=tuesday|08:00-17:00
          - --schedule=wednesday|08:00-17:00
          - --schedule=thursday|08:00-17:00
          - --schedule=friday|08:00-17:00
          - --schedule=saturday|08:00-17:00
          - --schedule=sunday|08:00-17:00
```

</details>

<details>

<summary>2. ...a 'stay up late' commit</summary>

```yaml
default_install_hook_types: [ pre-commit, post-commit ]
repos:
  - repo: https://github.com/oxwazz/schedule_pre_commit_hooks
    rev: v0.1.1
    hooks:
      - id: schedule
        stages:
          - post-commit
        always_run: true
        args:
          - --timezone=+0700
          - --schedule=monday|20:00-23:59,00:00-02:00
          - --schedule=tuesday|20:00-23:59,00:00-02:00
          - --schedule=wednesday|20:00-23:59,00:00-02:00
          - --schedule=thursday|20:00-23:59,00:00-02:00
          - --schedule=friday|20:00-23:59,00:00-02:00
          - --schedule=saturday|20:00-23:59,00:00-02:00
          - --schedule=sunday|20:00-23:59,00:00-02:00
```

</details>

<details>

<summary>3. ...a 'morning guy' commit</summary>

```yaml
default_install_hook_types: [ pre-commit, post-commit ]
repos:
  - repo: https://github.com/oxwazz/schedule_pre_commit_hooks
    rev: v0.1.1
    hooks:
      - id: schedule
        stages:
          - post-commit
        always_run: true
        args:
          - --timezone=+0700
          - --schedule=monday|06:00-08:00
          - --schedule=tuesday|06:00-08:00
          - --schedule=wednesday|06:00-08:00
          - --schedule=thursday|06:00-08:00
          - --schedule=friday|06:00-08:00
          - --schedule=saturday|06:00-08:00
          - --schedule=sunday|06:00-08:00
```

</details>

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have you! We have a
[contributing guide](./CONTRIBUTING.md) to help you get involved in the project.

## Credit

📌 Schedule - pre-commit hooks is currently being developed and maintained
by [Muhammad Rahmahalim](https://github.com/oxwazz).<br>
Thank you!

## License

[MIT](./LICENSE) License © 2024 [Muhammad Rahmahalim](https://github.com/oxwazz)