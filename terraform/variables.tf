variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region for Cloud Run & GCS"
  type        = string
  default     = "us-central1"
}

variable "app_name" {
  description = "Application and service name"
  type        = string
}

variable "image_tag" {
  description = "Docker image tag"
  type        = string
  default     = "latest"
}

variable "memory_limit" {
  description = "Memory limit for Cloud Run container"
  type        = string
  default     = "2Gi"
}

variable "cpu_limit" {
  description = "CPU limit for Cloud Run container"
  type        = string
  default     = "1"
}

variable "timeout_seconds" {
  description = "Request timeout seconds"
  type        = number
  default     = 300
}

variable "gcs_bucket" {
  description = "Bucket name for /srv/var stateful sync (leave empty for auto-generated)"
  type        = string
  default     = ""
}

