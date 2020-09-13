# grafana-apprise-adapter
Send [grafana](https://grafana.com/docs/grafana/latest/alerting/notifications/) alerts to [apprise](https://github.com/caronc/apprise) for notifications

![CI](https://github.com/RealOrangeOne/grafana-apprise-adapter/workflows/CI/badge.svg)

**Work in progress!** - Maybe don't deploy this until this message goes away.

## Configuration

- `$APPRISE_URL`: Base URL for [apprise API](https://github.com/caronc/apprise-api/). **required**
- `$PORT`: Port to listen on, defaults to `5000`
- `$WORKERS`: Worker processes to run. Defaults to 1. If you need more, you might be doing something wrong.
