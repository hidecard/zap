# Zap Cloud and Container Deployment Guide

**Baseline:** Zap v2.11.18
**Companion:** [Production Deployment Runbook](PRODUCTION_DEPLOYMENT_EN.md) · [Registry Deployment Boundaries](DEPLOYMENT_EN.md) · [Security Policy](../../SECURITY.md)

This guide extends the on-premise Ubuntu deployment runbook with platform-specific deployment patterns for Docker, Kubernetes, AWS, Azure, and GCP. The core principle is identical across platforms: **Zap stays on loopback; the TLS proxy faces public traffic; secrets come from the platform; the registry backend never touches the public network directly.**

---

## Architecture (platform-agnostic)

```text
Public Internet
     │
     ▼
┌──────────────┐     TLS 1.2/1.3     ┌──────────────────────┐
│ Load Balancer │ ──────────────────► │  Nginx / ALB /       │
│ (port 80/443) │   HTTPS only       │  Application Gateway │
└──────────────┘                      └──────────┬───────────┘
                                                │ loopback
                                                ▼
                                       ┌────────────────┐
                                       │  zap-web.service│
                                       │  (127.0.0.1)   │
                                       └────────┬───────┘
                                                │ loopback
                                                ▼
                                       ┌────────────────┐
                                       │  zap registry   │
                                       │  (127.0.0.1:8787)│
                                       └────────┬───────┘
                                                │
                                                ▼
                                       ┌────────────────┐
                                       │  SQLite / DB    │
                                       │  /var/lib/zap/  │
                                       └────────────────┘
```

---

## 1. Docker Deployment

### 1.1 Dockerfile

```dockerfile
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        sqlite3 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system zap && \
    useradd --system --gid zap --home /srv/zap --shell /usr/sbin/nologin zap

COPY bin/zap /usr/local/bin/zap
RUN chmod 0755 /usr/local/bin/zap

RUN install -d -o zap -g zap -m 0750 /srv/zap/app/data
RUN install -d -o root -g root -m 0755 /etc/zap/tls
RUN install -d -o root -g zap -m 0700 /etc/zap

COPY deploy/zap-web.service /etc/systemd/system/
COPY deploy/zap-web-migrate.service /etc/systemd/system/
COPY deploy/zap-web.nginx.conf /etc/nginx/sites-available/zap-web.conf

USER zap
WORKDIR /srv/zap/app
```

### 1.2 Docker Compose

```yaml
# docker-compose.yml
version: "3.9"

services:
  zap-web:
    build: .
    container_name: zap-web
    restart: unless-stopped
    read_only: true
    tmpfs:
      - /tmp:size=64m
    volumes:
      - zap-data:/srv/zap/app/data
      - zap-tls:/etc/zap/tls:ro
      - ./deploy/zap-web.env:/etc/zap/zap-web.env:ro
    networks:
      - backend
    depends_on:
      - registry

  registry:
    image: ghcr.io/hidecard/zap-registry:latest
    container_name: zap-registry
    restart: unless-stopped
    read_only: true
    tmpfs:
      - /tmp:size=32m
    volumes:
      - registry-data:/var/lib/zap-registry
      - ./deploy/registry.env:/etc/zap/registry.env:ro
    networks:
      - backend
    environment:
      - ZAP_REGISTRY_BIND=127.0.0.1:8787

  nginx:
    image: nginx:alpine
    container_name: zap-nginx
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./deploy/zap-web.nginx.conf:/etc/nginx/conf.d/default.conf:ro
      - zap-tls:/etc/zap/tls:ro
    networks:
      - backend
      - public
    depends_on:
      - zap-web

volumes:
  zap-data:
  registry-data:
  zap-tls:

networks:
  backend:
    driver: bridge
  public:
    driver: bridge
```

### 1.3 Deployment commands

```bash
# Build and start
docker compose build
docker compose up -d

# Verify
docker compose ps
curl -fsS https://localhost/health
curl -fsS http://127.0.0.1:8787/healthz

# Logs
docker compose logs -f zap-web
docker compose logs -f registry

# Stop
docker compose down
```

---

## 2. Kubernetes Deployment

### 2.1 Namespace

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: zap
  labels:
    app: zap
```

### 2.2 Registry Deployment

```yaml
# k8s/registry-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: zap-registry
  namespace: zap
spec:
  replicas: 1
  selector:
    matchLabels:
      app: zap-registry
  template:
    metadata:
      labels:
        app: zap-registry
    spec:
      containers:
        - name: registry
          image: ghcr.io/hidecard/zap-registry:latest
          ports:
            - containerPort: 8787
          envFrom:
            - secretRef:
                name: zap-registry-secrets
          resources:
            requests:
              memory: "128Mi"
              cpu: "250m"
            limits:
              memory: "256Mi"
              cpu: "500m"
          securityContext:
            runAsNonRoot: true
            runAsUser: 1000
            readOnlyRootFilesystem: true
            allowPrivilegeEscalation: false
            capabilities:
              drop: ["ALL"]
          volumeMounts:
            - name: registry-data
              mountPath: /var/lib/zap-registry
            - name: tmp
              mountPath: /tmp
      volumes:
        - name: registry-data
          persistentVolumeClaim:
            claimName: zap-registry-data
        - name: tmp
          emptyDir:
            sizeLimit: 32Mi
---
apiVersion: v1
kind: Service
metadata:
  name: zap-registry
  namespace: zap
spec:
  selector:
    app: zap-registry
  ports:
    - port: 8787
      targetPort: 8787
  clusterIP: None
```

### 2.3 Web Application Deployment

```yaml
# k8s/web-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: zap-web
  namespace: zap
spec:
  replicas: 2
  selector:
    matchLabels:
      app: zap-web
  template:
    metadata:
      labels:
        app: zap-web
    spec:
      containers:
        - name: web
          image: ghcr.io/hidecard/zap-web:latest
          ports:
            - containerPort: 3000
          envFrom:
            - secretRef:
                name: zap-web-secrets
          resources:
            requests:
              memory: "128Mi"
              cpu: "250m"
            limits:
              memory: "256Mi"
              cpu: "500m"
          securityContext:
            runAsNonRoot: true
            runAsUser: 1000
            readOnlyRootFilesystem: true
            allowPrivilegeEscalation: false
            capabilities:
              drop: ["ALL"]
          volumeMounts:
            - name: app-data
              mountPath: /srv/zap/app/data
            - name: tmp
              mountPath: /tmp
      volumes:
        - name: app-data
          persistentVolumeClaim:
            claimName: zap-web-data
        - name: tmp
          emptyDir:
            sizeLimit: 64Mi
---
apiVersion: v1
kind: Service
metadata:
  name: zap-web
  namespace: zap
spec:
  selector:
    app: zap-web
  ports:
    - port: 3000
      targetPort: 3000
  clusterIP: None
```

### 2.4 Ingress (Nginx Ingress Controller)

```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: zap-ingress
  namespace: zap
  annotations:
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"
    nginx.ingress.kubernetes.io/proxy-body-size: "1m"
    nginx.ingress.kubernetes.io/proxy-read-timeout: "15"
    nginx.ingress.kubernetes.io/proxy-send-timeout: "15"
    nginx.ingress.kubernetes.io/limit-connections: "32"
spec:
  ingressClassName: nginx
  tls:
    - hosts:
        - app.example.com
      secretName: zap-tls-secret
  rules:
    - host: app.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: zap-web
                port:
                  number: 3000
```

### 2.5 Persistent Volumes

```yaml
# k8s/pvc.yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: zap-web-data
  namespace: zap
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: zap-registry-data
  namespace: zap
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 512Mi
```

### 2.6 Secrets (example)

```yaml
# k8s/secrets.yaml (populate via kubectl create secret or external secrets operator)
apiVersion: v1
kind: Secret
metadata:
  name: zap-registry-secrets
  namespace: zap
type: Opaque
stringData:
  ZAP_REGISTRY_TOKEN: "<replace-with-platform-managed-secret>"
  ZAP_REGISTRY_SIGNING_SECRET: "<replace-with-platform-managed-secret>"
---
apiVersion: v1
kind: Secret
metadata:
  name: zap-web-secrets
  namespace: zap
type: Opaque
stringData:
  ZAP_DB_MAX_CONNECTIONS: "32"
  ZAP_DB_ACQUIRE_TIMEOUT_MS: "5000"
  ZAP_DB_QUERY_TIMEOUT_MS: "30000"
```

### 2.7 Deployment commands

```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/pvc.yaml
kubectl apply -f k8s/secrets.yaml
kubectl apply -f k8s/registry-deployment.yaml
kubectl apply -f k8s/web-deployment.yaml
kubectl apply -f k8s/ingress.yaml

kubectl -n zap get pods
kubectl -n zap logs -l app=zap-web
kubectl -n zap logs -l app=zap-registry
```

---

## 3. AWS Deployment

### 3.1 Architecture

```text
Route 53 (app.example.com)
     │
     ▼
┌──────────────┐
│  CloudFront   │ (optional CDN/WAF)
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  ALB          │ (Application Load Balancer)
│  (HTTPS)      │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  ECS Fargate  │  zap-web task (2+ tasks)
│  or EC2       │
└──────┬───────┘
       │ VPC loopback/VPC
       ▼
┌──────────────┐
│  RDS PostgreSQL│ (or S3 + SQLite for smaller workloads)
│  or ECS Registry│  zap-registry task (loopback within task)
└──────────────┘
```

### 3.2 ECS Fargate Task Definition (zap-web)

```json
{
  "family": "zap-web",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "executionRoleArn": "arn:aws:iam::ACCOUNT_ID:role/ecsTaskExecutionRole",
  "containerDefinitions": [
    {
      "name": "zap-web",
      "image": "ACCOUNT_ID.dkr.ecr.REGION.amazonaws.com/zap-web:latest",
      "portMappings": [
        {
          "containerPort": 3000,
          "protocol": "tcp"
        }
      ],
      "environment": [
        { "name": "ZAP_DB_MAX_CONNECTIONS", "value": "32" },
        { "name": "ZAP_DB_ACQUIRE_TIMEOUT_MS", "value": "5000" },
        { "name": "ZAP_DB_QUERY_TIMEOUT_MS", "value": "30000" }
      ],
      "secrets": [
        {
          "name": "ZAP_REGISTRY_TOKEN",
          "valueFrom": "arn:aws:secretsmanager:REGION:ACCOUNT_ID:secret:zap-registry-token"
        },
        {
          "name": "ZAP_REGISTRY_SIGNING_SECRET",
          "valueFrom": "arn:aws:secretsmanager:REGION:ACCOUNT_ID:secret:zap-registry-signing"
        }
      ],
      "essential": true,
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/zap-web",
          "awslogs-region": "REGION",
          "awslogs-stream-prefix": "zap"
        }
      }
    }
  ]
}
```

### 3.3 AWS Key Infrastructure

| Component | Recommendation |
|---|---|
| Compute | ECS Fargate (2+ tasks across AZs) or EC2 Auto Scaling Group (min 2) |
| Load Balancer | Application Load Balancer (HTTPS listener, TLS 1.2+) |
| DNS | Route 53 with ALB alias record |
| Secrets | AWS Secrets Manager (rotation enabled) |
| Database | RDS PostgreSQL (production) or DynamoDB (serverless alternative) |
| Static assets | S3 + CloudFront |
| WAF | AWS WAF attached to ALB (rate limiting, SQL injection, XSS rules) |
| Monitoring | CloudWatch (logs, metrics, alarms) + X-Ray (tracing) |
| CI/CD | CodePipeline → ECR push → ECS rolling update |

### 3.4 AWS Deployment Steps

```bash
# Build and push image
aws ecr get-login-password | docker login --username AWS --password-stdin ACCOUNT_ID.dkr.ecr.REGION.amazonaws.com
docker build -t zap-web:latest .
docker tag zap-web:latest ACCOUNT_ID.dkr.ecr.REGION.amazonaws.com/zap-web:latest
docker push ACCOUNT_ID.dkr.ecr.REGION.amazonaws.com/zap-web:latest

# Deploy via CloudFormation or Terraform
aws cloudformation create-stack \
  --stack-name zap-web \
  --template-body file://aws/cloudformation-template.yaml \
  --parameters ParameterKey=ImageTag,ParameterValue=latest \
  --capabilities CAPABILITY_IAM
```

### 3.5 AWS Security Groups

```text
ALB Security Group:
  Inbound: TCP 443 from 0.0.0.0/0, TCP 80 from 0.0.0.0/0
  Outbound: TCP 3000 to ECS tasks security group

ECS Tasks Security Group:
  Inbound: TCP 3000 from ALB security group only
  Outbound: TCP 443 to RDS security group, TCP 5432 to RDS
  All other outbound: DENY
```

---

## 4. Azure Deployment

### 4.1 Architecture

```text
Azure DNS (app.example.com)
     │
     ▼
┌───────────────────┐
│  Azure Front Door  │ (optional WAF)
└──────────┬────────┘
           │
           ▼
┌───────────────────┐
│  Application Gateway│ (HTTPS, WAF enabled)
└──────────┬────────┘
           │
           ▼
┌───────────────────┐
│  Azure Container   │  zap-web (2+ instances)
│  Apps (Linux)      │  or Azure Virtual Machine Scale Set
└──────────┬────────┘
           │ VNET internal
           ▼
┌───────────────────┐
│  Azure Database for│  PostgreSQL (production)
│  PostgreSQL        │  or Azure SQL
└───────────────────┘
```

### 4.2 Azure Container Apps (recommended)

```yaml
# azure/container-app.yaml
apiVersion: apps.azure.com/v1
kind: ContainerApp
metadata:
  name: zap-web
  namespace: zap
spec:
  location: eastus
  configuration:
    ingress:
      external: true
      targetPort: 3000
      tls:
        minimumVersion: v1_2
      transport: httpsOnly
      customDomains:
        - name: app.example.com
          secretRef: zap-tls-secret
    registry:
      server: ACR_SERVER.azurecr.io
      identity: zap-caas-runtime
  template:
    containers:
      - name: zap-web
        image: ACR_SERVER.azurecr.io/zap-web:latest
        resources:
          requests:
            cpu: 0.25
            memory: 0.5Gi
          limits:
            cpu: 0.5
            memory: 0.5Gi
        env:
          - name: ZAP_DB_MAX_CONNECTIONS
            value: "32"
          - name: ZAP_DB_ACQUIRE_TIMEOUT_MS
            value: "5000"
        envSrcs:
          - name: zap-registry-secrets
            secretRef: zap-registry-token
            secretRef: zap-registry-signing
```

### 4.3 Azure Key Vault Integration

| Secret | Key Vault Name |
|---|---|
| Registry Token | `zap-registry-token` |
| Registry Signing Secret | `zap-registry-signing` |
| Database Connection String | `zap-db-connection` |
| TLS Certificate | `zap-tls-cert` |

```bash
az keyvault set-attributes \
  --name zap-kv \
  --enabled-for-deployment true \
  --enabled-for-template-deployment true
```

### 4.4 Azure Deployment Steps

```bash
# Login and create resources
az login
az group create --name zap-rg --location eastus

# Create Container App Environment
az containerapp env create \
  --name zap-env \
  --resource-group zap-rg \
  --location eastus

# Deploy web app
az containerapp create \
  --name zap-web \
  --resource-group zap-rg \
  --environment zap-env \
  --image ACR_SERVER.azurecr.io/zap-web:latest \
  --target-port 3000 \
  --ingress external \
  --min-replicas 2 \
  --max-replicas 10 \
  --cpu 0.25 \
  --memory 0.5Gi

# Deploy registry (internal only)
az containerapp create \
  --name zap-registry \
  --resource-group zap-rg \
  --environment zap-env \
  --image ACR_SERVER.azurecr.io/zap-registry:latest \
  --target-port 8787 \
  --ingress none \
  --cpu 0.25 \
  --memory 0.25Gi
```

---

## 5. GCP Deployment

### 5.1 Architecture

```text
Cloud DNS (app.example.com)
     │
     ▼
┌───────────────────┐
│  Cloud Load Balancer│ (HTTPS, managed certificate)
└──────────┬────────┘
           │
           ▼
┌───────────────────┐
│  Cloud Run         │  zap-web (2+ instances, min instances ≥ 2)
│  or GKE            │  or Compute Engine Managed Instance Group
└──────────┬────────┘
           │ VPC internal
           ▼
┌───────────────────┐
│  Cloud SQL         │  PostgreSQL (production)
│  or Firestore      │  (serverless alternative)
└───────────────────┘
```

### 5.2 Cloud Run Deployment

```yaml
# gcp/cloud-run.yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: zap-web
  namespace: zap
  annotations:
    run.googleapis.com/max-instances: 20
    run.googleapis.com/min-instances: 2
    run.googleapis.com/cpu-throttling: true
    run.googleapis.com/memory: "512Mi"
    run.googleapis.com/cpu: "1000m"
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/target: 100
    spec:
      containers:
        - image: us-docker.pkg.dev/PROJECT_ID/zap-registry/zap-web:latest
          ports:
            - containerPort: 3000
          env:
            - name: ZAP_DB_MAX_CONNECTIONS
              value: "32"
            - name: ZAP_DB_ACQUIRE_TIMEOUT_MS
              value: "5000"
          envFrom:
            - secretKeyRef:
                name: zap-registry-secrets
                path: registry.env
      vpc-access:
        connector: projects/PROJECT_ID/locations/us-central1/connectors/zap-vpc
        egress: all-traffic
```

### 5.3 GCP Deployment Steps

```bash
# Build and push to Artifact Registry
gcloud auth configure-docker us-central1-docker.pkg.dev
docker build -t us-central1-docker.pkg.dev/PROJECT_ID/zap-registry/zap-web:latest .
docker push us-central1-docker.pkg.dev/PROJECT_ID/zap-registry/zap-web:latest

# Deploy to Cloud Run
gcloud run deploy zap-web \
  --image us-central1-docker.pkg.dev/PROJECT_ID/zap-registry/zap-web:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --min-instances 2 \
  --memory 512Mi \
  --cpu 1 \
  --set-env-vars ZAP_DB_MAX_CONNECTIONS=32,ZAP_DB_ACQUIRE_TIMEOUT_MS=5000 \
  --vpc-connector zap-vpc \
  --no-auto-gen-certificates

# Set up managed SSL certificate
gcloud compute ssl-certificates create zap-tls \
  --domains app.example.com

# Cloud Load Balancer
gcloud compute addresses create zap-ip --global
gcloud compute backend-services create zap-backend \
  --protocol HTTPS \
  --port-name https \
  --health-checks zap-health-check
gcloud compute url-maps create zap-map \
  --default-service zap-backend
gcloud compute target-https-proxies create zap-proxy \
  --url-map zap-map \
  --ssl-certificates zap-tls
```

### 5.4 GCP Networking Rules

```text
Cloud Load Balancer:
  Firewall: allow TCP 443 from 0.0.0.0/0 and ::/0
  Health check: TCP 3000 to Cloud Run (internal)

Cloud Run:
  Ingress: internal-and-cloud-load-balancing only
  Egress: VPC connector → Cloud SQL private IP

Cloud SQL:
  Authorized networks: none (private IP only)
  Connections: from VPC connector only
```

---

## 6. Cross-Platform Checklist

| Item | Docker | Kubernetes | AWS | Azure | GCP |
|---|---|---|---|---|---|
| Loopback Zap | ✅ | ✅ | ✅ | ✅ | ✅ |
| TLS termination | Nginx container | Ingress ALB/Nginx | ALB + ACM | App Gateway | LB + Managed Cert |
| Secret management | Docker secrets | K8s Secrets / External Secrets | AWS Secrets Manager | Key Vault | Secret Manager |
| DB | SQLite volume | PVC | RDS PostgreSQL | Cloud SQL | Cloud SQL |
| Auto-scaling | Compose replicas | HPA | ECS Auto Scaling | Container Apps scale | Cloud Run min/max |
| WAF | Nginx module | Ingress WAF | AWS WAF | WAF V2 | Cloud Armor |
| Health check | curl loopback | Liveness probe | ALB health check | App Gateway probe | Cloud Run health |
| Logging | Docker logs | CloudWatch / Fluentd | CloudWatch | Log Analytics | Cloud Logging |
| CI/CD | Compose build | GitOps (ArgoCD/Flux) | CodePipeline | AKS + ACR | Cloud Build |
| Backup | Volume snapshot | Velero | RDS automated backup | Cloud SQL exports | Cloud SQL automated |

---

## 7. Universal Deployment Principles

1. **Zap process never faces the public internet.** Always place a TLS proxy (Nginx, ALB, App Gateway, Cloud Load Balancer) in front.

2. **Secrets come from the platform.** Never bake `ZAP_REGISTRY_TOKEN`, `ZAP_REGISTRY_SIGNING_SECRET`, or database credentials into container images or repository files.

3. **Registry backend is loopback-only.** Whether in Docker, Kubernetes, or cloud, the registry service binds to `127.0.0.1` within its network namespace.

4. **Minimum privilege.** Run as non-root, drop all Linux capabilities, use read-only filesystems where possible, restrict network access with security groups / firewall rules / VPC boundaries.

5. **Resource limits.** Enforce memory limits (256 MiB recommended), CPU quotas, and connection limits in all environments.

6. **Health + readiness probes.** Deploy `/health` (liveness) and `/ready` (readiness) behind the proxy, and configure the platform to use them for traffic routing.

7. **Database migrations are release operations.** Run migrations as a separate, explicitly invoked step — not as a per-instance boot hook.

8. **Backups are mandatory.** Automate database backups with verified restore procedures before exposing production traffic.

---

## References

- [Production Deployment Runbook](PRODUCTION_DEPLOYMENT_EN.md)
- [Registry Deployment Boundaries](DEPLOYMENT_EN.md)
- [Security Policy](../../SECURITY.md)
- [Deployment artifacts](../../deploy/)
- [B4 Acceptance Contract](../../bootstrap/contracts/B4_ACCEPTANCE.tsv)
