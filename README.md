# Cron-Launcher

A simple cron launcher for linux.

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
