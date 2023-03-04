resource "honeycombio_dataset" "dataset" {
  name        = var.name
  description = "Bender's infrastructure has been updated to the latest version."
}

resource "honeycombio_marker" "deployment" {
  message = "Applying updated Terraform configuration"
  type    = "deploy"
  dataset = honeycombio_dataset.dataset.name
}
