resource "azurerm_resource_group" "function" {
  name     = "app-${var.name}"
  location = "${var.location}"
}
