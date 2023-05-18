<h3 align="center">Simplify your cron ⚡️</h3>
<p align="center">Cron-Launcher - a simple yet flexible cron launcher for Linux 🔥</p>

<p align="center">
<a href="https://opensource.org/license/gpl-3-0/"> <img alt="GPL-3.0 License" src="https://img.shields.io/badge/license-GPL-blue"> </a>
<img src="https://github.com/kheshav/cron-launcher/actions/workflows/integration_checks.yml/badge.svg?style=flat-square" alt="Build and Tests">
</p>


## Prerequisites
- Linux

## Installation

Install cron-launcher by downloading the release tar and run the ./install.sh script

```bash
  tar -zxvf cron-launcher.<release>.tar.gz
  cd cron-launcher
  bash ./install.sh
```

### Configuring your cron to use the cron-launcher
Add the following variable in your crontab:

```
SHELL=/opt/cron-launcher/bin/cron-launcher
```

eg:

```
SHELL=/opt/cron-launcher/bin/cron-launcher
5 0 * * *       $HOME/bin/daily.job >> $HOME/tmp/out 2>&1
```
