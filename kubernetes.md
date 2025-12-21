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


## Docker
### Create folder in docker container:
`docker exec docker-container-name mkdir -p folder-path`
# Kubernetes Basics

## First Deploy

### Microservices
`Microservices are small, autonomous services that work together`
Opposite: Monolith.

Microservice applications can compose of serveral, even dozens independently operating services.

### What is Kubernetes?
`Kubernetes is an open-source system for automatic deployment, scaling and management of containerized applications.`
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
2. Imperative config: Instead always use the declarative approach with the `deployment.yaml`.


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


## Introduction to storage
Storing data in Kubernetes is challenging because containers are ephemeral.
Kubernetes communicates with storage with **control plane interfaces**.
External storage solutions that can be linked to Kubernetes are called **Volumes**. 

### emptyDir volumes
shared filesystems **inside** a pod -> tied to the pods lifecycle.
It can be used as a chache or share files between containers in a pod.
location on node: `/var/lib/kubelet/pods/{podid}/volumes/kubernetes.io~empty-dir/`

### Persistent Volume
Cluster-wide resource that represents a piece of storage in the cluster.
Lifecycle independent -> outlives the pod that it is attached to.
**Local Persistent Volumes should not be used in production!**
Example:
```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: example-pv
spec:
  storageClassName: my-example-pv # this is the name you are using later to claim this volume
  capacity:
    storage: 1Gi # Could be e.q. 500Gi. Small amount is to preserve space when testing locally
  volumeMode: Filesystem # This declares that it will be mounted into pods as a directory
  accessModes:
  - ReadWriteOnce
  local:
    path: /tmp/kube
  nodeAffinity: ## This is only required for local, it defines which nodes can access it
    required:
      nodeSelectorTerms:
      - matchExpressions:
        - key: kubernetes.io/hostname
          operator: In
          values:
          - k3d-k3s-default-agent-0
```

### Persistent Volume Claim 
When a PVC is created Kubernetes finds an appropiate PV that satisfies the claim's requirement and binds them together.


Example:
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: image-claim # name of the volume claim, this will be used in the deployment
spec:
  storageClassName: my-example-pv # this is the name of the persistent volume we are claiming
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```


The design choice of PV and PVC has several advantages:
- **Resuability and Flexibility:**
    - PVs can outlive Pods
    - If a Pod is deleted the PVC can reattach to a new Pod
    - this allows stateful workloads to persist across Pod restarts
- **Role Decoupling:**
    - Cluster admins manage and configure available storage backends
    - Developers just request storage without needing to know the details.
- **Portability:**
    - Devs can write manifests with PVCs and not worry about the underlying storage.
    - This makes applications portable accross enviroments
- **Dynamic Provisioning:**
    - With `StorageClass` PVCs can automatically trigger the creation of new PVs on demand
- **Lifecycle management:**
    - PVs have `reclaim policies`: `Retain`, `Recycle`, `Delete`
    - This defines what happens when a PVC is deleted -> keep the data, wipe and reuse or delete the underlying storage.

### Remarks to Exercies 1.11
Make sure to put the agent that the local path is created on in deployment -> spec -> nodeSelector -> kubernetes.io/hostname 
and in PersistentVolume -> spec -> nodeAffinity -> required -> matchExpressions -> values


## Chapter 3: More building blocks
### Networking between pods


### Namespace
Kubernetes has the concept of namespaces. They can be understood as seperate clusters inside a Kubernetes cluster.
They help with origination, security and performance.
Newly created clusters have 3 namespaces out of the box:
1. `default`:  Cannot be deleted and pointed at by most tools
2. `kube-system`: Should be left alone
3. `kube-public`: is not really used right now

Best practice: do not touch existing namespaces. Instead create new ones for seperate tasks.
Create a new namespace `test` with the command: `kubectl create namespace test`.
Alternative: create a `.yaml` file and apply it like other kubernetes resources.
```yaml
kind: Namespace
apiVersion: v1
metadata: 
    name: test
    labels:
        name: test
```
Apply with `kubectl appy -f test.yaml`
See all namespaces with: `kubectl get namespaces`
If no namespace is defined in resource, it will be attached to the current namespace.
Specify namespace by:
- adding flag to create command: `kubectl apply -f pod.yaml --namespace=test `
- specify namespace in yaml declaration

use `kubens` to switch active namespace.

### Cross Namespace communication
Kubernetes uses the expanded form of the DNS address to communicate between namespaces:
`<Service Name>.<Namespace Name>.svc.cluster.local<F7>` 
The `Namespace Name` component can be omitted for services in the same namespace. 

### Labels
Used to further categorize resources in a namespace and to group them.
Labels are also used by selectors to pick a set of objects.
Example: add the label `importance=great` to a pod with:
`kubectl label po <pod-name> importance=great`
You can now filter by label: `kubectl get pod -l importance`

### Configuring applications
Kubernetes has 2 resources for configuration management:
### Secrets
for sensitive information that are given to containers at runtime
### ConfigMaps
For non sensitive information. Changing the ConfigMap will instantly change the behavior of the application.
