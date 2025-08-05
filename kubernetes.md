# Instructions

1. Create Dockerfile
2. build docker image `docker create -t docker-image-name .` 
3. Create `manifests/deployment.yaml`
4. kubernetes apply `deployment`: ` kubectl apply -f manifests/deployment.yaml`
5. (Optional): check if pods are active: `kubectl get pods -l app=name` name is found in `deployment.yaml`: spec -> containers -> - name


# Kubernetes Basics

## First Deploy

### Microservices
`Microservices are small, autonomous services that work together`
Opposite: Monolith.

Microservice applications can compose of serveral, even dozens independently operatinng services.

### What is Kubernetes?
`Kuberenetes is an open-source system for automatic deployment, scaling and management of containerized applications.`
The main responsibility of an orchestration-system is starting and stopping of containers.

### What is a cluster (with k3d)?
A cluster is a group of machines (called nodes) that work together.
- server node: nodes with control plane
- agent node: nodes without control plane


### Tools

- `kubectl`: Kubernetes cli and allows interaction with the cluster.
- `k3d`: lightweight kubernetes distribution. 

### Create 1. cluster

command: 
```console
k3d cluster create -a 2

```
this creates a Kubernetes cluster with 2 agents, 1 server and 1 load balancer.
```
 ghcr.io/k3d-io/k3d-proxy:5.8.3   "/bin/sh -c nginx-pr…"      80/tcp, 0.0.0.0:37601->6443/tcp   k3d-k3s-default-serverlb
```

The cluster can be accessed over port 6443 (in this example) using the load balancer.
We can opt out of the load balancer using the `--no-lb` flag. In this case the port opens to the server node.


### What is a pod?
Pod is an abstraction around one or more containers. It provides context so that the containers can share storage and a network.


### What is a Deployment resource?
It takes care of the deployment. It tells Kubernetes what container you want, how they should be running and how many of them should be running.

**Replica sets**: Replica Sets are used to tell how many replicas of a Pod should be running. They are managed by the deployments
