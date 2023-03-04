resource "azurerm_storage_account" "function" {
  name                            = "sierrasft${var.name}fa"
  resource_group_name             = azurerm_resource_group.function.name
  location                        = azurerm_resource_group.function.location
  account_tier                    = "Standard"
  account_replication_type        = "ZRS"
  allow_nested_items_to_be_public = false
}
