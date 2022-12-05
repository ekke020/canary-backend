#!/bin/bash

yq -i '.spec.template.spec.containers.[0].image |= "ekke020/canary-backend"' ./kubernetes/canary-backend-deployment.yaml

kubectl kustomize ./kubernetes/ > kubernetes/deployment.yaml