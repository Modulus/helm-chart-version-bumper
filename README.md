# WTF
Just me playing around with simple rust

# Main purpose
Bump version number in Chart.yaml for a helm chart
And ArgoCD kind: Application

# TO install
cargo install --path .

helm-chart-version-bumper

alias bump="helm-chart-version-bumper"

NB! There must either be a Chart.yaml, or an Argo CD application yaml file in your current directory when running this

# To run
Stand in directory which has Chart.yaml
```
bump 

//or

helm-chart-version-bumper
```

## To check
```
git diff
```
### Helm chart
This tool supports bumping the "version" property of this Chart.yaml for helm
```
apiVersion: v2
name: some-deploy-rules
description: A Stoopid Helm chart for Kubernetes Something using images
icon: https://www.dictionary.com/e/wp-content/uploads/2018/03/thisisfine-1.jpg
keywords:
- thisisfine
- development
- 
# A chart can be either an 'application' or a 'library' chart.
#
# Application charts are a collection of templates that can be packaged into versioned archives
# to be deployed.
#
# Library charts provide useful utilities or functions for the chart developer. They're included as
# a dependency of application charts to inject those utilities and functions into the rendering
# pipeline. Library charts do not define any templates and therefore cannot be deployed.
type: application

# This is the chart version. This version number should be incremented each time you make changes
# to the chart and its templates, including the app version.
# Versions are expected to follow Semantic Versioning (https://semver.org/)
<span style="color:blue">version: 0.2.0</span>

# This is the version number of the application being deployed. This version number should be
# incremented each time you make changes to the application. Versions are not expected to
# follow Semantic Versioning. They should reflect the version the application is using.
# It is recommended to use it with quotes.
appVersion: "1.16.0" 
dependencies:
- name: common
  repository: oci://registry-1.docker.io/bitnamicharts
  tags:
  - bitnami-common
  version: 2.x.x
```

### ArgoCD

This tool supports bumping the targetRevision property here
```
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: demo-app
  namespace: some-namespace
  finalizers:
    - resources-finalizer.argocd.argoproj.io
  labels:
    odm.hmm.com/instance: demo-app
  annotations:
    gitops-trace.hmm.com/build-reason: IndividualCI
spec:
  destination:
    namespace: some-namespace
    server: https://kubernetes.default.svc
  project: some-project
  source:
    chart: here
    helm:
      valuesObject: 
        container:
          image:
            repository: ubuntu
            version: 24.04
    targetRevision: 0.3.3
  syncPolicy:
    automated:
      prune: true
```