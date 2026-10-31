output "service_url" {
  description = "Deployed Cloud Run service URL"
  value       = google_cloud_run_v2_service.app.uri
}

output "service_name" {
  description = "Cloud Run service name"
  value       = google_cloud_run_v2_service.app.name
}

output "location" {
  description = "GCP region where resources are deployed"
  value       = google_cloud_run_v2_service.app.location
}

output "gcs_state_bucket" {
  description = "GCS bucket name for /srv/var/ stateful synchronization"
  value       = google_storage_bucket.stateful_sync.name
}
