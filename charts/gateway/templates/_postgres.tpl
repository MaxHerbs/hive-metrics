{{- define "gateway.password" }}
{{- $existing := (lookup "v1" "Secret" .Release.Namespace "postgres-config") }}
{{- if $existing }}
  {{- index $existing.data "password" | b64dec }}
{{- else }}
  {{- if not (index .Values "newPassword") }}
    {{- $_ := set .Values "newPassword" (randAlphaNum 24) }}
  {{- end }}
  {{- index .Values "newPassword" }}
{{- end }}
{{- end }}

{{- define "gateway.postgresPassword" }}
{{- $existing := (lookup "v1" "Secret" .Release.Namespace "postgres-config") }}
{{- if $existing }}
  {{- index $existing.data "postgres-password" | b64dec }}
{{- else }}
  {{- if not (index .Values "newPostgresPassword") }}
    {{- $_ := set .Values "newPostgresPassword" (randAlphaNum 24) }}
  {{- end }}
  {{- index .Values "newPostgresPassword" }}
{{- end }}
{{- end }}

