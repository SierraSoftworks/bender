// We retrieve information on the client deploying this plan
// to determine tenant information.
data "azuread_client_config" "current" {}

variable "location" {
  description = "The Azure location where this app will be deployed."
  default     = "North Europe"
}

variable "domain" {
  description = "The domain root at which the app will be accessible."
  default     = "sierrasoftworks.com"
}

variable "name" {
  description = "The name of the application being deployed."
  default     = "bender"
}

variable "repository" {
  description = "The name of the GitHub repository that contains the application."
  default     = "SierraSoftworks/bender"
}

variable "production-environment" {
  description = "The name of the production environment."
  default     = "Production"
}

variable "staging-environment" {
  description = "The name of the staging environment."
  default     = "Staging"
}

variable "app-settings" {
  description = "The application settings that should be configured for the function app."
  type        = map(string)
  default = {
    "FUNCTIONS_WORKER_RUNTIME" = "custom"
  }
}

variable "honeycomb-key-staging" {
  description = "The Honeycomb API key that should be used for staging."
}

variable "honeycomb-key-production" {
  description = "The Honeycomb API key that should be used for production."
}

variable "allowed_origins" {
  description = "The origins that should be allowed to access the API."
  type        = set(string)
  default     = ["*"]
}
