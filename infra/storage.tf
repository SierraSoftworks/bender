resource "azurerm_storage_account" "function" {
  name                            = "sierrasft${var.name}fa"
  resource_group_name             = azurerm_resource_group.function.name
  location                        = azurerm_resource_group.function.location
  account_tier                    = "Standard"
  account_replication_type        = "ZRS"
  allow_nested_items_to_be_public = false
}

# RBAC assignments for production function app to access storage account
resource "azurerm_role_assignment" "production_function_storage_blob_data_owner" {
  scope                = azurerm_storage_account.function.id
  role_definition_name = "Storage Blob Data Owner"
  principal_id         = azurerm_linux_function_app.production.identity[0].principal_id
}

resource "azurerm_role_assignment" "production_function_storage_account_contributor" {
  scope                = azurerm_storage_account.function.id
  role_definition_name = "Storage Account Contributor"
  principal_id         = azurerm_linux_function_app.production.identity[0].principal_id
}

# RBAC assignments for staging function app to access storage account
resource "azurerm_role_assignment" "staging_function_storage_blob_data_owner" {
  scope                = azurerm_storage_account.function.id
  role_definition_name = "Storage Blob Data Owner"
  principal_id         = azurerm_linux_function_app.staging.identity[0].principal_id
}

resource "azurerm_role_assignment" "staging_function_storage_account_contributor" {
  scope                = azurerm_storage_account.function.id
  role_definition_name = "Storage Account Contributor"
  principal_id         = azurerm_linux_function_app.staging.identity[0].principal_id
}

