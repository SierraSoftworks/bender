terraform {
  required_version = ">= 1.1.0"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.46.0"
    }

    azuread = {
      source  = "hashicorp/azuread"
      version = "~> 2.36.0"
    }

    honeycombio = {
      source  = "honeycombio/honeycombio"
      version = "~> 0.13.0"
    }
  }
}

terraform {
  cloud {
    organization = "sierrasoftworks"

    workspaces {
      name = "bender"
    }
  }
}

provider "azurerm" {
  features {}

  // NOTE: You can retrieve this secret using `op read op://epfkgzb2bz4msye2xrhffiz3se/jrlwg64m56hkbkbfvgljfkwcfy/Azure/client_secret`
  subscription_id = var.azure_subscription
  tenant_id       = var.azure_tenant
  client_id       = var.azure_client_id
  client_secret   = var.azure_client_secret
}

provider "azuread" {
  // NOTE: You can retrieve this secret using `op read op://epfkgzb2bz4msye2xrhffiz3se/jrlwg64m56hkbkbfvgljfkwcfy/Azure/client_secret`
  tenant_id     = var.azure_tenant
  client_id     = var.azure_client_id
  client_secret = var.azure_client_secret
}

provider "honeycombio" {
  # You can set the API key with the environment variable HONEYCOMB_API_KEY
}