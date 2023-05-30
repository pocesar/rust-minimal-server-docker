# rust-minimal-server-docker

A tiny and low memory file server that runs on Docker to serve your files

## Usage

```bash
docker run --rm -v /some/folder:/serve -p 8080:8080 ghcr.io/pocesar/rust-minimal-server-docker:latest \
  --pattern "*.zip"
```

To match all files in all folders, use:

```bash
--pattern "**/*.zip"
```

## License

MIT
