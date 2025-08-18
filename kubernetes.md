# Instructions

## kubernetes:
### Create local k3d registry
 `k3d registry create localregistry.localhost --port 5050`

You can now use the registry like this (example):
1. create a new cluster that uses this registry
`k3d cluster create --registry-use k3d-localregistry.localhost:5050`

2. tag an existing local image to be pushed to the registry
`docker tag nginx:latest k3d-localregistry.localhost:5050/mynginx:v0.1`

3. push that image to the registry
`docker push k3d-localregistry.localhost:5050/mynginx:v0.1`

4. run a pod that uses this image
`kubectl run mynginx --image k3d-localregistry.localhost:5050/mynginx:v0.1`

## System
### get current cluster 
`kubectl config current-context `

### describe deployment
`kubectl describe deployment`

### port-forwarding
`kubectl port-forward pod-name [LOCAL_PORT]:[REMOTE_PORT]`

### Deployment steps 

1. Create Dockerfile
2. build docker image `docker create -t docker-image-name .` 
    1. tag docker image: `docker tag docker-image-name temibonn/docker-image-name:latest` (`latest` is the tag and can change)
    2. push it to docker hub: docker push `temibonn/docker-image-name:latest`
3. Create `manifests/deployment.yaml`
4. kubernetes apply `deployment`: ` kubectl apply -f manifests/deployment.yaml` (Path can be local or **URL**)
5. (Optional): check if pods are active: `kubectl get pods -l app=name` name is found in `deployment.yaml`: spec -> containers -> - name

### Inspect Kubernetes Deployments
1. List deployments: `kubectl get deployments`
2. Describe deployment: `kubectl describe deployment-name`


### Inspect Kubernetes Pods
1. List pods: `kubectl get pods`
2. Describe pod: `kubectl describe pod-name`

# Kubernetes Basics

## First Deploy

### Microservices
`Microservices are small, autonomous services that work together`
Opposite: Monolith.

Microservice applications can compose of serveral, even dozens independently operating services.

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

**Replica sets**: Replica Sets are used to tell how many replicas of a Pod should be running. They are managed by the deployments.

### Declacritive configuration with Deployment.yaml

**anti-patterns:
1. Deleting a deployment: This should be the option of last resort! Instead: Create a new Docker image and replace the **tag** of the docker image in the delpoyment.yaml with the new tag.
2. Imperative config: Always use the declarative approach with the `deployment.yaml`.


## Introduction to Networking

We have (at least) 3 layers that need network configurations. 
1. The server that is hosted in the **Docker Container**
2. The docker container.
3. The kubernetes pod.

**Docker** and **Kubernetes** (`kubectl`) can use the `port-forward` command.
Port-forwarding is used in Kubernetes to forward a local port to a pod. It is not meant for production! Use it for debugging and developement purposes.


To forward a port in Kubernetes we need either:
- a `Service` resource
- an `Ingress` resource
- a `Gateway API` (most recent solution)

### Example exposing ports
A cluster with 2 agents and a load balancer is created.
The loadbalancer is exposed on the cluster port `8081`, mapped to port `80` on the load balancer.
Agent 0 is exposed on port `8082` and mapped to port `30080` on the agent
` k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2`


### Service
Because kubernetes pods are **ephemeral** (can be created and terminated at any moment), they cannot be used for communication with the application.

**Service resources** have the task of managing the application's accessibility, ensuring that it can be reached by connections originating both,
inside and outside the cluster

Example `service.yaml`:
```yaml
apiVersion: v1
kind: Service
metadata:
  name: hashresponse
spec:
  type: NodePort
  selector:
    app: hashresponse
  ports:
    - name: http
      nodePort: 30080
      protocol: TCP
      port: 1234 # Port in the cluster
      targetPort: 3000
```

nodeports of type `NodePort` not used in production as they are not configurable and open to **all of the nodes**.
Use a nodePort of type `LoadBalancer` instead.


### Ingress
Ingress is a native kubernetes resource that allows for the mapping of external DNS traffic to an internal Kubernetes service endpoint.
Ingress: Traffic into the cluster.
Egress: Traffic out of the cluster.
It is an **Incoming Network Access resource**.
Different type of resource from service.
An Ingress resource only defines the rules for routing (HTTP/HTTPS) traffic
