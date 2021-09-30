---
description: Let's get you started with running your own Bender server.
---

# Getting Started

## Starting the Server

We package up Bender as a Docker image which can be run on any Linux x86\_64 host. By default, the container will listen on port 8000.

```
$ docker pull ghcr.io/sierrasoftworks/bender/bender
$ docker run -p 8000:8000 ghcr.io/sierrasoftworks/bender/bender
```

What's that, you're not impressed? Want more pain do you? Fine...

## Running on Kubernetes

For those masochists among you who can't help but dabble in the dark arts, you can deploy Bender on a Kubernetes cluster with the following hieroglyphs. We accept no responsibility for any demons this may summon.

{% tabs %}
{% tab title="Deployment" %}
{% code title="bender/deployment.yml" %}
```yaml
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: bender-server
  labels:
    app.kubernetes.io/name: bender
    app.kubernetes.io/instance: bender-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app.kubernetes.io/name: bender
      app.kubernetes.io/instance: bender-server
  template:
    metadata:
      labels:
        app.kubernetes.io/name: bender
        app.kubernetes.io/instance: bender-server
    spec:
      containers:
        - name: server
          image: ghcr.io/sierrasoftworks/bender/bender:latest
          imagePullPolicy: IfNotPresent
          resources:
            requests:
              cpu: 10m
              memory: 20Mi
            limits:
              cpu: 500m
              memory: 100Mi
          env:
            - name: ENVIRONMENT
              valueFrom:
                fieldRef:
                  fieldPath: metadata.namespace
          ports:
            - name: http
              containerPort: 8000
              protocol: TCP
          readinessProbe:
            httpGet:
              port: http
              path: /api/v1/health
            initialDelaySeconds: 5
            periodSeconds: 1
          livenessProbe:
            httpGet:
              port: http
              path: /api/v1/health
            initialDelaySeconds: 30
            periodSeconds: 1
```
{% endcode %}
{% endtab %}

{% tab title="Service" %}
{% code title="bender/service.yml" %}
```yaml
---
apiVersion: v1
kind: Service
metadata:
  name: bender-server
spec:
  selector:
    app.kubernetes.io/name: bender
    app.kubernetes.io/instance: bender-server
  ports:
    - name: http
      port: 80
      targetPort: http
      protocol: TCP
```
{% endcode %}
{% endtab %}
{% endtabs %}



