## Website backend

Backend for [my website](https://github.com/thegamerx1/website)

Provides contact form and timesync (clock). Uses the clock of the host so make sure it's up to date.

### Building for ARM

```bash
docker buildx build --platform aarch64 --push --tag privateregistry:5000/websitebackend:x.x .
```
