## Build

```bash
docker buildx build --platform linux/amd64,linux/arm64 \
    -t modco/link:20240126 \
    -f Dockerfile.musl \
    --push \
    .

docker buildx build --platform linux/amd64,linux/arm64 \
    -t modco/link:20240126 \
    -f Dockerfile.musl \
    --output type=oci,dest=link_20240126.tar \
    .

```

确认 apiversion

```bash
kubectl api-resources | grep Gateway

kubectl apply -f deploy.yaml  -n default
kubectl delete -f deploy.yaml  -n default
```
