{{/* vim: set filetype=mustache: */}}
{{/*
Expand the name of the chart.
*/}}
{{- define "diem-validator.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "diem-validator.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.nameOverride -}}
{{- if contains $name .Release.Name -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "diem-validator.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{/*
Common labels
*/}}
{{- define "diem-validator.labels" -}}
{{- range $k, $v := .Values.labels }}
{{ $k }}: {{ $v }}
{{- end }}
helm.sh/chart: {{ include "diem-validator.chart" . }}
{{ include "diem-validator.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{/*
Multicluster labels. `multiclusterLabels` takes in a tuple of context and index as arguments.
It should be invoked as `diem-validator.multiclusterLabels (tuple $ $i)` where $i is the index
of the statefulset.

The logic below assigns a target cluster to each statefulset replica in a round-robin fashion.
*/}}
{{- define "diem-validator.multiclusterLabels" -}}
{{- $ctx := index $ 0 -}}
{{- if $ctx.Values.multicluster.enabled }}
{{- $index := index $ 1 -}}
{{- $numClusters := len $ctx.Values.multicluster.targetClusters }}
{{- $clusterIndex := mod $index $numClusters }}
{{- $cluster := index $ctx.Values.multicluster.targetClusters $clusterIndex }}
multicluster/targetcluster: {{ $cluster }}
{{- end }}
{{- end -}}

{{/*
Selector labels
*/}}
{{- define "diem-validator.selectorLabels" -}}
{{- range $k, $v := .Values.labels }}
{{ $k }}: {{ $v }}
{{- end }}
app.kubernetes.io/part-of: {{ include "diem-validator.name" . }}
app.kubernetes.io/managed-by: helm
{{- end -}}

{{/*
Create the name of the service account to use
*/}}
{{- define "diem-validator.serviceAccountName" -}}
{{- if .Values.serviceAccount.create -}}
    {{ default (include "diem-validator.fullname" .) .Values.serviceAccount.name }}
{{- else -}}
    {{ default "default" .Values.serviceAccount.name }}
{{- end -}}
{{- end -}}