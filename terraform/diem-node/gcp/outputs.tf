output "helm_release_name" {
  value = helm_release.validator.name
}

output "gke_cluster_endpoint" {
  value = google_container_cluster.diem.endpoint
}

output "gke_cluster_ca_certificate" {
  value = google_container_cluster.diem.master_auth[0].cluster_ca_certificate
}

output "gke_cluster_workload_identity_config" {
  value = google_container_cluster.diem.workload_identity_config
}
