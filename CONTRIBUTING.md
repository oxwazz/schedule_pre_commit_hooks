# Contributing Guidelines

*Pull requests, bug reports, and all other forms of contribution are welcomed and highly encouraged!* :octocat:

> **This guide serves to set clear expectations for everyone involved with the project so that we can improve it
together while also creating a welcoming space for everyone to participate. Following these guidelines will help ensure
a positive experience for contributors and maintainers.**

### Contents

- [Install](#books-install)
- [How it Works](#thread-how-it-works)

## :books: Install

Before you start contributing you must clone and installing this project on your local machine.

> Prerequisite: you must install [pre-commit](https://pre-commit.com/#installation) v4.0.1+ on your machine first

1. Clone the project

    ```sh
    # using ssh
    git clone git@github.com:oxwazz/schedule_pre_commit_hooks.git
    # or using https
    git clone https://github.com/oxwazz/schedule_pre_commit_hooks.git
    ```

2. Create dummy repo > add `.pre-commit-config.yaml` > `pre-commit` initialization > update hooks

   a. Create dummy repo

    ```bash
    mkdir dummy_repo
    cd dummy_repo
    git init
    ```

   b. add `.pre-commit-config.yaml`

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
              - --schedule=sunday|05:00-10:00
              - --schedule=monday|05:00-07:00,20:00-23:00
    ```

   c. `pre-commit` initialization

    ```bash
    pre-commit install
    ```

   d. update hooks

    ```bash
    pre-commit autoupdate
    ```

3. you can commit anything to testing the hook, and then check your latest commit date

done 🎉

## :thread: How it works

All the code in main.rs. It modifies your latest commit date based on allowed dates.
