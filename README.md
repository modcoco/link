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

1. 路径路由

根据 URL 路径 进行流量分流，类似于 K8s Ingress。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: path-routing
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - match:
        - uri:
            prefix: /service1
      route:
        - destination:
            host: service1
            port:
              number: 80
    - match:
        - uri:
            prefix: /service2
      route:
        - destination:
            host: service2
            port:
              number: 80
```

效果：
• 访问 example.com/service1，流量转发到 service1
• 访问 example.com/service2，流量转发到 service2

2. 基于权重的灰度发布

将流量按权重分配到不同版本（v1、v2），可用于 灰度发布 / A/B 测试。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: canary-release
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - route:
        - destination:
            host: my-service
            subset: v1
          weight: 80 # 80% 流量
        - destination:
            host: my-service
            subset: v2
          weight: 20 # 20% 流量
```

效果：
• 80% 的流量进入 my-service 的 v1
• 20% 的流量进入 my-service 的 v2
• 适用于 逐步放量、灰度发布

3. 流量镜像

把真实流量镜像（复制）到另一个服务，但不会影响原本请求的返回，适用于 流量回放 / 预发布环境测试。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: traffic-mirroring
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - route:
        - destination:
            host: my-service
            subset: v1
    - mirror:
        host: my-service
        subset: v2
      mirrorPercentage:
        value: 50 # 50% 请求被镜像到 v2
```

效果：
• 100% 真实流量走 v1
• 50% 的流量被复制到 v2，但用户仍然得到 v1 的响应
• v2 不会影响生产流量，但可以用来观测新版本行为

4. 基于 Header 的流量控制

可以根据 HTTP Header 分流，比如特定用户请求流量进入 v2，其他人使用 v1。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: header-routing
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - match:
        - headers:
            user:
              exact: "premium-user" # 如果请求 Header 里 user=premium-user
      route:
        - destination:
            host: my-service
            subset: v2
    - route:
        - destination:
            host: my-service
            subset: v1
```

效果：
• user=premium-user 的请求进入 v2
• 其他请求进入 v1
• 适用于 AB 测试、高级用户专属功能

5. 故障注入

可以模拟 请求超时、服务故障，用于测试系统的容错性。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: fault-injection
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - fault:
        delay:
          percentage:
            value: 50 # 50% 的请求故意延迟
          fixedDelay: 5s # 延迟 5 秒
        abort:
          percentage:
            value: 10 # 10% 请求失败
          httpStatus: 500 # 返回 500 错误
      route:
        - destination:
            host: my-service
```

效果：
• 50% 的请求 故意延迟 5 秒
• 10% 的请求 直接返回 500
• 适用于 故障演练、混沌测试

6. 超时与重试

防止请求长时间挂起，并支持自动重试。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: timeout-retry
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - timeout: 2s # 请求超时时间 2 秒
      retries:
        attempts: 3 # 失败后最多重试 3 次
        perTryTimeout: 1s # 每次重试最多等待 1 秒
      route:
        - destination:
            host: my-service
```

效果：
• 每次请求最多 等待 2 秒
• 失败后 最多重试 3 次
• 适用于 网络抖动、短暂错误的恢复

7. 流量分片（子集路由）

搭配 DestinationRule，可以对流量进行不同版本管理。

```yaml
apiVersion: networking.istio.io/v1beta1
kind: DestinationRule
metadata:
  name: my-service
spec:
  host: my-service
  subsets:
    - name: v1
      labels:
        version: v1
    - name: v2
      labels:
        version: v2
```

配合 VirtualService：

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: version-routing
spec:
  hosts:
    - "example.com"
  gateways:
    - my-gateway
  http:
    - match:
        - headers:
            version:
              exact: "v2"
      route:
        - destination:
            host: my-service
            subset: v2
    - route:
        - destination:
            host: my-service
            subset: v1
```

效果：
• Header version=v2 进入 v2
• 其他请求进入 v1
• 适用于 多版本管理


路径路由	根据 URL 路径分流
权重分流	灰度发布、A/B 测试
流量镜像	生产环境测试新版本
Header 路由	用户分层、灰度测试
故障注入	混沌工程、故障演练
超时与重试	网络不稳定环境
子集路由	版本管理、多环境控制