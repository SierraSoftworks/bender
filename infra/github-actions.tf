data "azurerm_subscription" "current" {
}

resource "azuread_application" "deploy-production" {
  display_name     = "deploy.${var.name}.production"
  owners           = [data.azuread_client_config.current.object_id]
  sign_in_audience = "AzureADMyOrg"
}

resource "azuread_service_principal" "deploy-production" {
  application_id               = azuread_application.deploy-production.application_id
  app_role_assignment_required = false
  owners = [
    data.azuread_client_config.current.object_id,
  ]
}

resource "azuread_application" "deploy-staging" {
  display_name     = "deploy.${var.name}.staging"
  owners           = [data.azuread_client_config.current.object_id]
  sign_in_audience = "AzureADMyOrg"
}

resource "azuread_service_principal" "deploy-staging" {
  application_id               = azuread_application.deploy-staging.application_id
  app_role_assignment_required = false
  owners = [
    data.azuread_client_config.current.object_id,
  ]
}

resource "azuread_application_federated_identity_credential" "production" {
  application_object_id = azuread_application.deploy-production.object_id
  display_name          = "Environment"
  description           = "Allows deployments from GitHub Actions to the 'Production' environment."
  audiences             = ["api://AzureADTokenExchange"]
  issuer                = "https://token.actions.githubusercontent.com"
  subject               = "repo:${var.repository}:environment:${var.production-environment}"
}

resource "azuread_application_federated_identity_credential" "staging" {
  application_object_id = azuread_application.deploy-staging.object_id
  display_name          = "Environment"
  description           = "Allows deployments from GitHub Actions to the 'Staging' environment."
  audiences             = ["api://AzureADTokenExchange"]
  issuer                = "https://token.actions.githubusercontent.com"
  subject               = "repo:${var.repository}:environment:${var.staging-environment}"
}

resource "azuread_application_federated_identity_credential" "prs" {
  application_object_id = azuread_application.deploy-staging.object_id
  display_name          = "PRs"
  description           = "Allows deployments from GitHub Actions for pull requests."
  audiences             = ["api://AzureADTokenExchange"]
  issuer                = "https://token.actions.githubusercontent.com"
  subject               = "repo:${var.repository}:pull_request"
}

resource "azurerm_role_assignment" "deploy-production" {
  scope                = azurerm_linux_function_app.production.id
  principal_id         = azuread_service_principal.deploy-production.object_id
  role_definition_name = "Contributor"
}

resource "azurerm_role_assignment" "deploy-staging" {
  scope                = azurerm_linux_function_app.staging.id
  principal_id         = azuread_service_principal.deploy-staging.object_id
  role_definition_name = "Contributor"
}

output "deploy-production" {
  value = {
    tenant_id       = data.azuread_client_config.current.tenant_id
    subscription_id = data.azurerm_subscription.current.subscription_id
    client_id       = azuread_application.deploy-production.application_id
  }
}

output "deploy-staging" {
  value = {
    tenant_id       = data.azuread_client_config.current.tenant_id
    subscription_id = data.azurerm_subscription.current.subscription_id
    client_id       = azuread_application.deploy-staging.application_id
  }
}
