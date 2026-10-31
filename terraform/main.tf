terraform {
  required_version = ">= 1.5.0"

  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
    local = {
      source  = "hashicorp/local"
      version = "~> 2.5"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

# Required APIs & Project info
resource "google_project_service" "apis" {
  for_each = toset([
    "run.googleapis.com",
    "cloudbuild.googleapis.com",
    "compute.googleapis.com",
    "storage.googleapis.com",
    "iam.googleapis.com",
    "artifactregistry.googleapis.com",
  ])
  project            = var.project_id
  service            = each.key
  disable_on_destroy = false
}

data "google_project" "current" {
  project_id = var.project_id
  depends_on = [google_project_service.apis]
}

# 1. GCS bucket for /srv/var/ stateful sync
resource "google_storage_bucket" "stateful_sync" {
  name                        = var.gcs_bucket != "" ? var.gcs_bucket : "${var.project_id}-${var.app_name}-var-state"
  location                    = var.region
  force_destroy               = false
  uniform_bucket_level_access = true

  versioning {
    enabled = false
  }

  depends_on = [google_project_service.apis]
}

# 2. Artifact Registry repository
resource "google_artifact_registry_repository" "repo" {
  location      = var.region
  repository_id = var.app_name
  format        = "DOCKER"
  depends_on    = [google_project_service.apis]
}

# 3. Service account & permissions
resource "google_service_account" "cloudrun_sa" {
  account_id   = "${var.app_name}-sa"
  display_name = "Service Account for ${var.app_name} Cloud Run"
  depends_on   = [google_project_service.apis]
}

resource "google_storage_bucket_iam_member" "gcs_access" {
  bucket = google_storage_bucket.stateful_sync.name
  role   = "roles/storage.objectAdmin"
  member = "serviceAccount:${google_service_account.cloudrun_sa.email}"
}

# 4. Cloud Run service & public access
resource "google_cloud_run_v2_service" "app" {
  name     = var.app_name
  location = var.region
  ingress  = "INGRESS_TRAFFIC_ALL"

  template {
    timeout               = "${var.timeout_seconds}s"
    service_account       = google_service_account.cloudrun_sa.email
    execution_environment = "EXECUTION_ENVIRONMENT_GEN2"

    scaling {
      max_instance_count = 1
    }

    containers {
      image = "us-docker.pkg.dev/cloudrun/container/hello"

      resources {
        limits = {
          memory = var.memory_limit
          cpu    = var.cpu_limit
        }
      }

      env {
        name  = "GCS_BUCKET"
        value = google_storage_bucket.stateful_sync.name
      }
    }
  }

  lifecycle {
    ignore_changes = [template[0].containers[0].image, client, client_version]
  }
}

resource "google_cloud_run_v2_service_iam_member" "public_access" {
  project  = google_cloud_run_v2_service.app.project
  location = google_cloud_run_v2_service.app.location
  name     = google_cloud_run_v2_service.app.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}

# 5. Cloud Build deployment permissions
resource "google_project_iam_member" "cloudbuild_run_developer" {
  for_each = {
    for p in setproduct(
      ["@cloudbuild.gserviceaccount.com", "-compute@developer.gserviceaccount.com"],
      ["roles/run.developer", "roles/logging.logWriter", "roles/storage.objectAdmin", "roles/artifactregistry.admin"]
    ) : "${p[0]}/${p[1]}" => p
  }
  project = var.project_id
  member  = "serviceAccount:${data.google_project.current.number}${each.value[0]}"
  role    = each.value[1]
}

resource "google_service_account_iam_member" "cloudbuild_act_as" {
  for_each           = toset(["@cloudbuild.gserviceaccount.com", "-compute@developer.gserviceaccount.com"])
  service_account_id = google_service_account.cloudrun_sa.name
  role               = "roles/iam.serviceAccountUser"
  member             = "serviceAccount:${data.google_project.current.number}${each.value}"
}

# 6. Dynamic cloudbuild.yaml generation
resource "local_file" "cloudbuild" {
  content = templatefile("${path.module}/cloudbuild.yaml.tftpl", {
    project_id = var.project_id
    app_name   = var.app_name
    image_tag  = var.image_tag
    region     = var.region
  })
  filename = "${path.module}/../cloudbuild.yaml"
}

# 7. Automatic build & deployment via Cloud Build
resource "terraform_data" "auto_deploy" {
  triggers_replace = [
    timestamp(),
  ]

  provisioner "local-exec" {
    working_dir = "${path.module}/.."
    command = join(" && ", ["./srv/compile.sh --release --no-run",
      "gcloud builds submit --config=cloudbuild.yaml --ignore-file=terraform/.gcloudignore",
    "./srv/clear.sh"])
  }

  depends_on = [
    google_cloud_run_v2_service.app,
    google_cloud_run_v2_service_iam_member.public_access,
    google_project_iam_member.cloudbuild_run_developer,
    google_service_account_iam_member.cloudbuild_act_as,
    google_artifact_registry_repository.repo,
    local_file.cloudbuild,
  ]
}


